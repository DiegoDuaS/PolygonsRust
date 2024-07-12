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
    let points = vec![
        Vector3::new(165.0, 380.0, 0.0),
        Vector3::new(185.0, 360.0, 0.0),
        Vector3::new(180.0, 330.0, 0.0),
        Vector3::new(207.0, 345.0, 0.0),
        Vector3::new(233.0, 330.0, 0.0),
        Vector3::new(230.0, 360.0, 0.0),
        Vector3::new(250.0, 380.0, 0.0),
        Vector3::new(220.0, 385.0, 0.0),
        Vector3::new(205.0, 410.0, 0.0),
        Vector3::new(193.0, 383.0, 0.0),
    ];

    framebuffer.set_current_color(0xFFFFFF00);
    framebuffer.fill_polygon(&points, 0xFFFFFF00);

    framebuffer.set_current_color(0xFFFFFFFF);  
    framebuffer.draw_polygon(&points);

    

    // Guardar como archivo BMP
    let filename = "poligono1.bmp";
    write_bmp_file(filename, width as u32, height as u32, &framebuffer.buffer)?;

    println!("Archivo BMP generado: {}", filename);
    Ok(())
}
