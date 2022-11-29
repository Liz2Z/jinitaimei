use opencv::core::Vector;
use opencv::imgcodecs::imwrite;
use opencv::imgcodecs::ImwriteFlags;
use opencv::prelude::*;

pub struct Config<'a> {
    dest: &'a str,
    img: Mat,
}

pub fn write_file(config: Config) {
    let mut params: Vector<i32> = Vector::default();
    params.push(ImwriteFlags::IMWRITE_JPEG_QUALITY as i32);
    params.push(100);

    imwrite(config.dest, &config.img, &params).unwrap();
}
