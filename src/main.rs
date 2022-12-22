#[macro_use] extern crate rocket;
mod gen_image;

extern crate image;
extern crate num_complex;

use std::io::Cursor;
use crate::gen_image::generate;

#[get("/")]
fn index() -> &'static str {
    let imgx = 800;
    let imgy = 800;

    let imgbuf = generate(imgx, imgy);
    // Save the image as “fractal.png”, the format is deduced from the path
    // imgbuf.save("fractal.png").unwrap();
    // let mut bytes = Vec::new();
    let mut bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    imgbuf.write_to(&mut bytes, image::ImageOutputFormat::from(image::ImageFormat::Jpeg)).expect("");

    // TODO: How can I return the image to the browser?
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
