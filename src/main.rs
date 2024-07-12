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


    // Guardar como archivo BMP
    let filename = "prueba2.bmp";
    write_bmp_file(filename, width as u32, height as u32, &framebuffer.buffer)?;

    println!("Archivo BMP generado: {}", filename);
    Ok(())
}
