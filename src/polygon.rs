use nalgebra::Vector3;
use crate::framebuffer::Framebuffer;
use crate::line::Line;

pub trait Polygon {
    fn draw_polygon(&mut self, points: &[Vector3<f32>]);
}

impl Polygon for Framebuffer {
    fn draw_polygon(&mut self, points: &[Vector3<f32>]) {
        if points.len() < 2 {
            return;
        }
        
        for i in 0..points.len() {
            let start = points[i];
            let end: nalgebra::Matrix<f32, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 3, 1>> = points[(i + 1) % points.len()]; // Cerrar el polígono
            self.draw_line(start, end);
        }
    }
}
