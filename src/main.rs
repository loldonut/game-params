use clap::{Parser};
use std::{collections::HashMap, fs};
use serde::Deserialize;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    file: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    flags: Option<Vec<String>>,
    env: HashMap<String, String>,
    gamescope: Gamescope,
}

#[derive(Debug, Deserialize)]
struct Gamescope {
    width: u32,
    height: u32,
    scaler: Option<String>,
    fullscreen: Option<bool>,
    flags: Option<Vec<String>>
}

fn parse_gs_params(gs: &Gamescope) -> String {
    let mut params: Vec<String> = vec!["LD_PRELOAD= gamescope".to_string()];

    let video_size = format!("-w {} -h {}", gs.width, gs.height);
    let env_preload = "-- env LD_PRELOAD=\"$LD_PRELOAD\"".to_string();

    params.push(video_size);

    if gs.scaler.is_some() {
        let scaler = format!("-S {}", gs.scaler.as_ref().unwrap());
        params.push(scaler);
    }

    if gs.flags.is_some() {
        let flags = gs.flags.as_ref().unwrap();
        params.push(flags.join(" "));
    }

    if gs.fullscreen.is_some() {
        params.push("-f".to_string());
    }

    params.push(env_preload);

    params.join(" ")
}

fn parse_params(env: &HashMap<String, String>, gs: &Gamescope, flags: &Option<Vec<String>>) -> String {
    let mut params: Vec<String> = vec![];
    let mut env_params: Vec<String> = vec![];
    let gs_params = parse_gs_params(&gs);

    for (k, v) in env {
        env_params.push(format!("{}={}", k, v));
    }

    params.push(env_params.join(" "));
    params.push(gs_params);
    params.push("%command%".to_string());

    if flags.is_some() {
        params.push(flags.as_ref().unwrap().join(" "));
    }

    params.join(" ")
}

fn main() {
    let cli = Cli::parse();

    let file_path = cli.file;

    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    let config: Config = toml::from_str(&contents).expect("Failed to parse file");
    let params = parse_params(&config.env, &config.gamescope, &config.flags);

    println!("{}", params);
}

