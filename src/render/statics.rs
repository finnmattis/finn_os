use crate::render::renderer::{Matrix4x4, Mesh, Triangle, Vector};
use alloc::vec;
use libm::{cosf, sinf};

pub(super) fn get_cube() -> Mesh {
    Mesh {
        tris: vec![
            //SOUTH CLOCKWISE
            Triangle {
                p: [
                    Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    Vector {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                ],
            },
            Triangle {
                p: [
                    Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vector {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ],
            },
            //EAST CLOCKWISE
            Triangle {
                p: [
                    Vector {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                ],
            },
            Triangle {
                p: [
                    Vector {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                ],
            },
            //NORTH CLOCKWISE
            Triangle {
                p: [
                    Vector {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                ],
            },
            Triangle {
                p: [
                    Vector {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                ],
            },
            //WEST CLOCKWISE
            Triangle {
                p: [
                    Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                ],
            },
            Triangle {
                p: [
                    Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ],
            },
            //TOP CLOCKWISE
            Triangle {
                p: [
                    Vector {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vector {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                ],
            },
            Triangle {
                p: [
                    Vector {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                ],
            },
            //BOTTOM CLOCKWISE
            Triangle {
                p: [
                    Vector {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ],
            },
            Triangle {
                p: [
                    Vector {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    Vector {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ],
            },
        ],
    }
}

pub(super) fn get_rotation_matrix_x(theta: f32) -> Matrix4x4 {
    let mut rotate_x = Matrix4x4::new();
    rotate_x.m[0][0] = 1.0;
    rotate_x.m[1][1] = cosf(theta * 0.5);
    rotate_x.m[1][2] = sinf(theta * 0.5);
    rotate_x.m[2][1] = -sinf(theta * 0.5);
    rotate_x.m[2][2] = cosf(theta * 0.5);
    rotate_x.m[3][3] = 1.0;
    rotate_x
}

pub(super) fn get_rotation_matrix_z(theta: f32) -> Matrix4x4 {
    let mut rotate_z = Matrix4x4::new();
    rotate_z.m[0][0] = cosf(theta);
    rotate_z.m[0][1] = sinf(theta);
    rotate_z.m[1][0] = -sinf(theta);
    rotate_z.m[1][1] = cosf(theta);
    rotate_z.m[2][2] = 1.0;
    rotate_z.m[3][3] = 1.0;
    rotate_z
}
