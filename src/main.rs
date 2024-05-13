use eframe::egui;
use std::process::Command;

fn apply_pipewire_settings(setting: &str, value: &str) {
    let mut command = Command::new("pw-metadata");
    command
        .arg("-n")
        .arg("settings")
        .arg("0")
        .arg(setting)
        .arg(value)
        .output()
        .expect("failed to execute process");
}

fn change_sample_rate(sample_rate: u32) -> Option<()> {
    let as_str = sample_rate.to_string();
    apply_pipewire_settings(&"clock.force-rate", &as_str);
    Some(())
}

fn change_block_size(block_size: u32) -> Option<()> {
    let as_str = block_size.to_string();
    apply_pipewire_settings(&"clock.force-quantum", &as_str);
    Some(())
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Pipewire Configurator",
        options,
        Box::new(|_| Box::<App>::default()),
    )
}

struct App {
    sample_rate: u32,
    block_size: u32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            sample_rate: 0,
            block_size: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Sample Rate")
                .selected_text(self.sample_rate.to_string())
                .show_ui(ui, |ui| {
                    let current = self.sample_rate;
                    for i in [44100, 48000, 88200, 96000, 176400, 192000] {
                        ui.selectable_value(&mut self.sample_rate, i, i.to_string());
                    }
                    if self.sample_rate != current {
                        change_sample_rate(self.sample_rate);
                        println!("changed sample rate {}", self.sample_rate);
                    }
                });
            egui::ComboBox::from_label("Block Size")
                .selected_text(self.block_size.to_string())
                .show_ui(ui, |ui| {
                    let current = self.block_size;
                    for i in [32, 64, 128, 256, 512, 1024, 2048] {
                        ui.selectable_value(&mut self.block_size, i, i.to_string());
                    }
                    if self.block_size != current {
                        change_block_size(self.block_size);
                        println!("changed block size: {}", self.block_size);
                    }
                });
        });
    }
}
