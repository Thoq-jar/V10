use super::logutil::{
  colors,
  styles,
  n_line,
  print_c
};

pub(crate) fn about_v12() {
  let banner = r#"
____    ____  __   ___      .______       __    __  .__   __. .___________. __  .___  ___.  _______
\   \  /   / /_ | |__ \     |   _  \     |  |  |  | |  \ |  | |           ||  | |   \/   | |   ____|
 \   \/   /   | |    ) |    |  |_)  |    |  |  |  | |   \|  | `---|  |----`|  | |  \  /  | |  |__
  \      /    | |   / /     |      /     |  |  |  | |  . `  |     |  |     |  | |  |\/|  | |   __|
   \    /     | |  / /_     |  |\  \----.|  `--'  | |  |\   |     |  |     |  | |  |  |  | |  |____
    \__/      |_| |____|    | _| `._____| \______/  |__| \__|     |__|     |__| |__|  |__| |_______|
"#;
  print_c(colors::MAGENTA, styles::BOLD, banner);
  n_line();
  print_c(colors::MAGENTA, styles::BOLD, " About V12 - A simple, lightweight, and easy-to-use JavaScript Runtime written in Rust.\n");
  print_c(colors::MAGENTA, styles::BOLD, " Version: 0.5.2\n");
  print_c(colors::MAGENTA, styles::BOLD, " Author(s): Thoq\n");
  print_c(colors::MAGENTA, styles::BOLD, " License: MIT\n");
  print_c(colors::MAGENTA, styles::BOLD, " Build: 10424 (development)\n");
  n_line();
  print_c(colors::MAGENTA, styles::BOLD, " HELP:\n");
  print_c(colors::MAGENTA, styles::BOLD, "   Usage: V12 <script_path>.ts/.js, version [debug]\n");
}
