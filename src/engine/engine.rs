use crate::utils::helper::{register_console, Logger};
use boa_engine::{Context, JsResult, Source};
use std::fs;
use std::sync::Arc;
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
    utils::utils_v12::on_initialize();
    println!("[V12]: Engine running with state: {}", self.state);
  }

  pub fn begin(&self, script_path: &str) -> JsResult<()> {
    let script: String = fs::read_to_string(script_path)
      .expect(&format!("[V12]: Unable to read file: {}", script_path));
    let mut context: Context = Context::default();

    register_console(&mut context, Arc::new(Logger::new()));
    context.eval(Source::from_bytes(&script))?;
    Ok(())
  }
}