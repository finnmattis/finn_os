use crate::graphics::VGA;
use crate::timer::sleep;
use alloc::vec::Vec;
use core::f32::consts::PI;
use lazy_static::lazy_static;
use libm::{sqrtf, tanf};

use super::statics::{get_cube, get_rotation_matrix_x, get_rotation_matrix_z};

lazy_static! {
    pub static ref RENDERER: Renderer = Renderer::new();
}

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 200.0;

#[derive(Clone, Copy)]
pub(super) struct Vector {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) z: f32,
}

impl Vector {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

pub(super) struct Triangle {
    pub(super) p: [Vector; 3],
}

pub(super) struct Mesh {
    pub(super) tris: Vec<Triangle>,
}

pub(super) struct Matrix4x4 {
    pub(super) m: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn new() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }

    pub fn mult(&self, i: &Vector) -> Vector {
        let x = i.x * self.m[0][0] + i.y * self.m[1][0] + i.z * self.m[2][0] + self.m[3][0];
        let y = i.x * self.m[0][1] + i.y * self.m[1][1] + i.z * self.m[2][1] + self.m[3][1];
        let z = i.x * self.m[0][2] + i.y * self.m[1][2] + i.z * self.m[2][2] + self.m[3][2];
        let w = i.x * self.m[0][3] + i.y * self.m[1][3] + i.z * self.m[2][3] + self.m[3][3];
        if w != 0.0 {
            Vector {
                x: x / w,
                y: y / w,
                z: z / w,
            }
        } else {
            Vector { x, y, z }
        }
    }
}

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
            mesh_cube: get_cube(),
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

    pub async fn render(&self) {
        let mut iterations: f32 = 0.0;
        loop {
            //Compute rotation matrices
            let theta: f32 = 1.0 * iterations;
            let rotate_x = get_rotation_matrix_x(theta);
            let rotate_z = get_rotation_matrix_z(theta);

            //Compute triangles to render
            let mut triangles_to_raster: Vec<Triangle> = Vec::new();
            for tri in self.mesh_cube.tris.iter() {
                // Rotate in Z-Axis
                let mut v1 = rotate_z.mult(&tri.p[0]);
                let mut v2 = rotate_z.mult(&tri.p[1]);
                let mut v3 = rotate_z.mult(&tri.p[2]);
                // Rotate in X-Axis
                v1 = rotate_x.mult(&v1);
                v2 = rotate_x.mult(&v2);
                v3 = rotate_x.mult(&v3);
                //Translate backwards in Z
                v1 = Vector {
                    x: v1.x,
                    y: v1.y,
                    z: v1.z + 3.0,
                };
                v2 = Vector {
                    x: v2.x,
                    y: v2.y,
                    z: v2.z + 3.0,
                };
                v3 = Vector {
                    x: v3.x,
                    y: v3.y,
                    z: v3.z + 3.0,
                };
                //Get Normal
                let mut l1 = Vector::new();
                let mut l2 = Vector::new();
                let mut normal = Vector::new();

                //Calculate two lines of triangle
                l1.x = v2.x - v1.x;
                l1.y = v2.y - v1.y;
                l1.z = v2.z - v1.z;

                l2.x = v3.x - v1.x;
                l2.y = v3.y - v1.y;
                l2.z = v3.z - v1.z;

                //Use cross product to get normal
                normal.x = l1.y * l2.z - l1.z * l2.y;
                normal.y = l1.z * l2.x - l1.x * l2.z;
                normal.z = l1.x * l2.y - l1.y * l2.x;

                //Normalize
                let l = sqrtf(normal.x * normal.x + normal.y * normal.y + normal.z * normal.z);
                normal.x /= l;
                normal.y /= l;
                normal.z /= l;

                //Get dot product of normal and vector from camera to triangle
                //Note: can use v1 for each because all three points are in the same plane
                let dot_product = normal.x * (v1.x - self.camera_vector.x)
                    + normal.y * (v1.y - self.camera_vector.y)
                    + normal.z * (v1.z - self.camera_vector.z);

                //If dot product is negative, triangle is visible
                if dot_product < 0.0 {
                    // Project 3D --> 2D
                    v1 = self.proj_matrix.mult(&v1);
                    v2 = self.proj_matrix.mult(&v2);
                    v3 = self.proj_matrix.mult(&v3);
                    // Scale into view
                    v1.x += (v1.x + 1.0) * 0.5 * WIDTH as f32;
                    v1.y += (v1.y + 1.0) * 0.5 * HEIGHT as f32;
                    v2.x += (v2.x + 1.0) * 0.5 * WIDTH as f32;
                    v2.y += (v2.y + 1.0) * 0.5 * HEIGHT as f32;
                    v3.x += (v3.x + 1.0) * 0.5 * WIDTH as f32;
                    v3.y += (v3.y + 1.0) * 0.5 * HEIGHT as f32;

                    let p1 = Vector {
                        x: v1.x,
                        y: v1.y,
                        z: v1.z,
                    };

                    let p2 = Vector {
                        x: v2.x,
                        y: v2.y,
                        z: v2.z,
                    };

                    let p3 = Vector {
                        x: v3.x,
                        y: v3.y,
                        z: v3.z,
                    };
                    triangles_to_raster.push(Triangle { p: [p1, p2, p3] });
                }
            }

            // Draw triangles to double buffer
            VGA.lock().clear_screen(0x00);
            for tri in triangles_to_raster {
                VGA.lock().fill_triangle(
                    (tri.p[0].x as isize, tri.p[0].y as isize),
                    (tri.p[1].x as isize, tri.p[1].y as isize),
                    (tri.p[2].x as isize, tri.p[2].y as isize),
                    0x0F,
                );
            }

            // Swap buffers
            VGA.lock().draw_screen();

            sleep(1).await;
            iterations += 0.05;
        }
    }
}
