use opencv::core::BorderTypes;
use opencv::core::Vector;
use opencv::imgcodecs::imread;
use opencv::imgcodecs::imwrite;
use opencv::imgcodecs::ImreadModes;
use opencv::imgcodecs::ImwriteFlags;
use opencv::imgproc::pyr_down;
use opencv::prelude::*;

pub fn proc() {
    let result = imread("./img.jpeg", ImreadModes::IMREAD_UNCHANGED as i32).unwrap();

    println!("Hello, world! {:#?}", result);

    let mut dest = Mat::default();

    pyr_down(
        &result,
        &mut dest,
        opencv::core::Size_ {
            width: 720,
            height: 450,
        },
        BorderTypes::BORDER_REPLICATE as i32,
    )
    .unwrap();

    let mut params: Vector<i32> = Vector::default();

    params.push(ImwriteFlags::IMWRITE_JPEG_QUALITY as i32);
    params.push(100);

    imwrite("./image.output.jpeg", &dest, &params).unwrap();
}
