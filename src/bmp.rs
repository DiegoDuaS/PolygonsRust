use std::fs::File;
use std::io::{self, Write, BufWriter};

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 32;

pub fn write_bmp_file(filename: &str, width: u32, height: u32, data: &[u32]) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    write_bmp_header(&mut writer, width, height)?;
    write_pixel_data(&mut writer, width, height, data)?;

    writer.flush()?;
    Ok(())
}

fn write_bmp_header(writer: &mut BufWriter<File>, width: u32, height: u32) -> io::Result<()> {
    // BMP File Header (14 bytes)
    let file_size = BMP_HEADER_SIZE + (width as usize * height as usize * 4);
    let file_header = [
        0x42, 0x4D,                                 // Tipo de archivo "BM"
        (file_size & 0xFF) as u8,                   // Tamaño del archivo (byte 1)
        ((file_size >> 8) & 0xFF) as u8,            // Tamaño del archivo (byte 2)
        ((file_size >> 16) & 0xFF) as u8,           // Tamaño del archivo (byte 3)
        ((file_size >> 24) & 0xFF) as u8,           // Tamaño del archivo (byte 4)
        0, 0, 0, 0,                                 // Reservado
        BMP_HEADER_SIZE as u8, 0, 0, 0,             // Offset del comienzo de los datos de la imagen
    ];


    // BMP Info Header (40 bytes)
    let info_header = [
        40, 0, 0, 0,                            // Tamaño del encabezado de la imagen
        width as u8, (width >> 8) as u8, (width >> 16) as u8, (width >> 24) as u8, // Ancho de la imagen
        height as u8, (height >> 8) as u8, (height >> 16) as u8, (height >> 24) as u8, // Altura de la imagen
        1, 0,                                   // Número de planos de color (siempre 1)
        BMP_BITS_PER_PIXEL as u8, 0,            // Bits por píxel (32 para RGBA)
        0, 0, 0, 0,                             // Tipo de compresión (sin compresión)
        0, 0, 0, 0,                             // Tamaño de los datos de la imagen (puede ser 0 para sin compresión)
        0, 0, 0, 0,                             // Resolución horizontal de píxeles por metro (no relevante)
        0, 0, 0, 0,                             // Resolución vertical de píxeles por metro (no relevante)
        0, 0, 0, 0,                             // Número de colores en la paleta (0 para sin paleta)
        0, 0, 0, 0,                             // Número de colores importantes (no relevante)
    ];

    // Escribir encabezado de archivo BMP
    writer.write_all(&file_header)?;

    // Escribir encabezado de imagen BMP
    writer.write_all(&info_header)?;

    Ok(())
}

fn write_pixel_data(writer: &mut BufWriter<File>, width: u32, height: u32, data: &[u32]) -> io::Result<()> {
    for y in (0..height).rev() {
        for x in 0..width {
            let index = (y * width + x) as usize;
            let pixel = data[index];

            let b = (pixel & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let r = ((pixel >> 16) & 0xFF) as u8;
            let a = ((pixel >> 24) & 0xFF) as u8;

            // Escribir píxel en orden RGBA (32 bits por píxel)
            writer.write_all(&[b, g, r, a])?;
        }
    }

    Ok(())
}
