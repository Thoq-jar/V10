mod engine;
mod utils;
mod tests;

use std::env;
use crate::engine::engine::Engine;
use crate::utils::helper::log;
use crate::utils::helper::about;
use crate::utils::typescript::process_typescript_file;
use boa_engine::JsResult;

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
      log("Engine has started successfully.");
      engine.interpret_js(arg)?;
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
  about();
  Ok(())
}
