const banner = `
██╗   ██╗ ██╗██████╗ 
██║   ██║███║╚════██╗
██║   ██║╚██║ █████╔╝
╚██╗ ██╔╝ ██║██╔═══╝ 
 ╚████╔╝  ██║███████╗
  ╚═══╝   ╚═╝╚══════╝
`

function about() {
  printc(styles.bold, colors.magenta, banner);
  nline();
  printc(styles.bold, colors.magenta, 'About the V12 Engine:');
  printc(styles.bold, colors.magenta, 'Version: 1.0.0');
  printc(styles.bold, colors.magenta, 'Author: Thoq');
  printc(styles.bold, colors.magenta, 'License: MIT');
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

function nline() {
  console.log("")
}

function printc(style, color, msg) {
  console.log(`${style}${color}${msg}${colors.reset}`);
}