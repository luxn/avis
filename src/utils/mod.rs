use glam::{Vec3, Mat4, Quat};

pub mod file;
pub mod random;

fn create_transformation_matrix(translation: Vec3, rx: f32, ry: f32, rz: f32, scale: f32) -> Mat4 {
    let scale_vec = Vec3::new(scale,scale,scale);
    let rotation = Quat::from_rotation_ypr(ry, rx, rz);

    Mat4::from_scale_rotation_translation(scale_vec, rotation, translation)
}