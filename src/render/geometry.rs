#![allow(dead_code)]
use alloc::vec;
use alloc::{string::String, vec::Vec};
use libm::{cosf, sinf, sqrtf};
#[derive(Debug, Clone, Copy)]
pub(super) struct Vector {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) z: f32,
    pub(super) w: f32,
}

impl Vector {
    pub(super) fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub(super) fn add(v1: &Vector, v2: &Vector) -> Vector {
        Vector {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
            z: v1.z + v2.z,
            w: 1.0,
        }
    }

    pub(super) fn add_scaler(v1: &Vector, scaler: &f32) -> Vector {
        Vector {
            x: v1.x + scaler,
            y: v1.y + scaler,
            z: v1.z + scaler,
            w: 1.0,
        }
    }

    pub(super) fn sub(v1: &Vector, v2: &Vector) -> Vector {
        Vector {
            x: v1.x - v2.x,
            y: v1.y - v2.y,
            z: v1.z - v2.z,
            w: 1.0,
        }
    }

    pub(super) fn sub_scaler(v1: &Vector, scaler: &f32) -> Vector {
        Vector {
            x: v1.x - scaler,
            y: v1.y - scaler,
            z: v1.z - scaler,
            w: 1.0,
        }
    }

    pub(super) fn mult_scaler(v1: &Vector, scaler: &f32) -> Vector {
        Vector {
            x: v1.x * scaler,
            y: v1.y * scaler,
            z: v1.z * scaler,
            w: 1.0,
        }
    }

    pub(super) fn div_scaler(v1: &Vector, scaler: &f32) -> Vector {
        Vector {
            x: v1.x / scaler,
            y: v1.y / scaler,
            z: v1.z / scaler,
            w: 1.0,
        }
    }

    pub(super) fn dot(v1: &Vector, v2: &Vector) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub(super) fn cross(v1: &Vector, v2: &Vector) -> Vector {
        Vector {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
            w: 1.0,
        }
    }

    pub(super) fn mag(v: &Vector) -> f32 {
        sqrtf(v.x * v.x + v.y * v.y + v.z * v.z)
    }

    pub(super) fn norm(v: &Vector) -> Vector {
        let mag = Vector::mag(v);
        Vector {
            x: v.x / mag,
            y: v.y / mag,
            z: v.z / mag,
            w: 1.0,
        }
    }

    pub(super) fn intersect_plane(
        plane_point: Vector,
        mut plane_normal: Vector,
        line_start: Vector,
        line_end: Vector,
    ) -> Vector {
        //Make sure plane normal is indeed normal
        plane_normal = Vector::norm(&plane_normal);
        let plane_d = -Vector::dot(&plane_normal, &plane_point);
        let ad = Vector::dot(&line_start, &plane_normal);
        let bd = Vector::dot(&line_end, &plane_normal);
        let t = (-plane_d - ad) / (bd - ad);
        let line_start_to_end = Vector::sub(&line_end, &line_start);
        let line_to_intersect = Vector::mult_scaler(&line_start_to_end, &t);
        Vector::add(&line_start, &line_to_intersect)
    }

    pub(super) fn clip_plane(
        plane_point: Vector,
        mut plane_normal: Vector,
        in_tri: Triangle,
    ) -> [Option<Triangle>; 2] {
        //Make sure plane normal is indeed normal
        plane_normal = Vector::norm(&plane_normal);

        let dist = |v: &Vector| -> f32 {
            Vector::dot(&plane_normal, v) - Vector::dot(&plane_normal, &plane_point)
        };

        //Create two temporary storage arrays to classify points either side of plane
        //If distance sign is positive, point lies on "inside" of plane
        let mut inside_points: [Vector; 3] = [Vector::new(); 3];
        let mut outside_points: [Vector; 3] = [Vector::new(); 3];

        //Get signed distance of each point in triangle to plane
        let mut inside_point_count = 0;
        let mut outside_point_count = 0;
        let mut dists: [f32; 3] = [0.0; 3];
        for i in 0..3 {
            dists[i] = dist(&in_tri.p[i]);
            if dists[i] >= 0.0 {
                inside_points[inside_point_count] = in_tri.p[i];
                inside_point_count += 1;
            } else {
                outside_points[outside_point_count] = in_tri.p[i];
                outside_point_count += 1;
            }
        }
        //return correct triangles
        if inside_point_count == 0 {
            //All points lie on the outside of plane, so clip whole triangle
            //It ceases to exist
            return [None, None];
        }
        if inside_point_count == 1 {
            //Triangle should be clipped. As two points lie outside
            //the plane, the triangle simply becomes a smaller triangle
            let mut out_tri = Triangle::new();
            out_tri.color = 1;

            //The inside point is valid, so keep that...
            out_tri.p[0] = inside_points[0];
            //but the two new points are at the locations where the
            //original sides of the triangle (lines) intersect with the plane
            out_tri.p[1] = Vector::intersect_plane(
                plane_point,
                plane_normal,
                inside_points[0],
                outside_points[0],
            );
            out_tri.p[2] = Vector::intersect_plane(
                plane_point,
                plane_normal,
                inside_points[0],
                outside_points[1],
            );
            //Return the newly formed single triangle
            return [Some(out_tri), None];
        }
        if inside_point_count == 2 {
            //Triangle should be clipped. As two points lie inside the plane,
            //the clipped triangle becomes a "quad". Fortunately, we can
            //represent a quad with two new triangles
            let mut out_tri1 = Triangle::new();
            out_tri1.color = 2;
            let mut out_tri2 = Triangle::new();
            out_tri2.color = 4;
            //The first triangle consists of the two inside points and a new
            //point determined by the location where one side of the triangle
            //intersected with the plane
            out_tri1.p[0] = inside_points[0];
            out_tri1.p[1] = inside_points[1];
            out_tri1.p[2] = Vector::intersect_plane(
                plane_point,
                plane_normal,
                inside_points[0],
                outside_points[0],
            );
            //The second triangle is composed of one of he inside points, a
            //new point determined by the intersection of the other side of the
            //triangle and the plane, and the newly created point above
            out_tri2.p[0] = inside_points[1];
            out_tri2.p[1] = out_tri1.p[2];
            out_tri2.p[2] = Vector::intersect_plane(
                plane_point,
                plane_normal,
                inside_points[1],
                outside_points[0],
            );
            //Return two newly formed triangles which form a quad
            return [Some(out_tri1), Some(out_tri2)];
        }
        if inside_point_count == 3 {
            //All points lie on the inside of plane, so do nothing
            //and allow the triangle to simply pass through
            return [Some(in_tri), None];
        }

        todo!()
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct Triangle {
    pub(super) p: [Vector; 3],
    pub(super) color: u8,
}

impl Triangle {
    pub(super) fn new() -> Self {
        Self {
            p: [Vector::new(), Vector::new(), Vector::new()],
            color: 0,
        }
    }
}

pub(super) struct Mesh {
    pub(super) tris: Vec<Triangle>,
}
#[derive(Debug)]
pub(super) struct Matrix4x4 {
    pub(super) m: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub(super) fn new() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }

    pub(super) fn create_rot_x(theta: f32) -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cosf(theta), -sinf(theta), 0.0],
                [0.0, sinf(theta), cosf(theta), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub(super) fn create_rot_y(theta: f32) -> Self {
        Self {
            m: [
                [cosf(theta), 0.0, sinf(theta), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-sinf(theta), 0.0, cosf(theta), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub(super) fn create_rot_z(theta: f32) -> Self {
        Self {
            m: [
                [cosf(theta), -sinf(theta), 0.0, 0.0],
                [sinf(theta), cosf(theta), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub(super) fn create_translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ],
        }
    }

    pub(super) fn mult(m1: &Self, m2: &Self) -> Self {
        let mut m = Self::new();
        for c in 0..4 {
            for r in 0..4 {
                m.m[r][c] = m1.m[r][0] * m2.m[0][c]
                    + m1.m[r][1] * m2.m[1][c]
                    + m1.m[r][2] * m2.m[2][c]
                    + m1.m[r][3] * m2.m[3][c];
            }
        }
        m
    }

    pub(super) fn mult_vec(m: &Self, i: &Vector) -> Vector {
        let mut v = Vector::new();
        v.x = i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + i.w * m.m[3][0];
        v.y = i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + i.w * m.m[3][1];
        v.z = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + i.w * m.m[3][2];
        v.w = i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + i.w * m.m[3][3];
        v
    }

    pub(super) fn point_at(pos: &Vector, target: &Vector, up: &Vector) -> Self {
        // Calculate forward direction
        let mut forward = Vector::sub(&target, &pos);
        forward = Vector::norm(&forward);

        // Calculate up direction
        let a = Vector::mult_scaler(&forward, &Vector::dot(&up, &forward));
        let mut new_up = Vector::sub(&up, &a);
        new_up = Vector::norm(&new_up);

        //Create right direction
        let right = Vector::cross(&new_up, &forward);

        Self {
            m: [
                [right.x, right.y, right.z, 0.0],
                [new_up.x, new_up.y, new_up.z, 0.0],
                [forward.x, forward.y, forward.z, 0.0],
                [pos.x, pos.y, pos.z, 1.0],
            ],
        }
    }

    ///This function only works for rotation/translation matrices
    pub(super) fn quick_inverse(m: &Self) -> Self {
        let mut matrix = Matrix4x4::new();
        matrix.m[0][0] = m.m[0][0];
        matrix.m[0][1] = m.m[1][0];
        matrix.m[0][2] = m.m[2][0];
        matrix.m[0][3] = 0.0;
        matrix.m[1][0] = m.m[0][1];
        matrix.m[1][1] = m.m[1][1];
        matrix.m[1][2] = m.m[2][1];
        matrix.m[1][3] = 0.0;
        matrix.m[2][0] = m.m[0][2];
        matrix.m[2][1] = m.m[1][2];
        matrix.m[2][2] = m.m[2][2];
        matrix.m[2][3] = 0.0;
        matrix.m[3][0] =
            -(m.m[3][0] * matrix.m[0][0] + m.m[3][1] * matrix.m[1][0] + m.m[3][2] * matrix.m[2][0]);
        matrix.m[3][1] =
            -(m.m[3][0] * matrix.m[0][1] + m.m[3][1] * matrix.m[1][1] + m.m[3][2] * matrix.m[2][1]);
        matrix.m[3][2] =
            -(m.m[3][0] * matrix.m[0][2] + m.m[3][1] * matrix.m[1][2] + m.m[3][2] * matrix.m[2][2]);
        matrix.m[3][3] = 1.0;
        return matrix;
    }
}

impl Mesh {
    pub(super) fn from_obj_file(file: &str) -> Mesh {
        ///This function mutates cur_char
        fn add_vector(file: &[u8], mut cur_char: usize) -> Vector {
            let mut x = String::new();
            let mut y = String::new();
            let mut z = String::new();

            let mut iterations = 0;

            while cur_char < file.len() && iterations < 3 {
                //Remove spaces
                while file[cur_char] as char == ' ' {
                    cur_char += 1;
                }

                //Generate number
                let mut num = String::new();
                while cur_char < file.len()
                    && vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.', '-']
                        .contains(&(file[cur_char] as char))
                {
                    num.push(file[cur_char] as char);
                    cur_char += 1;
                }

                //Add number to correct vector
                match iterations {
                    0 => x = num,
                    1 => y = num,
                    2 => z = num,
                    _ => panic!("Invalid number of iterations"),
                }

                iterations += 1;
            }

            return Vector {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                z: z.parse().unwrap(),
                w: 1.0,
            };
        }

        fn add_face(file: &[u8], mut cur_char: usize, verts: &Vec<Vector>) -> Vec<Triangle> {
            let mut v1 = String::new();
            let mut v2 = String::new();
            let mut v3 = String::new();

            let mut iterations = 0;

            while cur_char < file.len() && iterations < 3 {
                //Remove spaces
                while file[cur_char] as char == ' ' {
                    cur_char += 1;
                }

                //Generate number

                let mut num = String::new();
                while cur_char < file.len() && (file[cur_char] as char).is_digit(10) {
                    num.push(file[cur_char] as char);
                    cur_char += 1;
                }

                //Add number to correct vector
                match iterations {
                    0 => v1 = num,
                    1 => v2 = num,
                    2 => v3 = num,
                    _ => panic!("Invalid number of iterations"),
                }

                iterations += 1;
            }

            let v1 = v1.parse::<usize>().unwrap() - 1;
            let v2 = v2.parse::<usize>().unwrap() - 1;
            let v3 = v3.parse::<usize>().unwrap() - 1;

            let mut tris = Vec::new();

            tris.push(Triangle {
                p: [verts[v1], verts[v2], verts[v3]],
                color: 0,
            });
            tris
        }

        //strings are encoded in UTF-8 and we need to convert them to bytes to index them
        let file = file.as_bytes();

        //Local cache of verts
        let mut verts = Vec::new();
        let mut tris = Vec::new();
        let mut cur_char = 0;

        while cur_char < file.len() {
            if file[cur_char] as char == 'v' {
                verts.push(add_vector(&file, cur_char + 1));
            } else if file[cur_char] as char == 'f' {
                tris.append(&mut add_face(&file, cur_char + 1, &verts));
            }
            cur_char += 1;
        }
        Mesh { tris }
    }
}

#[cfg(test)]
mod test {
    use super::{Matrix4x4, Vector};

    #[test_case]
    fn vector_test() {
        //Test vector addition
        let v1 = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };
        let v2 = Vector {
            x: 4.0,
            y: 5.0,
            z: 6.0,
            w: 1.0,
        };
        let v3 = Vector {
            x: 5.0,
            y: 7.0,
            z: 9.0,
            w: 1.0,
        };
        let v4 = Vector::add(&v1, &v2);
        assert_eq!(v4.x, v3.x);
        assert_eq!(v4.y, v3.y);
        assert_eq!(v4.z, v3.z);
        //Test vector subtraction
        let v5 = Vector::sub(&v3, &v2);
        assert_eq!(v5.x, v1.x);
        assert_eq!(v5.y, v1.y);
        assert_eq!(v5.z, v1.z);
        //Test vector multiplication
        let v6 = Vector::mult_scaler(&v1, &2.0);
        assert_eq!(v6.x, 2.0);
        assert_eq!(v6.y, 4.0);
        assert_eq!(v6.z, 6.0);
        //Test vector division
        let v7 = Vector::div_scaler(&v6, &2.0);
        assert_eq!(v7.x, 1.0);
        assert_eq!(v7.y, 2.0);
        assert_eq!(v7.z, 3.0);
        //Test vector dot product
        let v8 = Vector::dot(&v1, &v2);
        assert_eq!(v8, 32.0);
        //Test vector cross product
        let v9 = Vector::cross(&v1, &v2);
        assert_eq!(v9.x, -3.0);
        assert_eq!(v9.y, 6.0);
        assert_eq!(v9.z, -3.0);
        //Test vector magnitude
        let v10 = Vector::mag(&v1);
        assert_eq!(v10, 3.7416573867739413);
        //Test vector normalization
        let v11 = Vector::norm(&v1);
        assert_eq!(v11.x, 0.2672612419124244);
        assert_eq!(v11.y, 0.5345224838248488);
        assert_eq!(v11.z, 0.8017837);
    }

    #[test_case]
    fn test_matrix() {
        //test matrix multiplication
        let m1 = Matrix4x4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ],
        };
        let m2 = Matrix4x4 {
            m: [
                [17.0, 18.0, 19.0, 20.0],
                [21.0, 22.0, 23.0, 24.0],
                [25.0, 26.0, 27.0, 28.0],
                [29.0, 30.0, 31.0, 32.0],
            ],
        };
        let m3 = Matrix4x4 {
            m: [
                [250.0, 260.0, 270.0, 280.0],
                [618.0, 644.0, 670.0, 696.0],
                [986.0, 1028.0, 1070.0, 1112.0],
                [1354.0, 1412.0, 1470.0, 1528.0],
            ],
        };
        let m4 = Matrix4x4::mult(&m1, &m2);
        assert_eq!(m4.m[0][0], m3.m[0][0]);
        assert_eq!(m4.m[0][1], m3.m[0][1]);
        assert_eq!(m4.m[0][2], m3.m[0][2]);
        assert_eq!(m4.m[0][3], m3.m[0][3]);
        assert_eq!(m4.m[1][0], m3.m[1][0]);
        assert_eq!(m4.m[1][1], m3.m[1][1]);
        assert_eq!(m4.m[1][2], m3.m[1][2]);
        assert_eq!(m4.m[1][3], m3.m[1][3]);
        assert_eq!(m4.m[2][0], m3.m[2][0]);
        assert_eq!(m4.m[2][1], m3.m[2][1]);
        assert_eq!(m4.m[2][2], m3.m[2][2]);
        assert_eq!(m4.m[2][3], m3.m[2][3]);
        assert_eq!(m4.m[3][0], m3.m[3][0]);
        assert_eq!(m4.m[3][1], m3.m[3][1]);
        assert_eq!(m4.m[3][2], m3.m[3][2]);
        assert_eq!(m4.m[3][3], m3.m[3][3]);

        //test matrix vector multiplication
        let v1 = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };
        let v2 = Vector {
            x: 51.0,
            y: 58.0,
            z: 65.0,
            w: 72.0,
        };
        let v3 = Matrix4x4::mult_vec(&m1, &v1);
        assert_eq!(v3.x, v2.x);
        assert_eq!(v3.y, v2.y);
        assert_eq!(v3.z, v2.z);
        assert_eq!(v3.w, v2.w);
    }
}
