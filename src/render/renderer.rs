use super::geometry::*;
use super::objects::SHIP;
use crate::graphics::VGA;
use crate::timer::sleep;
use alloc::vec::Vec;
use core::f32::consts::PI;
use lazy_static::lazy_static;
use libm::tanf;

lazy_static! {
    pub static ref RENDERER: Renderer = Renderer::new();
}

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 200.0;

pub struct Renderer {
    mesh_cube: Mesh,
    proj_matrix: Matrix4x4,
    camera_vector: Vector,
}

impl Renderer {
    pub fn new() -> Self {
        let near: f32 = 0.5;
        let far: f32 = 1000.0;
        let fov: f32 = 90.0;
        let aspect_ratio: f32 = HEIGHT / WIDTH;
        let fov_rad = 1.0 / tanf(fov * 0.5 / 180.0 * PI);

        Self {
            mesh_cube: Mesh::from_obj_file(SHIP),
            proj_matrix: Matrix4x4 {
                m: [
                    [aspect_ratio * fov_rad, 0.0, 0.0, 0.0],
                    [0.0, fov_rad, 0.0, 0.0],
                    [0.0, 0.0, far / (far - near), 1.0],
                    [0.0, 0.0, (-far * near) / (far - near), 0.0],
                ],
            },
            camera_vector: Vector::new(),
        }
    }

    fn get_color(lum: f32) -> u8 {
        let brightness = (lum * 16 as f32) as u8;
        // VGA default pallette sets 15 as white - 15-31 are grayscale colors
        return brightness + 15;
    }

    pub async fn render(&self) {
        let mut iterations: f32 = 0.0;
        loop {
            //Compute rotation matrices
            let theta: f32 = 1.0 * iterations;
            let rotate_x_mat = Matrix4x4::create_rot_x(theta);
            let rotate_z_mat = Matrix4x4::create_rot_z(theta);

            let world_mat = Matrix4x4::mult(&rotate_x_mat, &rotate_z_mat);

            //Compute triangles to render
            let mut triangles_to_raster: Vec<Triangle> = Vec::new();
            for tri in self.mesh_cube.tris.iter() {
                // Rotate in Z-Axis
                let mut v1 = Matrix4x4::mult_vec(&world_mat, &tri.p[0]);
                let mut v2 = Matrix4x4::mult_vec(&world_mat, &tri.p[1]);
                let mut v3 = Matrix4x4::mult_vec(&world_mat, &tri.p[2]);
                //Translate backwards in Z
                v1 = Vector {
                    x: v1.x,
                    y: v1.y,
                    z: v1.z + 10.0,
                    w: 0.0,
                };
                v2 = Vector {
                    x: v2.x,
                    y: v2.y,
                    z: v2.z + 10.0,
                    w: 0.0,
                };
                v3 = Vector {
                    x: v3.x,
                    y: v3.y,
                    z: v3.z + 10.0,
                    w: 0.0,
                };

                //Calculate two lines of triangle
                let l1 = Vector::sub(&v2, &v1);
                let l2 = Vector::sub(&v3, &v1);

                //Use cross product to get normal
                let mut normal = Vector::cross(&l1, &l2);

                //Normalize normal
                normal = Vector::norm(&normal);

                //Get ray from camera to triangle
                let camera_ray = Vector::sub(&v1, &self.camera_vector);

                //If ray is aligned with normal, then triangle is visible
                if Vector::dot(&camera_ray, &normal) < 0.0 {
                    //Illuminate
                    let mut light_direction = Vector {
                        x: 0.0,
                        y: 0.0,
                        z: -1.0,
                        w: 1.0,
                    };
                    light_direction = Vector::norm(&light_direction);

                    //How much light is hitting the triangle
                    let dp = Vector::dot(&normal, &light_direction);
                    let color = Self::get_color(dp);

                    // Project 3D --> 2D
                    v1 = Matrix4x4::mult_vec(&self.proj_matrix, &v1);
                    v2 = Matrix4x4::mult_vec(&self.proj_matrix, &v2);
                    v3 = Matrix4x4::mult_vec(&self.proj_matrix, &v3);

                    v1 = Vector::div(&v1, &v1.w);
                    v2 = Vector::div(&v2, &v2.w);
                    v3 = Vector::div(&v3, &v3.w);

                    // Scale into view
                    v1.x += (v1.x + 1.0) * 0.5 * WIDTH as f32;
                    v1.y += (v1.y + 1.0) * 0.5 * HEIGHT as f32;
                    v2.x += (v2.x + 1.0) * 0.5 * WIDTH as f32;
                    v2.y += (v2.y + 1.0) * 0.5 * HEIGHT as f32;
                    v3.x += (v3.x + 1.0) * 0.5 * WIDTH as f32;
                    v3.y += (v3.y + 1.0) * 0.5 * HEIGHT as f32;

                    triangles_to_raster.push(Triangle {
                        p: [v1, v2, v3],
                        color,
                    });
                }
            }

            // Draw triangles to double buffer
            VGA.lock().clear_screen(0x00);
            //Sort triangles from back to front (painter's algorithm)
            triangles_to_raster.sort_by(|a, b| {
                let z1 = (a.p[0].z + a.p[1].z + a.p[2].z) / 3.0;
                let z2 = (b.p[0].z + b.p[1].z + b.p[2].z) / 3.0;
                z2.partial_cmp(&z1).unwrap()
            });

            for tri in triangles_to_raster {
                VGA.lock().fill_triangle(
                    (tri.p[0].x as isize, tri.p[0].y as isize),
                    (tri.p[1].x as isize, tri.p[1].y as isize),
                    (tri.p[2].x as isize, tri.p[2].y as isize),
                    tri.color,
                );
            }

            // Swap buffers
            VGA.lock().swap_buffers();

            sleep(1).await;
            iterations += 0.05;
        }
    }
}
