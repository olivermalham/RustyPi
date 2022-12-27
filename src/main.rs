mod gen_image;

extern crate image;
extern crate num_complex;

use std::io::Cursor;
use crate::gen_image::generate;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::ContentType;


fn image() -> Vec<u8> {
    let imgx = 1800;
    let imgy = 800;

    let imgbuf = generate(imgx, imgy);

    // for n in 1..100 {
    //     imgbuf = generate(imgx, imgy);
    // }

    let mut bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    imgbuf.write_to(&mut bytes,
                    image::ImageOutputFormat::from(image::ImageFormat::Jpeg)
                    ).expect("");

    bytes.into_inner()
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::jpeg()).body(image())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
