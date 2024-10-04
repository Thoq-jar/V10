use std::{
  env,
  path::PathBuf,
};

pub fn on_initialize() {
  println!("[V12]: Initializing V12 modules...");
}

pub fn on_de_initialize() {
  println!("\n[V12]: V12 shut down successfully!");
}

pub fn path(ts_file_path: &str) -> PathBuf {
  let current_dir: PathBuf = env::current_dir().expect("[V12]: Unable to get current directory");
  current_dir.join(ts_file_path)
}