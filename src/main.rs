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

   // Dibuja un polígono
   let points2 = vec![
    Vector3::new(321.0, 335.0, 0.0),
    Vector3::new(288.0, 286.0, 0.0),
    Vector3::new(339.0, 251.0, 0.0),
    Vector3::new(374.0, 302.0, 0.0),
];

    framebuffer.set_current_color(0xFF0000FF);
    framebuffer.fill_polygon(&points2, 0xFF0000FF);

    framebuffer.set_current_color(0xFFFFFFFF);  
    framebuffer.draw_polygon(&points2);


    // Guardar como archivo BMP
    let filename = "polygon1.bmp";
    write_bmp_file(filename, width as u32, height as u32, &framebuffer.buffer)?;

    println!("Archivo BMP generado: {}", filename);
    Ok(())
}
