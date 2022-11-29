use super::info;
use opencv::imgcodecs::imread;
use opencv::imgcodecs::ImreadModes;
use opencv::imgproc::cvt_color;
use opencv::imgproc::resize;
use opencv::imgproc::ColorConversionCodes;
use opencv::imgproc::InterpolationFlags;
use opencv::prelude::*;

// 黑色背景白色字体情况下，灰度值：黑 -> 白
const CHAR_LIST: [char; 8] = [' ', '`', ':', '^', '>', '\\', '?', '@'];
const GRAY_INTERVAL: f64 = (255 / CHAR_LIST.len()) as f64;

/// 将灰度值转换成对应的 ascii
fn gray_to_ascii(gray: f64) -> char {
    let const_val: f64 = GRAY_INTERVAL.ceil();
    let char_index = (gray / const_val).floor() as usize;
    *CHAR_LIST.get(char_index).unwrap()
}

pub fn proc(input: &str) {
    let raw_mat = imread(input, ImreadModes::IMREAD_UNCHANGED as i32).unwrap();

    // 转换颜色到灰度
    let mut gray_dest = Mat::default();
    cvt_color(
        &raw_mat,
        &mut gray_dest,
        ColorConversionCodes::COLOR_RGB2GRAY as i32,
        0,
    )
    .unwrap();

    // 调整分辨率
    let mut resize_dest = Mat::default();
    resize(
        &gray_dest,
        &mut resize_dest,
        opencv::core::Size_ {
            width: 128,
            height: 128,
        },
        0.5,
        0.5,
        InterpolationFlags::INTER_CUBIC as i32,
    )
    .unwrap();

    // 打印到控制台
    let vec_2d = info::get_gray_val(resize_dest);

    for row in vec_2d.iter() {
        let mut str = String::new();
        for cell in row.iter() {
            str.push(gray_to_ascii(*cell as f64));
        }

        println!("{}", str);
    }
}
