use std::process::Command;
use clap::Parser;

fn apply_pipewire_settings(setting: &str, value: &str){
    let mut command = Command::new("pw-metadata");
    command.arg("-n")
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

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    sample_rate: Option<u32>,
    #[arg(short, long)]
    block_size: Option<u32>,
}

fn main() {
    let args = Cli::parse();
    args.sample_rate.and_then(change_sample_rate);
    args.block_size.and_then(change_block_size);
}
