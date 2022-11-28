use crate::graphics::Color16;
use crate::graphics::VGA;
use crate::timer::sleep;
use alloc::vec::Vec;
use core::f32::consts::PI;
use lazy_static::lazy_static;
use libm::tanf;

use super::statics::{get_cube, get_rotation_matrix_x, get_rotation_matrix_z};

lazy_static! {
    pub static ref RENDERER: Renderer = Renderer::new();
}

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;

#[derive(Clone, Copy)]
pub(super) struct Vertex {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) z: f32,
}

impl Vertex {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

pub(super) struct Triangle {
    pub(super) p: [Vertex; 3],
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

    pub fn mult(&self, i: &Vertex) -> Vertex {
        let x = i.x * self.m[0][0] + i.y * self.m[1][0] + i.z * self.m[2][0] + self.m[3][0];
        let y = i.x * self.m[0][1] + i.y * self.m[1][1] + i.z * self.m[2][1] + self.m[3][1];
        let z = i.x * self.m[0][2] + i.y * self.m[1][2] + i.z * self.m[2][2] + self.m[3][2];
        let w = i.x * self.m[0][3] + i.y * self.m[1][3] + i.z * self.m[2][3] + self.m[3][3];
        if w != 0.0 {
            Vertex {
                x: x / w,
                y: y / w,
                z: z / w,
            }
        } else {
            Vertex { x, y, z }
        }
    }
}

pub struct Renderer {
    mesh_cube: Mesh,
    mat_proj: Matrix4x4,
}

impl Renderer {
    pub fn new() -> Self {
        let near: f32 = 0.5;
        let far: f32 = 1000.0;
        let fov: f32 = 90.0;
        let aspect_ratio: f32 = 0.75; // 480/640
        let fov_rad = 1.0 / tanf(fov * 0.5 / 180.0 * PI);

        Self {
            mesh_cube: get_cube(),
            mat_proj: Matrix4x4 {
                m: [
                    [aspect_ratio * fov_rad, 0.0, 0.0, 0.0],
                    [0.0, fov_rad, 0.0, 0.0],
                    [0.0, 0.0, far / (far - near), 1.0],
                    [0.0, 0.0, (-far * near) / (far - near), 0.0],
                ],
            },
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
                let mut verticies: [Vertex; 3] = [Vertex::new(); 3];
                for (i, v) in tri.p.iter().enumerate() {
                    // Rotate in Z-Axis
                    let mut res = rotate_z.mult(v);
                    // Rotate in X-Axis
                    res = rotate_x.mult(&res);
                    //Translate backwards in Z
                    res.z += 3.0;
                    // Project 3D --> 2D
                    res = self.mat_proj.mult(&res);
                    // Scale into view
                    res.x = (res.x + 1.0) * 0.5 * WIDTH as f32;
                    res.y = (res.y + 1.0) * 0.5 * HEIGHT as f32;
                    // Store vertex
                    verticies[i] = res;
                }
                triangles_to_raster.push(Triangle { p: verticies });
            }
            VGA.lock().clear_screen(Color16::Black);

            // Draw triangles
            for tri in triangles_to_raster {
                VGA.lock().draw_tri(
                    (tri.p[0].x as isize, tri.p[0].y as isize),
                    (tri.p[1].x as isize, tri.p[1].y as isize),
                    (tri.p[2].x as isize, tri.p[2].y as isize),
                    Color16::Blue,
                );
            }

            sleep(1).await;
            iterations += 0.05;
        }
    }
}
