use std::process::Command;

mod saozi;
mod shipin;
mod shuffle;

fn main() {
    // 测试saozi.rs
    // let seed = "1234567890";
    // let length = 10;
    // let raw = shuffle::shuffle(length, seed);
    // let mut encrypted = vec![];
    // for (index, size) in saozi::encrypt(raw.len(), seed) {
    //     encrypted.extend_from_slice(&raw[index..index + size]);
    // }
    // let mut decrypted = vec![];
    // for index in saozi::decrypt(encrypted.len(), seed) {
    //     decrypted.push(encrypted[index]);
    // }
    // assert_eq!(raw, decrypted);

    //测试shipin.rs
    Command::new("ffmpeg")
        .arg("-i")
        .arg("-")
        .arg("E:/e.mp4")
        .arg("-y")
        .stdin(shipin::jiami("E:/1.mp4", "1234567890"))
        .status()
        .expect("ffmpeg failed to execute");
    Command::new("ffmpeg")
       .arg("-i")
       .arg("-")
       .arg("E:/d.mp4")
       .arg("-y")
       .stdin(shipin::jiemi("E:/e.mp4", "1234567890"))
       .status()
       .expect("ffmpeg failed to execute");
}
