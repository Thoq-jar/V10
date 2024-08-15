pub(crate) fn about_v12() {
  let banner = r#"
____    ____  __   ___       _______ .__   __.   _______  __  .__   __.  _______
\   \  /   / /_ | |__ \     |   ____||  \ |  |  /  _____||  | |  \ |  | |   ____|
 \   \/   /   | |    ) |    |  |__   |   \|  | |  |  __  |  | |   \|  | |  |__
  \      /    | |   / /     |   __|  |  . `  | |  | |_ | |  | |  . `  | |   __|
   \    /     | |  / /_     |  |____ |  |\   | |  |__| | |  | |  |\   | |  |____
    \__/      |_| |____|    |_______||__| \__|  \______| |__| |__| \__| |_______|
"#;
  print_c(colors::MAGENTA, styles::BOLD, banner);
  n_line();
  print_c(colors::MAGENTA, styles::BOLD, "About V12 - A simple, lightweight, and easy-to-use JavaScript Interpreter/Engine written in Rust.\n");
  print_c(colors::MAGENTA, styles::BOLD, "Version: 0.4.0\n");
  print_c(colors::MAGENTA, styles::BOLD, "Author(s): Thoq\n");
  print_c(colors::MAGENTA, styles::BOLD, "License: MIT\n");
  print_c(colors::MAGENTA, styles::BOLD, "Build: 81424 (development)\n");
}

mod colors {
  pub const RESET: &str = "\x1b[0m";
  pub const MAGENTA: &str = "\x1b[35m";
}

mod styles {
  pub const BOLD: &str = "\x1b[1m";
}

fn n_line() {
  println!();
}

fn print_c(color: &str, style: &str, msg: &str) {
  print!("{}{}{}{}", style, color, msg, colors::RESET);
}
