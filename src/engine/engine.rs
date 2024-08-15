use crate::utils::helper::{register_console};
use boa_engine::{Context, JsResult, Source};
use std::fs;
use crate::*;

pub struct Engine {
  pub(crate) state: String,
}

impl Engine {
  pub fn new() -> Self {
    Engine {
      state: String::from("initialized"),
    }
  }

  pub fn run(&self) {
    if *DEBUG.lock().unwrap() {
      utils::utils_v12::on_initialize();
      println!("[V12]: Engine running with state: {}", self.state);
    }
  }

  pub fn begin(&self, script_path: &str) -> JsResult<()> {
    let script: String = fs::read_to_string(script_path)
      .expect(&format!("[V12]: Unable to read file: {}", script_path));
    let mut context: Context = Context::default();

    register_console(&mut context);
    context.eval(Source::from_bytes(&script))?;
    if *DEBUG.lock().unwrap() {
      Ok(on_de_initialize())
    } else {
      Ok(())
    }
  }
}