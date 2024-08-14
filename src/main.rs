mod engine;
mod utils;

use std::env;
use crate::engine::engine::Engine;
use crate::utils::helper::log;
use crate::utils::typescript::process_typescript_file;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("[V12]: Usage: {} <script_path> or version", args[0]);
    return;
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
      engine.interpret_js(arg);
    }
    "version" => {
      about();
    }
    _ => {
      eprintln!("[V12]: Error: Script file must have a .js or .ts extension");
    }
  }
}

fn about() {
  let script_path = "src/js/V12.js";
  let script = std::fs::read_to_string(script_path).expect("[V12]: Internal error error in engine!");
  let mut context = boa::Context::new();

  let console = boa::object::ObjectInitializer::new(&mut context)
    .function(|_, args, _| {
      if let Some(arg) = args.get(0) {
        println!("{}", arg.to_string(&mut Default::default()).unwrap_or_default());
      }
      Ok(boa::JsValue::Undefined)
    }, "log", 1)
    .build();

  context.register_global_property("console", console, boa::property::Attribute::all());
  context.eval(&script).expect("[V12]: Failed to execute script");
  context.eval("about();").expect("[V12]: Failed to execute about function");
}