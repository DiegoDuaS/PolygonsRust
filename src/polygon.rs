use nalgebra::Vector3;
use crate::framebuffer::Framebuffer;
use crate::line::Line;

pub trait Polygon {
    fn draw_polygon(&mut self, points: &[Vector3<f32>]);
    fn fill_polygon(&mut self, points: &[Vector3<f32>], color: u32);
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

    fn fill_polygon(&mut self, points: &[Vector3<f32>], color: u32) {
        if points.len() < 3 {
            return; // No se puede rellenar un polígono con menos de 3 puntos
        }

        // Encontrar los límites del polígono
        let mut min_x = self.width as i32;
        let mut max_x = 0;
        let mut min_y = self.height as i32;
        let mut max_y = 0;
        
        for point in points {
            let x = point.x as i32;
            let y = point.y as i32;
            if x < min_x { min_x = x; }
            if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
        }

        // Algoritmo de escaneo de líneas
        for y in min_y..=max_y {
            let mut inside = false;
            let mut intersections = Vec::new();

            // Encontrar intersecciones con las líneas del polígono
            for i in 0..points.len() {
                let j = (i + 1) % points.len();
                let point_i = &points[i];
                let point_j = &points[j];

                let (x0, y0) = (point_i.x as i32, point_i.y as i32);
                let (x1, y1) = (point_j.x as i32, point_j.y as i32);

                if (y0 <= y && y < y1) || (y1 <= y && y < y0) {
                    let x_intersect = (x0 as f64) + (y as f64 - y0 as f64) *
                                      (x1 as f64 - x0 as f64) / (y1 as f64 - y0 as f64);
                    intersections.push(x_intersect);
                }
            }

            // Ordenar intersecciones
            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            // Rellenar entre las intersecciones
            for i in (0..intersections.len()).step_by(2) {
                let start = intersections[i].max(0.0) as usize;
                let end = intersections[i + 1].min(self.width as f64 - 1.0) as usize;

                for x in start..=end {
                    self.point(x, y as usize);
                }
            }
        }
    }
}
