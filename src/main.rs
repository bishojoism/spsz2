use eframe;
use spsz::{jiamibaocun, jiamibofang, jiemibaocun, jiemibofang};

mod saozi;
mod shipin;
mod shuffle;
mod spsz;

struct MyApp {
    seed: String,
    url: String,
    input: String,
    output: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            seed: String::new(),
            url: String::new(),
            input: String::new(),
            output: String::new(),
        }
    }
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = eframe::egui::FontDefinitions::default();
        fonts.font_data.insert(
            "simsun".to_owned(),
            eframe::egui::FontData::from_static(include_bytes!("simsun.ttc")).into(),
        );
        fonts
            .families
            .entry(eframe::egui::FontFamily::Proportional)
            .and_modify(|v| v.insert(0, "simsun".to_owned()));
        fonts
            .families
            .entry(eframe::egui::FontFamily::Monospace)
            .and_modify(|v| v.insert(0, "simsun".to_owned()));
        cc.egui_ctx.set_fonts(fonts);
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("加密会把画面大小变为原来的3x3=9倍，故建议加密之前先把视频压到360p！");
            ui.horizontal(|ui| {
                ui.label("密码：");
                ui.text_edit_singleline(&mut self.seed);
            });
            ui.horizontal(|ui| {
                ui.label("网址：");
                ui.text_edit_singleline(&mut self.url);
                if ui.button("解析").clicked() {
                    let client = reqwest::blocking::Client::new();
                    let response = client
                        .post("https://sve-api.itcox.cn/public/parseVideo")
                        .header("Content-Type", "application/json")
                        .body(serde_json::json!({"url": self.url}).to_string())
                        .send()
                        .expect("failed to send request");
                    let text = response.text().expect("failed to read response");
                    let json: serde_json::Value =
                        serde_json::from_str(&text).expect("failed to parse json");
                    self.input = json["data"]["videoUrl"].as_str().expect("failed to get videoUrl").to_string();
                }
            });
            ui.horizontal(|ui| {
                ui.label("输入：");
                ui.text_edit_singleline(&mut self.input);
                if ui.button("加密播放").clicked() {
                    jiamibofang(&self.input, &self.seed);
                }
                if ui.button("解密播放").clicked() {
                    jiemibofang(&self.input, &self.seed);
                }
            });
            ui.horizontal(|ui| {
                ui.label("输出：");
                ui.text_edit_singleline(&mut self.output);
                if ui.button("加密保存").clicked() {
                    jiamibaocun(&self.input, &self.output, &self.seed);
                }
                if ui.button("解密保存").clicked() {
                    jiemibaocun(&self.input, &self.output, &self.seed);
                }
            })
        });
    }
}

fn main() {
    eframe::run_native(
        "视频臊子2",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
    .expect("failed to run eframe");
}
