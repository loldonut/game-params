use std::collections::HashMap;

use serde::Deserialize;

type VecString = Vec<String>;
type HashMapStrings = HashMap<String, String>;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub flags: Option<VecString>,
    pub env: Option<HashMapStrings>,
    pub gamescope: Option<Gamescope>,
    #[serde(rename = "dll-overrides")]
    pub dll_overrides: Option<HashMapStrings>
}

#[derive(Debug, Deserialize)]
pub struct Gamescope {
    width: Option<u32>,
    height: Option<u32>,
    scaler: Option<String>,
    fullscreen: Option<bool>,
    flags: Option<VecString>
}

fn parse_env(hm: &HashMap<String, String>) -> Vec<String> {
    let mut res = vec![];

    for (k, v) in hm {
        res.push(format!("{}={}", k, v));
    }

    res
}

fn parse_gs_params(gs: &Gamescope) -> String {
    let mut params: VecString = vec!["LD_PRELOAD= gamescope".to_string()];
    let env_preload = "-- env LD_PRELOAD=\"$LD_PRELOAD\"".to_string();

    if gs.width.is_some() && gs.height.is_some() {
        let width = gs.width.as_ref().unwrap();
        let height = gs.height.as_ref().unwrap();

        let video_size = format!("-w {} -h {}", width, height);
        params.push(video_size);
    }

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
    let env: &Option<HashMapStrings> = &config.env;
    let gs: &Option<Gamescope> = &config.gamescope;
    let flags: &Option<VecString> = &config.flags;
    let dll_overrides: &Option<HashMapStrings> = &config.dll_overrides;
    let mut add_command = false;

    let mut params: Vec<String> = vec![];

    if dll_overrides.is_some() && !dll_overrides.as_ref().unwrap().is_empty() {
        let overrides = parse_env(dll_overrides.as_ref().unwrap());
        params.push(format!("WINEDLLOVERRIDES=\"{}\"", overrides.join(";")));
    }

    if env.is_some() && !env.as_ref().unwrap().is_empty() {
        let e = parse_env(env.as_ref().unwrap());
        params.push(e.join(" "));
        add_command = true;
    }

    if gs.is_some() {
        let gs_params = parse_gs_params(gs.as_ref().unwrap());
        params.push(gs_params);
        add_command = true;
    }

    if add_command {
        params.push("%command%".to_string());
    }

    if flags.is_some() {
        params.push(flags.as_ref().unwrap().join(" "));
    }

    params.join(" ")
}
