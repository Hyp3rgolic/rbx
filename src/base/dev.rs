use crate::math::prelude::*;
use glam::{Mat4, Vec2, Vec3, Vec4};

pub fn test_ground() {}

pub fn clear(buffer: &mut [u32]) {
    for (index, color) in buffer.iter_mut().enumerate() {
        *color = 0x00000000;
    }
}

pub fn draw_pixel(buffer: &mut [u32], size: &winit::dpi::PhysicalSize<u32>, v: Vec2, color: u32) {
    if v.x >= 0.0 && v.y >= 0.0 && v.x < size.width as f32 && v.y < size.height as f32 {
        let index = (dbg!(v.x) + v.y * size.width as f32) as usize;

        buffer[index] = color;
    }
}

pub fn render(
    buffer: &mut [u32],
    size: &winit::dpi::PhysicalSize<u32>,
    meshes: &Vec<Mesh>,
    proj_mat: Mat4,
    view_mat: Mat4,
) {
    for i in meshes {
        for v in &i.vertices {
            let p = proj_mat * view_mat * Vec4::new(v[0], v[1], v[2], 1.0);

            let x = (p.x / p.w + 1.0) * size.width as f32 / 2.0;
            let y = (1.0 - p.y / p.w) * size.height as f32 / 2.0;

            draw_pixel(buffer, size, Vec2::new(x, y), 0x00ffffff);
        }
    }
}

pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

pub struct Mesh {
    vertices: Vec<[f32; 3]>,
    position: Vec3,
    rotation: Vec3,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: vec![
                [-0.5, -0.5, 0.5],
                [0.5, -0.5, 0.5],
                [0.5, 0.5, 0.5],
                [-0.5, 0.5, 0.5],
                // Back face
                [-0.5, -0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5],
                [0.5, -0.5, -0.5],
                // Top face
                [-0.5, 0.5, -0.5],
                [-0.5, 0.5, 0.5],
                [0.5, 0.5, 0.5],
                [0.5, 0.5, -0.5],
                // Bottom face
                [-0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5],
                [0.5, -0.5, 0.5],
                [-0.5, -0.5, 0.5],
                // Right face
                [0.5, -0.5, -0.5],
                [0.5, 0.5, -0.5],
                [0.5, 0.5, 0.5],
                [0.5, -0.5, 0.5],
                // Left face
                [-0.5, -0.5, -0.5],
                [-0.5, -0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [-0.5, 0.5, -0.5],
            ],
            position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}
