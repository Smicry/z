use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn data_path() -> Option<String> {
    match env::var("_Z_DATA")
        .or_else(|_| env::var("HOME"))
        .map(|path| path + "/.z")
    {
        Ok(path) => Some(path),
        Err(_) => None,
    }
}

pub fn _z_dirs(path: &str) -> Vec<String> {
    let mut data: Vec<String> = Vec::new();
    let _ = File::open(path).map(|f| {
        for line in BufReader::new(f).lines() {
            let _ = line.map(|text| {
                if let Some(text) = text.split("|").next() {
                    if Path::new(text).is_dir() {
                        data.push(text.to_string());
                    }
                }
            });
        }
    });
    return data;
}
