use std::process::{ChildStdout, Command, Stdio};

use eframe::emath::Numeric;

use crate::saozi::{decrypt, encrypt, n2m};

const N: usize = 120;

pub fn jiami(input: &str, seed: &str) -> ChildStdout {
    let binding = String::from_utf8(
        Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-select_streams")
            .arg("v:0")
            .arg("-show_entries")
            .arg("stream=width,height")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg(input)
            .output()
            .expect("ffprobe failed to execute")
            .stdout,
    )
    .expect("ffprobe output is not valid UTF-8");
    let dimensions = binding.lines().collect::<Vec<_>>();
    if dimensions.len() != 2 {
        panic!("ffprobe output is not valid");
    }
    let width = dimensions[0]
        .parse::<usize>()
        .expect("width is not a number");
    let height = dimensions[1]
        .parse::<usize>()
        .expect("height is not a number");

    let w = (width.to_f64() / N.to_f64()).ceil() as usize;
    let h = (height.to_f64() / N.to_f64()).ceil() as usize;

    let enlarged_width = w * N;
    let enlarged_height = h * N;
    let enlarged = Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-vf")
        .arg(format!("scale={}:{}", enlarged_width, enlarged_height))
        .arg("-f")
        .arg("matroska")
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()
        .expect("ffmpeg failed to execute")
        .stdout
        .expect("ffmpeg failed to get stdout");

    let result = encrypt(N, seed);

    let mut x_filter = String::new();
    let mut x_alias = 0;
    for (index, size) in &result {
        x_alias += 1;
        x_filter += &format!(
            "[0]crop={}:ih:{}:0:exact=1[{}];",
            size * w,
            index * w,
            x_alias
        );
    }
    for i in 1..=x_alias {
        x_filter += &format!("[{}]", i);
    }
    x_filter += &format!("hstack=inputs={}", x_alias);
    let x = Command::new("ffmpeg")
        .arg("-i")
        .arg("-")
        .arg("-filter_complex")
        .arg(x_filter)
        .arg("-f")
        .arg("matroska")
        .arg("-")
        .stdin(enlarged)
        .stdout(Stdio::piped())
        .spawn()
        .expect("ffmpeg failed to execute")
        .stdout
        .expect("ffmpeg failed to get stdout");

    let mut y_filter = String::new();
    let mut y_alias = 0;
    for (index, size) in &result {
        y_alias += 1;
        y_filter += &format!(
            "[0]crop=iw:{}:0:{}:exact=1[{}];",
            size * h,
            index * h,
            y_alias
        );
    }
    for i in 1..=y_alias {
        y_filter += &format!("[{}]", i);
    }
    y_filter += &format!("vstack=inputs={}", y_alias);
    let y = Command::new("ffmpeg")
        .arg("-i")
        .arg("-")
        .arg("-filter_complex")
        .arg(y_filter)
        .arg("-f")
        .arg("matroska")
        .arg("-")
        .stdin(x)
        .stdout(Stdio::piped())
        .spawn()
        .expect("ffmpeg failed to execute")
        .stdout
        .expect("ffmpeg failed to get stdout");

    let length = result.iter().fold(0, |acc, (_, size)| acc + size);
    let reduced_width = (length.to_f64() / N.to_f64() * width.to_f64() / 2.0).round() as usize * 2;
    let reduced_height = (length.to_f64() / N.to_f64() * height.to_f64() / 2.0).round() as usize * 2;
    let reduced = Command::new("ffmpeg")
        .arg("-i")
        .arg("-")
        .arg("-vf")
        .arg(format!("scale={}:{}", reduced_width, reduced_height))
        .arg("-f")
        .arg("matroska")
        .arg("-")
        .stdin(y)
        .stdout(Stdio::piped())
        .spawn()
        .expect("ffmpeg failed to execute")
        .stdout
        .expect("ffmpeg failed to get stdout");

    reduced
}

const M: usize = n2m(N);

pub fn jiemi(input: &str, seed: &str) -> ChildStdout {
    let binding = String::from_utf8(
        Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-select_streams")
            .arg("v:0")
            .arg("-show_entries")
            .arg("stream=width,height")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg(input)
            .output()
            .expect("ffprobe failed to execute")
            .stdout,
    )
    .expect("ffprobe output is not valid UTF-8");
    let dimensions = binding.lines().collect::<Vec<_>>();
    if dimensions.len() != 2 {
        panic!("ffprobe output is not valid");
    }
    let width = dimensions[0]
        .parse::<usize>()
        .expect("width is not a number");
    let height = dimensions[1]
        .parse::<usize>()
        .expect("height is not a number");

    let w = (width.to_f64() / M.to_f64()).ceil() as usize;
    let h = (height.to_f64() / M.to_f64()).ceil() as usize;

    let enlarged_width = w * M;
    let enlarged_height = h * M;
    let enlarged = Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-vf")
        .arg(format!("scale={}:{}", enlarged_width, enlarged_height))
        .arg("-f")
        .arg("matroska")
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()
        .expect("ffmpeg failed to execute")
        .stdout
        .expect("ffmpeg failed to get stdout");

    let result = decrypt(M, seed);
    let mut x_filter = String::new();
    let mut x_alias = 0;
    for index in &result {
        x_alias += 1;
        x_filter += &format!("[0]crop={}:ih:{}:0:exact=1[{}];", w, index * w, x_alias);
    }
    for i in 1..=x_alias {
        x_filter += &format!("[{}]", i);
    }
    x_filter += &format!("hstack=inputs={}", x_alias);
    let x = Command::new("ffmpeg")
        .arg("-i")
        .arg("-")
        .arg("-filter_complex")
        .arg(x_filter)
        .arg("-f")
        .arg("matroska")
        .arg("-")
        .stdin(enlarged)
        .stdout(Stdio::piped())
        .spawn()
        .expect("ffmpeg failed to execute")
        .stdout
        .expect("ffmpeg failed to get stdout");

    let mut y_filter = String::new();
    let mut y_alias = 0;
    for index in &result {
        y_alias += 1;
        y_filter += &format!("[0]crop=iw:{}:0:{}:exact=1[{}];", h, index * h, y_alias);
    }
    for i in 1..=y_alias {
        y_filter += &format!("[{}]", i);
    }
    y_filter += &format!("vstack=inputs={}", y_alias);
    let y = Command::new("ffmpeg")
        .arg("-i")
        .arg("-")
        .arg("-filter_complex")
        .arg(y_filter)
        .arg("-f")
        .arg("matroska")
        .arg("-")
        .stdin(x)
        .stdout(Stdio::piped())
        .spawn()
        .expect("ffmpeg failed to execute")
        .stdout
        .expect("ffmpeg failed to get stdout");

    let length = result.len();
    let reduced_width = (length.to_f64() / M.to_f64() * width.to_f64() / 2.0).round() as usize * 2;
    let reduced_height = (length.to_f64() / M.to_f64() * height.to_f64() / 2.0).round() as usize * 2;
    let reduced = Command::new("ffmpeg")
        .arg("-i")
        .arg("-")
        .arg("-vf")
        .arg(format!("scale={}:{}", reduced_width, reduced_height))
        .arg("-f")
        .arg("matroska")
        .arg("-")
        .stdin(y)
        .stdout(Stdio::piped())
        .spawn()
        .expect("ffmpeg failed to execute")
        .stdout
        .expect("ffmpeg failed to get stdout");

    reduced
}
