mod engine;
mod utils;

use std::env;
use crate::engine::engine::Engine;
use crate::utils::helper::{log, register_console};
use crate::utils::typescript::process_typescript_file;
use boa_engine::{Context, Source, JsResult};

fn main() -> JsResult<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("[V12]: Usage: {} <script_path> or version", args[0]);
    return Ok(());
  }

  let arg1 = &args[1];

  match arg1.as_str() {
    arg if arg.ends_with(".ts") => {
      process_typescript_file(arg);
    }
    arg if arg.ends_with(".js") => {
      let engine = Engine::new();
      engine.run();
      log("Engine has started successfully.");
      engine.interpret_js(arg)?;
    }
    "version" => {
      about()?;
    }
    _ => {
      eprintln!("[V12]: Error: Script file must have a .js or .ts extension");
    }
  }

  Ok(())
}

fn about() -> JsResult<()> {
  let script_path = "src/js/V12.js";
  let script = std::fs::read_to_string(script_path).expect("[V12]: Internal error error in engine!");
  let mut context = Context::default();

  register_console(&mut context);
  context.eval(Source::from_bytes(&script))?;
  context.eval(Source::from_bytes("about();"))?;

  Ok(())
}