use super::geometry::*;
use super::objects::SHIP;
use crate::graphics::VGA;
use crate::io::{get_key_ev, KeyCode, KeyEvent, KeyState, MOUSE, SCANCODE_QUEUE};
use crate::timer::sleep;
use alloc::vec::Vec;
use core::f32::consts::PI;
use libm::tanf;

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 200.0;

pub async fn render() {
    let mesh = Mesh::from_obj_file(SHIP);
    let proj_matrix = {
        let near: f32 = 0.5;
        let far: f32 = 1000.0;
        let fov: f32 = 90.0;
        let aspect_ratio: f32 = HEIGHT / WIDTH;
        let fov_rad = 1.0 / tanf(fov * 0.5 / 180.0 * PI);
        Matrix4x4 {
            m: [
                [aspect_ratio * fov_rad, 0.0, 0.0, 0.0],
                [0.0, fov_rad, 0.0, 0.0],
                [0.0, 0.0, far / (far - near), 1.0],
                [0.0, 0.0, (-far * near) / (far - near), 0.0],
            ],
        }
    };

    let mut camera_vector = Vector::new();
    let mut look_direction = Vector {
        x: 0.0,
        y: 0.0,
        z: 1.0,
        w: 1.0,
    };
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;

    let scancode_queue = SCANCODE_QUEUE.try_get().unwrap();

    //Don't want to register input on every press - this only works if you have a key repeat rate
    // Instead increment every frame based on the following variables
    let mut w_pressed = false;
    let mut a_pressed = false;
    let mut s_pressed = false;
    let mut d_pressed = false;

    let mut iterations: f32 = 0.0;
    loop {
        // Get user input
        while let Ok(code) = scancode_queue.pop() {
            if let Ok(key_event) = get_key_ev(code) {
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
                    _ => {}
                }
            }
        }

        if w_pressed {
            camera_vector = Vector::add(&camera_vector, &look_direction);
        }
        if s_pressed {
            camera_vector = Vector::sub(&camera_vector, &look_direction);
        }
        let opp_look_direction = Vector {
            x: look_direction.z,
            y: 0.0,
            z: -look_direction.x,
            w: 1.0,
        };
        if a_pressed {
            camera_vector = Vector::add(&camera_vector, &opp_look_direction);
        }
        if d_pressed {
            camera_vector = Vector::sub(&camera_vector, &opp_look_direction);
        }

        let (delta_x, delta_y) = MOUSE.lock().get_coords();
        yaw += delta_x as f32 * 0.02;
        pitch += delta_y as f32 * 0.02;

        //Compute world matrix (rotation and translation)
        // let theta: f32 = 1.0 * iterations;
        let theta: f32 = 0.0;
        let rotate_x_mat = Matrix4x4::create_rot_x(theta);
        let rotate_z_mat = Matrix4x4::create_rot_z(theta * 0.5);
        let trans_mat = Matrix4x4::create_translation(0.0, 0.0, 10.0);

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
        let camera_rot_mat_y = Matrix4x4::create_rot_y(yaw);
        let camera_rot_mat_x = Matrix4x4::create_rot_x(pitch);
        let camera_rot_mat = Matrix4x4::mult(&camera_rot_mat_y, &camera_rot_mat_x);
        look_direction = Matrix4x4::mult_vec(&camera_rot_mat, &target);
        target = Vector::add(&camera_vector, &look_direction);

        let camera_mat = Matrix4x4::point_at(&camera_vector, &target, &up);
        let view_mat = Matrix4x4::quick_inverse(&camera_mat);

        //Compute triangles to render
        let mut triangles_to_raster: Vec<Triangle> = Vec::new();
        for tri in mesh.tris.iter() {
            let mut new_tri = Triangle::new();
            // Rotatation and Translation
            new_tri.p[0] = Matrix4x4::mult_vec(&world_mat, &tri.p[0]);
            new_tri.p[1] = Matrix4x4::mult_vec(&world_mat, &tri.p[1]);
            new_tri.p[2] = Matrix4x4::mult_vec(&world_mat, &tri.p[2]);

            //Calculate two lines of triangle
            let l1 = Vector::sub(&new_tri.p[1], &new_tri.p[0]);
            let l2 = Vector::sub(&new_tri.p[2], &new_tri.p[0]);

            //Use cross product to get normal
            let mut normal = Vector::cross(&l1, &l2);

            //Normalize normal
            normal = Vector::norm(&normal);

            //Get ray from camera to triangle
            let camera_ray = Vector::sub(&new_tri.p[0], &camera_vector);

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
                // VGA default pallette sets 15 as white - 15-31 are grayscale colors
                let brightness = (dp * 16 as f32) as u8;
                new_tri.color = brightness + 15;

                //Convert world space --> view space
                new_tri.p[0] = Matrix4x4::mult_vec(&view_mat, &new_tri.p[0]);
                new_tri.p[1] = Matrix4x4::mult_vec(&view_mat, &new_tri.p[1]);
                new_tri.p[2] = Matrix4x4::mult_vec(&view_mat, &new_tri.p[2]);

                //Clip triangles against near plane
                let front_cam = Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 2.0,
                    w: 1.0,
                };
                let normal = Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                    w: 1.0,
                };

                let clipped: [Option<Triangle>; 2] = Vector::clip_plane(front_cam, normal, new_tri);

                for clipped_tri in clipped {
                    if let Some(mut clipped_tri) = clipped_tri {
                        // Project 3D --> 2D
                        clipped_tri.p[0] = Matrix4x4::mult_vec(&proj_matrix, &clipped_tri.p[0]);
                        clipped_tri.p[1] = Matrix4x4::mult_vec(&proj_matrix, &clipped_tri.p[1]);
                        clipped_tri.p[2] = Matrix4x4::mult_vec(&proj_matrix, &clipped_tri.p[2]);

                        // normalising is in cartesian space so we need to divide by w
                        clipped_tri.p[0] =
                            Vector::div_scaler(&clipped_tri.p[0], &clipped_tri.p[0].w);
                        clipped_tri.p[1] =
                            Vector::div_scaler(&clipped_tri.p[1], &clipped_tri.p[1].w);
                        clipped_tri.p[2] =
                            Vector::div_scaler(&clipped_tri.p[2], &clipped_tri.p[2].w);

                        // X/Y are inverted so put them back
                        clipped_tri.p[0].x *= -1.0;
                        clipped_tri.p[1].x *= -1.0;
                        clipped_tri.p[2].x *= -1.0;
                        clipped_tri.p[0].y *= -1.0;
                        clipped_tri.p[1].y *= -1.0;
                        clipped_tri.p[2].y *= -1.0;

                        // Scale into view
                        clipped_tri.p[0].x += (clipped_tri.p[0].x + 1.0) * 0.5 * WIDTH as f32;
                        clipped_tri.p[0].y += (clipped_tri.p[0].y + 1.0) * 0.5 * HEIGHT as f32;
                        clipped_tri.p[1].x += (clipped_tri.p[1].x + 1.0) * 0.5 * WIDTH as f32;
                        clipped_tri.p[1].y += (clipped_tri.p[1].y + 1.0) * 0.5 * HEIGHT as f32;
                        clipped_tri.p[2].x += (clipped_tri.p[2].x + 1.0) * 0.5 * WIDTH as f32;
                        clipped_tri.p[2].y += (clipped_tri.p[2].y + 1.0) * 0.5 * HEIGHT as f32;

                        triangles_to_raster.push(clipped_tri);
                    }
                }
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
            let mut triangle_queue: Vec<Triangle> = Vec::new();
            triangle_queue.push(tri);
            let mut new_tris = 0;

            for plane in 0..4 {
                let mut tris_to_draw = 0;
                while new_tris > 0 {
                    let cur_tri = triangle_queue[0];
                    new_tris -= 1;

                    let clipped_tris = match plane {
                        0 => Vector::clip_plane(
                            Vector {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                                w: 1.0,
                            },
                            Vector {
                                x: 0.0,
                                y: 1.0,
                                z: 0.0,
                                w: 1.0,
                            },
                            cur_tri,
                        ),
                        1 => Vector::clip_plane(
                            Vector {
                                x: 0.0,
                                y: HEIGHT - 1.0,
                                z: 0.0,
                                w: 1.0,
                            },
                            Vector {
                                x: 0.0,
                                y: -1.0,
                                z: 0.0,
                                w: 1.0,
                            },
                            cur_tri,
                        ),
                        2 => Vector::clip_plane(
                            Vector {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                                w: 1.0,
                            },
                            Vector {
                                x: 1.0,
                                y: 0.0,
                                z: 0.0,
                                w: 1.0,
                            },
                            cur_tri,
                        ),
                        3 => Vector::clip_plane(
                            Vector {
                                x: WIDTH - 1.0,
                                y: 0.0,
                                z: 0.0,
                                w: 1.0,
                            },
                            Vector {
                                x: -1.0,
                                y: 0.0,
                                z: 0.0,
                                w: 1.0,
                            },
                            cur_tri,
                        ),
                        _ => {
                            unreachable!();
                        }
                    };

                    for clipped_tri in clipped_tris {
                        if let Some(clipped_tri) = clipped_tri {
                            triangle_queue[tris_to_draw] = clipped_tri;
                            tris_to_draw += 1;
                        }
                    }
                }
            }

            for tri in triangle_queue {
                VGA.lock().fill_triangle(
                    (tri.p[0].x as isize, tri.p[0].y as isize),
                    (tri.p[1].x as isize, tri.p[1].y as isize),
                    (tri.p[2].x as isize, tri.p[2].y as isize),
                    tri.color,
                );
                // WIREFRAME FOR DEBUGGING
                // VGA.lock().draw_triangle(
                //     (tri.p[0].x as isize, tri.p[0].y as isize),
                //     (tri.p[1].x as isize, tri.p[1].y as isize),
                //     (tri.p[2].x as isize, tri.p[2].y as isize),
                //     0x0F,
                // )
            }
        }

        let crosshair = [
            0b000000111000000,
            0b000000101000000,
            0b000000101000000,
            0b000000101000000,
            0b000000111000000,
            0b000000000000000,
            0b111110000011111,
            0b100010000010001,
            0b111110000011111,
            0b000000000000000,
            0b000000111000000,
            0b000000101000000,
            0b000000101000000,
            0b000000101000000,
            0b000000111000000,
        ];

        //Draw crosshair
        VGA.lock().draw_bitmap(
            (WIDTH as usize / 2) - 6,
            HEIGHT as usize / 2,
            &crosshair,
            0xF,
        );

        // Swap buffers
        VGA.lock().swap_buffers();

        sleep(1).await;
        iterations += 0.05;
    }
}
