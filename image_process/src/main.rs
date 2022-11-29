use opencv::imgcodecs::imread;
use opencv::imgcodecs::ImreadModes;
use opencv::imgproc::cvt_color;
use opencv::imgproc::resize;
use opencv::imgproc::ColorConversionCodes;
use opencv::imgproc::InterpolationFlags;
use opencv::prelude::*;
mod info;

fn main() {
    let input = "./OIP-C.jpeg";

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

    let mut char_array = ['@', '?', '\\', '>', '^', ':', '`', ' '];
    char_array.reverse();

    // 打印到控制台
    let vec_2d = info::get_gray_val(resize_dest);

    for row in vec_2d.iter() {
        let mut str = String::new();
        for cell in row.iter() {
            let a = *cell as f64;
            let a = (a / 32.0).floor();

            str.push(*char_array.get(a as usize).unwrap());
            // vec.push(cell);
        }

        println!("{}", str);
    }
}
