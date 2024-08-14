use boa_engine::{Context, JsValue, JsString, NativeFunction, object::ObjectInitializer, object::FunctionBinding, property::PropertyKey};

const PREFIX: &str = "[V12]: ";

const COLORS: [&str; 10] = [
  "\x1b[0m",   // reset
  "\x1b[31m",  // red
  "\x1b[33m",  // yellow
  "\x1b[34m",  // blue
  "\x1b[32m",  // green
  "\x1b[36m",  // cyan
  "\x1b[35m",  // magenta
  "\x1b[37m",  // white
  "\x1b[30m",  // black
  "\x1b[39m",  // normal
];

const STYLES: [&str; 4] = [
  "\x1b[1m",   // bold
  "\x1b[4m",   // underline
  "\x1b[7m",   // reverse
  "\x1b[22m",  // normal
];

pub fn log(message: &str) {
  println!("{}: {}", PREFIX, message);
}

pub fn register_console(context: &mut Context) {
  let log_function = move |_arg0: &JsValue, arg1: &[JsValue], _arg2: &mut Context| {
    if let Some(arg) = arg1.first() {
      let message = arg.to_string(&mut Default::default()).unwrap_or_default();
      println!("{:?}", message);
    }
    Ok(JsValue::Undefined)
  };

  let log_function_native = NativeFunction::from_copy_closure(log_function);

  let console = ObjectInitializer::new(context)
    .function(log_function_native, FunctionBinding::from(JsString::from("log")), 1)
    .build();

  context.register_global_property(PropertyKey::from(JsString::from("console")), console, boa_engine::property::Attribute::all())
    .expect("[V12]: Fatal: Oh, 💩! Something went wrong!");
}

const BANNER: &str = "
██╗   ██╗ ██╗██████╗
██║   ██║███║╚════██╗
██║   ██║╚██║ █████╔╝
╚██╗ ██╔╝ ██║██╔═══╝
 ╚████╔╝  ██║███████╗
  ╚═══╝   ╚═╝╚══════╝
";

fn nline() {
  println!();
}

fn printc(style: &str, color: &str, msg: &str) {
  print!("{}{}{}{}", style, color, msg, COLORS[0]);
}

pub(crate) fn about() {
  printc(STYLES[0], COLORS[6], BANNER);
  nline();
  printc(STYLES[0], COLORS[6], "About the V12 Engine:\n");
  printc(STYLES[0], COLORS[6], "Version: 1.0.0\n");
  printc(STYLES[0], COLORS[6], "Author: Thoq\n");
  printc(STYLES[0], COLORS[6], "License: MIT\n");
}
