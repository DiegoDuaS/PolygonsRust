use nalgebra::Vector3;
use crate::framebuffer::Framebuffer;

pub trait Line {
    fn draw_line(&mut self, start: Vector3<f32>, end: Vector3<f32>);
}

fn bresenham_line<F>(mut x0: i32, mut y0: i32, x1: i32, y1: i32, mut plot: F)
where
    F: FnMut(i32, i32),
{
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        plot(x0, y0);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

impl Line for Framebuffer {
    fn draw_line(&mut self, start: Vector3<f32>, end: Vector3<f32>) {
        bresenham_line(
            start.x as i32,
            start.y as i32,
            end.x as i32,
            end.y as i32,
            |x, y| {
                if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
                    self.point(x as usize, y as usize);
                }
            },
        );
    }
}
