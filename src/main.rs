#[macro_use] extern crate rocket;
mod gen_image;

extern crate image;
extern crate num_complex;

use std::io::Cursor;
use crate::gen_image::generate;

use rocket::http::ContentType;


#[get("/")]
fn index() -> (ContentType, Vec<u8>) {
    let imgx = 1800;
    let imgy = 800;

    let imgbuf = generate(imgx, imgy);
    let mut bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    imgbuf.write_to(&mut bytes, image::ImageOutputFormat::from(image::ImageFormat::Jpeg)).expect("");

    return (ContentType::JPEG, bytes.into_inner());
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
