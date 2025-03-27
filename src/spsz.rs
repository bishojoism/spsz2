use crate::shipin;
use std::process::Command;

pub fn jiamibofang(input: &str, seed: &str) {
    Command::new("ffplay")
        .arg("-")
        .stdin(shipin::jiami(input, seed))
        .spawn()
        .expect("ffplay failed to execute")
        .wait()
        .expect("ffplay failed to wait");
}

pub fn jiemibofang(input: &str, seed: &str) {
    Command::new("ffplay")
        .arg("-")
        .stdin(shipin::jiemi(input, seed))
        .spawn()
        .expect("ffplay failed to execute")
        .wait()
        .expect("ffplay failed to wait");
}

pub fn jiamibaocun(input: &str, output: &str, seed: &str) {
    Command::new("ffmpeg")
        .arg("-i")
        .arg("-")
        .arg(output)
        .arg("-y")
        .stdin(shipin::jiami(input, seed))
        .spawn()
        .expect("ffmpeg failed to execute")
        .wait()
        .expect("ffmpeg failed to wait");
}

pub fn jiemibaocun(input: &str, output: &str, seed: &str) {
    Command::new("ffmpeg")
        .arg("-i")
        .arg("-")
        .arg(output)
        .arg("-y")
        .stdin(shipin::jiemi(input, seed))
        .spawn()
        .expect("ffmpeg failed to execute")
        .wait()
        .expect("ffmpeg failed to wait");
}
