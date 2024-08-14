use boa_engine::{object::FunctionBinding, object::ObjectInitializer, property::PropertyKey, Context, JsObject, JsString, JsValue, NativeFunction};

const COLORS: [&str; 10] = [
  "\x1b[0m",  // reset
  "\x1b[31m", // red
  "\x1b[33m", // yellow
  "\x1b[34m", // blue
  "\x1b[32m", // green
  "\x1b[36m", // cyan
  "\x1b[35m", // magenta
  "\x1b[37m", // white
  "\x1b[30m", // black
  "\x1b[39m", // normal
];

const STYLES: [&str; 4] = [
  "\x1b[1m",  // bold
  "\x1b[4m",  // underline
  "\x1b[7m",  // reverse
  "\x1b[22m", // normal
];

pub fn log(message: &str) {
  println!("[V12]: {}", message);
}

pub fn register_console(context: &mut Context) {
  let log_function = move |_arg0: &JsValue, arg1: &[JsValue], _arg2: &mut Context| {
    if let Some(arg) = arg1.first() {
      let message: JsString = arg.to_string(&mut Default::default()).unwrap_or_default();
      std::print!("{:?}\n", message);
    }
    Ok(JsValue::Undefined)
  };

  let log_function_native: NativeFunction = NativeFunction::from_copy_closure(log_function);

  let console: JsObject = ObjectInitializer::new(context)
    .function(
      log_function_native,
      FunctionBinding::from(JsString::from("log")),
      1,
    )
    .build();

  context
    .register_global_property(
      PropertyKey::from(JsString::from("console")),
      console,
      boa_engine::property::Attribute::all(),
    )
    .expect("[V12]: Fatal: Oh, 💩! Something went wrong!");
}

const BANNER: &str = r#"
____    ____  __   ___       _______ .__   __.   _______  __  .__   __.  _______
\   \  /   / /_ | |__ \     |   ____||  \ |  |  /  _____||  | |  \ |  | |   ____|
 \   \/   /   | |    ) |    |  |__   |   \|  | |  |  __  |  | |   \|  | |  |__
  \      /    | |   / /     |   __|  |  . `  | |  | |_ | |  | |  . `  | |   __|
   \    /     | |  / /_     |  |____ |  |\   | |  |__| | |  | |  |\   | |  |____
    \__/      |_| |____|    |_______||__| \__|  \______| |__| |__| \__| |_______|
"#;

fn n_line() {
  println!();
}

fn print_c(style: &str, color: &str, msg: &str) {
  print!("{}{}{}{}", style, color, msg, COLORS[0]);
}

pub(crate) fn about() {
  print_c(STYLES[0], COLORS[6], BANNER);
  n_line();
  print_c(STYLES[0], COLORS[6], "About the V12 Engine/Interpreter:\n");
  print_c(STYLES[0], COLORS[6], "Version: 1.0.0\n");
  print_c(STYLES[0], COLORS[6], "Author: Thoq\n");
  print_c(STYLES[0], COLORS[6], "License: MIT\n");
}
