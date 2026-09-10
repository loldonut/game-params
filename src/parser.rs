use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub flags: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
    pub gamescope: Gamescope,
    #[serde(rename = "dll-overrides")]
    pub dll_overrides: Option<HashMap<String, String>>
}

#[derive(Debug, Deserialize)]
pub struct Gamescope {
    width: u32,
    height: u32,
    scaler: Option<String>,
    fullscreen: Option<bool>,
    flags: Option<Vec<String>>,
}

fn parse_env(hm: &HashMap<String, String>) -> Vec<String> {
    let mut res = vec![];

    for (k, v) in hm {
        res.push(format!("{}={}", k, v));
    }

    res
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

pub fn parse_params(config: &Config) -> String {
    let env: &Option<HashMap<String, String>> = &config.env;
    let gs: &Gamescope = &config.gamescope;
    let flags: &Option<Vec<String>> = &config.flags;
    let dll_overrides: &Option<HashMap<String, String>> = &config.dll_overrides;

    let mut params: Vec<String> = vec![];
    let gs_params = parse_gs_params(gs);

    if !dll_overrides.as_ref().unwrap().is_empty() {
        let overrides = parse_env(dll_overrides.as_ref().unwrap());
        params.push(format!("WINEDLLOVERRIDES=\"{}\"", overrides.join(";")));
    }

    if !env.as_ref().unwrap().is_empty() {
        let e = parse_env(env.as_ref().unwrap());
        params.push(e.join(" "));
    }

    params.push(gs_params);
    params.push("%command%".to_string());

    if flags.is_some() {
        params.push(flags.as_ref().unwrap().join(" "));
    }

    params.join(" ")
}
