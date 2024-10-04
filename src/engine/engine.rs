use boa_engine::{object::FunctionObjectBuilder, object::ObjectInitializer, property::PropertyKey, Context, JsObject, JsString, JsValue, JsResult, Source, native_function::NativeFunction};
use std::fs;
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
    if *DEBUG.lock().unwrap() {
      utils::helper::on_initialize();
      println!("[V12]: Engine running with state: {}", self.state);
    }
  }

  pub fn begin(&self, script_path: &str) -> JsResult<()> {
    let script: String = fs::read_to_string(script_path).expect(&format!("[V12]: Unable to read file: {}", script_path));
    let mut context: Context = Context::default();

    self.register_global_function(&mut context, "console", console_function);
    self.register_global_function(&mut context, "require", require_function);
    self.register_global_function(&mut context, "stdout", stdout_functions);

    context.eval(Source::from_bytes(&script))?;
    if *DEBUG.lock().unwrap() {
      Ok(on_de_initialize())
    } else {
      Ok(())
    }
  }

  fn register_global_function<F>(&self, context: &mut Context, name: &str, func: F)
  where
    F: Fn(&mut Context) -> JsObject,
  {
    let global_object = func(context);
    context
      .register_global_property(
        PropertyKey::from(JsString::from(name)),
        global_object,
        boa_engine::property::Attribute::all(),
      )
      .expect("[V12]: Fatal: Oh, 💩! Something went wrong!");
  }
}

fn console_function(context: &mut Context) -> JsObject {
  let log_function: NativeFunction = NativeFunction::from_fn_ptr(move |_arg0: &JsValue, arg1: &[JsValue], _arg2: &mut Context| {
    log_message(arg1, _arg2);
    Ok(JsValue::Undefined)
  });

  let log_function_native = FunctionObjectBuilder::new(context.realm(), log_function)
    .name("log")
    .length(1)
    .build();

  ObjectInitializer::new(context)
    .property(
      PropertyKey::from(JsString::from("log")),
      log_function_native,
      boa_engine::property::Attribute::all(),
    )
    .build()
}

fn require_function(context: &mut Context) -> JsObject {
  let require_function: NativeFunction = NativeFunction::from_fn_ptr(move |_arg0: &JsValue, arg1: &[JsValue], _arg2: &mut Context| {
    if let Some(arg) = arg1.first() {
      if let Ok(module_path) = arg.to_string(_arg2) {
        if let Ok(module_path_str) = module_path.to_std_string() {
          let module_script = fs::read_to_string(module_path_str).expect("Unable to read module file");
          let module = _arg2.eval(Source::from_bytes(&module_script)).expect("Failed to evaluate module");
          if let Some(exports) = module.as_object() {
            return Ok(exports.get(PropertyKey::from(JsString::from("test1")), _arg2).unwrap_or(JsValue::Undefined));
          }
        }
      }
    }
    Ok(JsValue::Undefined)
  });

  let require_function_native = FunctionObjectBuilder::new(context.realm(), require_function)
    .name("require")
    .length(1)
    .build();

  ObjectInitializer::new(context)
    .property(
      PropertyKey::from(JsString::from("require")),
      require_function_native,
      boa_engine::property::Attribute::all(),
    )
    .build()
}

fn stdout_functions(context: &mut Context) -> JsObject {
  let debug_function: NativeFunction = NativeFunction::from_fn_ptr(move |_arg0: &JsValue, arg1: &[JsValue], _arg2: &mut Context| {
    log_message(arg1, _arg2);
    Ok(JsValue::Undefined)
  });

  let info_function: NativeFunction = NativeFunction::from_fn_ptr(move |_arg0: &JsValue, arg1: &[JsValue], _arg2: &mut Context| {
    log_info(arg1, _arg2);
    Ok(JsValue::Undefined)
  });

  let debug_function_native = FunctionObjectBuilder::new(context.realm(), debug_function)
    .name("debug")
    .length(1)
    .build();

  let info_function_native = FunctionObjectBuilder::new(context.realm(), info_function)
    .name("info")
    .length(1)
    .build();

  ObjectInitializer::new(context)
    .property(
      PropertyKey::from(JsString::from("debug")),
      debug_function_native,
      boa_engine::property::Attribute::all(),
    )
    .property(
      PropertyKey::from(JsString::from("info")),
      info_function_native,
      boa_engine::property::Attribute::all(),
    )
    .build()
}

fn log_message(arg1: &[JsValue], context: &mut Context) {
  if let Some(arg) = arg1.first() {
    if let Ok(message) = arg.to_string(context) {
      if let Ok(msg_str) = message.to_std_string() {
        if *DEBUG.lock().unwrap() {
          if let Ok(num) = msg_str.parse::<f64>() {
            println!("\x1b[33m{}\x1b[0m", num);
          } else {
            println!("{}", msg_str);
          }
        } else {
          if let Ok(num) = msg_str.parse::<f64>() {
            println!("\x1b[33m{}\x1b[0m", num);
          } else {
            println!("{}", msg_str);
          }
        }
      }
    }
  }
}

fn log_info(arg1: &[JsValue], context: &mut Context) {
  if let Some(arg) = arg1.first() {
    if let Ok(out) = arg.to_string(context) {
      if let Ok(msg_str) = out.to_std_string() {
        println!("[V12 Info]: {}", msg_str);
      }
    }
  }
}
