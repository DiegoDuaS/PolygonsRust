mod framebuffer;
mod bmp;
mod line;
mod polygon;

use std::io;
use framebuffer::Framebuffer;
use crate::bmp::write_bmp_file; 
use nalgebra::Vector3;
use crate::line::Line;
use crate::polygon::Polygon;

fn main() -> io::Result<()> {
    let width = 500;
    let height = 500;
    let mut framebuffer = Framebuffer::new(width, height);

    // Dibujar algo en el framebuffer
    framebuffer.clear();
    framebuffer.set_current_color(0xFFFF0000);  // Rojo opaco


    // Dibuja un polígono
    let points = vec![
        Vector3::new(10.0, 10.0, 0.0),
        Vector3::new(10.0, 90.0, 0.0),
        Vector3::new(90.0, 90.0, 0.0),
        Vector3::new(90.0, 10.0, 0.0),
    ];
    framebuffer.draw_polygon(&points);
    framebuffer.fill_polygon(&points, 0xFF00FF00);

    // Guardar como archivo BMP
    let filename = "prueba2.bmp";
    write_bmp_file(filename, width as u32, height as u32, &framebuffer.buffer)?;

    println!("Archivo BMP generado: {}", filename);
    Ok(())
}
