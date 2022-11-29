use opencv::core::Vector;
use opencv::imgcodecs::imread;
use opencv::imgcodecs::imwrite;
use opencv::imgcodecs::ImreadModes;
use opencv::imgcodecs::ImwriteFlags;
use opencv::imgproc::cvt_color;
// use opencv::imgproc::threshold;
use opencv::imgproc::ColorConversionCodes;
// use opencv::imgproc::ThresholdTypes;
use opencv::prelude::*;

pub struct Config<'a> {
    pub input: &'a str,
    pub dest: &'a str,
}

pub fn proc(config: Config) -> Mat {
    let result = imread(config.input, ImreadModes::IMREAD_UNCHANGED as i32).unwrap();

    let mut gray_dest = Mat::default();

    // 转换颜色
    cvt_color(
        &result,
        &mut gray_dest,
        ColorConversionCodes::COLOR_RGB2GRAY as i32,
        0,
    )
    .unwrap();

    // 二值化
    // let mut res_dest = Mat::default();
    // threshold(
    //     &gray_dest,
    //     &mut res_dest,
    //     77f64,
    //     255f64,
    //     ThresholdTypes::THRESH_BINARY as i32,
    // )
    // .unwrap();

    gray_dest
}
