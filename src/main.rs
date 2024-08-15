mod engine;
mod utils;
mod tests;

use std::env;
use crate::engine::engine::Engine;
use crate::utils::logutil::{colors, log, print_c, styles};
use crate::utils::typescript::process_typescript_file;
use boa_engine::JsResult;
use crate::utils::helper::on_de_initialize;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DEBUG: Mutex<bool> = Mutex::new(false);
}

fn main() -> JsResult<()> {
    let mut args: Vec<String> = env::args().collect();
    if let Some(last_arg) = args.last() {
        if last_arg == "debug" {
            let mut debug = DEBUG.lock().unwrap();
            *debug = true;
            args.pop();
        }
    }

    if args.len() < 2 {
        print_c(colors::RED, styles:: BOLD, "[V12]: Error: Usage: V12 <script_path>.ts/.js, version [debug]\n");
        return if *DEBUG.lock().unwrap() {
            Ok(on_de_initialize())
        } else {
            Ok(())
        }
    }

    let arg1: &String = &args[1];

    match arg1.as_str() {
        arg if arg.ends_with(".ts") => {
            process_typescript_file(arg);
        }
        arg if arg.ends_with(".js") => {
            let engine: Engine = Engine::new();
            engine.run();
            if *DEBUG.lock().unwrap() {
                log("Engine has started successfully.\n");
            }
            engine.begin(arg)?;
        }
        "version" => {
            show_about()?;
        }
        _ => {
            print_c(colors::RED, styles:: BOLD, "[V12]: Error: Usage: V12 <script_path>.ts/.js, version [debug]\n");
        }
    }

    Ok(())
}

fn show_about() -> JsResult<()> {
    let engine = Engine::new();
    engine.run();
    utils::about_v12::about_v12();
    if *DEBUG.lock().unwrap() {
        Ok(on_de_initialize())
    } else {
        Ok(())
    }
}