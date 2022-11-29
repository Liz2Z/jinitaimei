use opencv::core::Vector;
use opencv::imgcodecs::imread;
use opencv::imgcodecs::imwrite;
use opencv::imgcodecs::ImreadModes;
use opencv::imgcodecs::ImwriteFlags;
use opencv::imgproc::resize;
use opencv::imgproc::InterpolationFlags;
use opencv::prelude::*;

pub struct Config<'a> {
    pub input: &'a str,
    pub dest: &'a str,
}

pub fn proc(config: Config) {
    let result = imread(config.input, ImreadModes::IMREAD_UNCHANGED as i32).unwrap();

    let mut dest = Mat::default();

    resize(
        &result,
        &mut dest,
        // 调整分辨率
        opencv::core::Size_ {
            width: 128,
            height: 128,
        },
        0.5,
        0.5,
        InterpolationFlags::INTER_CUBIC as i32,
    )
    .unwrap();

    let mut params: Vector<i32> = Vector::default();

    params.push(ImwriteFlags::IMWRITE_JPEG_QUALITY as i32);
    params.push(100);

    imwrite(config.dest, &dest, &params).unwrap();
}
