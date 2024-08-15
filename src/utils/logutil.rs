pub fn log(message: &str) {
  println!("[V12]: {}", message);
}

pub mod colors {
  pub const RESET: &str = "\x1b[0m";
  pub const MAGENTA: &str = "\x1b[35m";
  pub const RED: &str = "\x1b[31m";
}

pub mod styles {
  pub const BOLD: &str = "\x1b[1m";
  // pub const NORMAL: &str = "\x1b[0m";
}

pub fn n_line() {
  println!();
}

pub fn print_c(color: &str, style: &str, msg: &str) {
  print!("{}{}{}{}", style, color, msg, colors::RESET);
}
