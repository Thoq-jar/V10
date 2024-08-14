export function aboutV12() {
  const banner = `
____    ____  __   ___       _______ .__   __.   _______  __  .__   __.  _______
\\   \\  /   / /_ | |__ \\     |   ____||  \\ |  |  /  _____||  | |  \\ |  | |   ____|
 \\   \\/   /   | |    ) |    |  |__   |   \\|  | |  |  __  |  | |   \\|  | |  |__
  \\      /    | |   / /     |   __|  |  . \`  | |  | |_ | |  | |  . \`  | |   __|
   \\    /     | |  / /_     |  |____ |  |\\   | |  |__| | |  | |  |\\   | |  |____
    \\__/      |_| |____|    |_______||__| \\__|  \\______| |__| |__| \\__| |_______|
`;
  print_c(colors.magenta, styles.bold, banner);
  n_line();
  print_c(colors.magenta, styles.bold, "About V12 - A simple, lightweight, and easy-to-use JavaScript Interpreter/Engine written in Rust.");
  print_c(colors.magenta, styles.bold, "Version: 1.0.0");
  print_c(colors.magenta, styles.bold, "Author(s): Thoq");
  print_c(colors.magenta, styles.bold, "License: MIT");
  print_c(colors.magenta, styles.bold, "Build: 81424 (development)");
}

const colors = {
  reset: "\x1b[0m",
  red: "\x1b[31m",
  yellow: "\x1b[33m",
  blue: "\x1b[34m",
  green: "\x1b[32m",
  cyan: "\x1b[36m",
  magenta: "\x1b[35m",
  white: "\x1b[37m",
  black: "\x1b[30m",
  normal: "\x1b[39m",
};

const styles = {
  bold: "\x1b[1m",
  underline: "\x1b[4m",
  reverse: "\x1b[7m",
  normal: "\x1b[22m",
};

function n_line() {
  console.log("");
}

function print_c(style, color, msg) {
  console.log(`${style}${color}${msg}${colors.reset}`);
}

aboutV12();