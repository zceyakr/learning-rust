extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter };

pub fn ferris() {
    let stdout = stdout();
    let out = b"Hello fellow Rustacean!";
    let width = out.len();

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();

}
