use crate::utils::logutil::*;

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
  print_c(colors::MAGENTA, styles::BOLD, " About V12 - A simple, lightweight, and easy-to-use JavaScript Interpreter/Engine written in Rust.\n");
  print_c(colors::MAGENTA, styles::BOLD, " Version: 0.5.0\n");
  print_c(colors::MAGENTA, styles::BOLD, " Author(s): Thoq\n");
  print_c(colors::MAGENTA, styles::BOLD, " License: MIT\n");
  print_c(colors::MAGENTA, styles::BOLD, " Build: 81424 (development)\n");
  n_line();
  print_c(colors::MAGENTA, styles::BOLD, " HELP:\n");
  print_c(colors::MAGENTA, styles::BOLD, "   Usage: V12 <script_path>.ts/.js, version [debug]\n");
}
