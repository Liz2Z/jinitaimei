use opencv::prelude::*;

pub fn get_gray_val(input: Mat) -> Vec<Vec<i32>> {
    let mut mat2 = Mat::default();

    input
        .convert_to(&mut mat2, opencv::core::CV_32SC1, 1.0, 0.0)
        .unwrap();

    let mut vec_2d: Vec<Vec<i32>> = vec![];

    for i in 0..mat2.rows() {
        let mut vec: Vec<i32> = vec![];
        for j in 0..mat2.cols() {
            vec.push(mat2.at_2d::<i32>(i, j).unwrap().clone());
        }

        vec_2d.push(vec);
    }

    vec_2d
}
