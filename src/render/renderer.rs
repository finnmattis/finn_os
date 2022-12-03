use super::geometry::*;
use super::objects::{AXIS, SHIP};
use crate::graphics::VGA;
use crate::serial_println;
use crate::task::keyboard::{get_all_scancodes, SCANCODE_QUEUE};
use crate::task::keyboard_util::{KeyCode, KeyState};
use crate::task::keyboard_util::{KeyEvent, Keyboard};
use crate::timer::sleep;
use alloc::vec::Vec;
use core::f32::consts::PI;
use crossbeam_queue::ArrayQueue;
use lazy_static::lazy_static;
use libm::tanf;

lazy_static! {
    pub static ref RENDERER: Renderer = Renderer::new();
}

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 200.0;

pub struct Renderer {
    mesh: Mesh,
    proj_matrix: Matrix4x4,
}

impl Renderer {
    pub fn new() -> Self {
        let near: f32 = 0.5;
        let far: f32 = 1000.0;
        let fov: f32 = 90.0;
        let aspect_ratio: f32 = HEIGHT / WIDTH;
        let fov_rad = 1.0 / tanf(fov * 0.5 / 180.0 * PI);

        Self {
            mesh: Mesh::from_obj_file(AXIS),
            proj_matrix: Matrix4x4 {
                m: [
                    [aspect_ratio * fov_rad, 0.0, 0.0, 0.0],
                    [0.0, fov_rad, 0.0, 0.0],
                    [0.0, 0.0, far / (far - near), 1.0],
                    [0.0, 0.0, (-far * near) / (far - near), 0.0],
                ],
            },
        }
    }

    fn get_color(lum: f32) -> u8 {
        let brightness = (lum * 16 as f32) as u8;
        // VGA default pallette sets 15 as white - 15-31 are grayscale colors
        return brightness + 15;
    }

    pub async fn render(&self) {
        let mut camera_vector = Vector::new();
        let mut look_direction = Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 1.0,
        };
        let mut yaw: f32 = 0.0;

        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeQueue already initialized");
        let queue = SCANCODE_QUEUE.try_get().unwrap();
        let keyboard = Keyboard::new();

        //Don't want to register input on every press - this only works if you have a key repeat rate
        // Instead increment every frame based on the following variables
        let mut w_pressed = false;
        let mut a_pressed = false;
        let mut s_pressed = false;
        let mut d_pressed = false;
        let mut left_pressed = false;
        let mut right_pressed = false;
        let mut up_pressed = false;
        let mut down_pressed = false;

        let mut iterations: f32 = 0.0;
        loop {
            // Get user input
            while let Ok(code) = queue.pop() {
                if let Ok(key_event) = keyboard.get_key_ev(code) {
                    match key_event {
                        KeyEvent {
                            code: KeyCode::W,
                            state: KeyState::Down,
                        } => {
                            w_pressed = true;
                        }
                        KeyEvent {
                            code: KeyCode::W,
                            state: KeyState::Up,
                        } => {
                            w_pressed = false;
                        }
                        KeyEvent {
                            code: KeyCode::A,
                            state: KeyState::Down,
                        } => {
                            a_pressed = true;
                        }
                        KeyEvent {
                            code: KeyCode::A,
                            state: KeyState::Up,
                        } => {
                            a_pressed = false;
                        }
                        KeyEvent {
                            code: KeyCode::S,
                            state: KeyState::Down,
                        } => {
                            s_pressed = true;
                        }
                        KeyEvent {
                            code: KeyCode::S,
                            state: KeyState::Up,
                        } => {
                            s_pressed = false;
                        }
                        KeyEvent {
                            code: KeyCode::D,
                            state: KeyState::Down,
                        } => {
                            d_pressed = true;
                        }
                        KeyEvent {
                            code: KeyCode::D,
                            state: KeyState::Up,
                        } => {
                            d_pressed = false;
                        }
                        KeyEvent {
                            code: KeyCode::Numpad4,
                            state: KeyState::Down,
                        } => {
                            left_pressed = true;
                        }
                        KeyEvent {
                            code: KeyCode::Numpad4,
                            state: KeyState::Up,
                        } => {
                            left_pressed = false;
                        }
                        KeyEvent {
                            code: KeyCode::Numpad6,
                            state: KeyState::Down,
                        } => {
                            right_pressed = true;
                        }
                        KeyEvent {
                            code: KeyCode::Numpad6,
                            state: KeyState::Up,
                        } => {
                            right_pressed = false;
                        }
                        KeyEvent {
                            code: KeyCode::Numpad8,
                            state: KeyState::Down,
                        } => {
                            up_pressed = true;
                        }
                        KeyEvent {
                            code: KeyCode::Numpad8,
                            state: KeyState::Up,
                        } => {
                            up_pressed = false;
                        }
                        KeyEvent {
                            code: KeyCode::Numpad2,
                            state: KeyState::Down,
                        } => {
                            down_pressed = true;
                        }
                        KeyEvent {
                            code: KeyCode::Numpad2,
                            state: KeyState::Up,
                        } => {
                            down_pressed = false;
                        }
                        _ => {}
                    }
                }
            }

            if w_pressed {
                camera_vector.y -= 1.0;
            }
            if a_pressed {
                camera_vector.x -= 1.0;
            }
            if s_pressed {
                camera_vector.y += 1.0;
            }
            if d_pressed {
                camera_vector.x += 1.0;
            }
            if up_pressed {
                camera_vector = Vector::add(&camera_vector, &look_direction);
            }
            if down_pressed {
                camera_vector = Vector::sub(&camera_vector, &look_direction);
            }
            if left_pressed {
                yaw += 0.05;
            }
            if right_pressed {
                yaw -= 0.05;
            }

            //Compute world matrix (rotation and translation)
            // let theta: f32 = 1.0 * iterations;
            let theta: f32 = 1.0;
            let rotate_x_mat = Matrix4x4::create_rot_x(theta);
            let rotate_z_mat = Matrix4x4::create_rot_z(theta);
            let trans_mat = Matrix4x4::create_translation(0.0, 0.0, 12.0);

            let mut world_mat = Matrix4x4::mult(&rotate_x_mat, &rotate_z_mat);
            world_mat = Matrix4x4::mult(&world_mat, &trans_mat);

            //Get view matrix
            let up = Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 1.0,
            };

            let mut target = Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 1.0,
            };
            let camera_rot_mat = Matrix4x4::create_rot_y(yaw);
            look_direction = Matrix4x4::mult_vec(&camera_rot_mat, &target);
            target = Vector::add(&camera_vector, &look_direction);

            let camera_mat = Matrix4x4::point_at(&camera_vector, &target, &up);
            let view_mat = Matrix4x4::quick_inverse(&camera_mat);

            //Compute triangles to render
            let mut triangles_to_raster: Vec<Triangle> = Vec::new();
            for tri in self.mesh.tris.iter() {
                // Rotate in Z-Axis
                let mut v1 = Matrix4x4::mult_vec(&world_mat, &tri.p[0]);
                let mut v2 = Matrix4x4::mult_vec(&world_mat, &tri.p[1]);
                let mut v3 = Matrix4x4::mult_vec(&world_mat, &tri.p[2]);

                //Calculate two lines of triangle
                let l1 = Vector::sub(&v2, &v1);
                let l2 = Vector::sub(&v3, &v1);

                //Use cross product to get normal
                let mut normal = Vector::cross(&l1, &l2);

                //Normalize normal
                normal = Vector::norm(&normal);

                //Get ray from camera to triangle
                let camera_ray = Vector::sub(&v1, &camera_vector);

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

                    //Convert world space --> view space
                    v1 = Matrix4x4::mult_vec(&view_mat, &v1);
                    v2 = Matrix4x4::mult_vec(&view_mat, &v2);
                    v3 = Matrix4x4::mult_vec(&view_mat, &v3);

                    // Project 3D --> 2D
                    v1 = Matrix4x4::mult_vec(&self.proj_matrix, &v1);
                    v2 = Matrix4x4::mult_vec(&self.proj_matrix, &v2);
                    v3 = Matrix4x4::mult_vec(&self.proj_matrix, &v3);

                    v1 = Vector::div_scaler(&v1, &v1.w);
                    v2 = Vector::div_scaler(&v2, &v2.w);
                    v3 = Vector::div_scaler(&v3, &v3.w);

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
