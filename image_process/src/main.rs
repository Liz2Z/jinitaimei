mod img_to_ascii;
mod info;
use std::thread;
use std::time;

fn main() {
    for i in 1..107 {
        let suffix = if i < 10 {
            String::from("00") + &i.to_string()
        } else if i < 100 {
            String::from("0") + &i.to_string()
        } else {
            i.to_string()
        };

        let str = String::from("/Users/zan/Downloads/output/00000") + &suffix + ".jpg";

        img_to_ascii::proc(&str);
        thread::sleep(time::Duration::new(0, 60000000));
        print!("\x1b[2J");
        print!("\x1b[H");
    }
}
