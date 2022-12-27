extern crate image;
extern crate num_complex;
// extern crate imageproc;

use image::error::UnsupportedErrorKind::Color;
use image::Rgb;
use imageproc::drawing::{ draw_line_segment_mut };


/// An example of generating julia fractals.
pub fn generate(imgx: u32, imgy: u32) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    let line_colour = Rgb([255u8, 255u8, 0u8]);

    draw_line_segment_mut(&mut imgbuf, (0.0,0.0), (imgx as f32,imgy as f32), line_colour);
    draw_line_segment_mut(&mut imgbuf, (imgx as f32,0.0), (0.0,imgy as f32), line_colour);

    return imgbuf;
}
