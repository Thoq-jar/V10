mod engine;
mod utils;
mod tests;

use std::env;
use crate::engine::engine::Engine;
use crate::utils::helper::log;
use crate::utils::typescript::process_typescript_file;
use boa_engine::JsResult;
use crate::utils::utils_v12::on_de_initialize;

fn main() -> JsResult<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("[V12]: Usage: {} <script_path> or version", args[0]);
    return Ok(());
  }

  let arg1: &String = &args[1];

  match arg1.as_str() {
    arg if arg.ends_with(".ts") => {
      process_typescript_file(arg);
    }
    arg if arg.ends_with(".js") => {
      let engine: Engine = Engine::new();
      engine.run();
      log("Engine has started successfully.\n");
      engine.begin(arg)?;
      utils::utils_v12::on_de_initialize();
    }
    "version" => {
      show_about()?;
    }
    _ => {
      eprintln!("[V12]: Error: Script file must have a .js or .ts extension");
    }
  }

  Ok(())
}

fn show_about() -> JsResult<()> {
  let engine = Engine::new();
  engine.run();
  utils::about_v12::about_v12();
  on_de_initialize();
  Ok(())
}