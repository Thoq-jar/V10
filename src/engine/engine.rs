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
        let script: String = fs::read_to_string(script_path)
            .expect(&format!("[V12]: Unable to read file: {}", script_path));
        let mut context: Context = Context::default();

        // Load in custom functions
        require(&mut context);
        console(&mut context);
        stdout(&mut context);

        context.eval(Source::from_bytes(&script))?;
        if *DEBUG.lock().unwrap() {
            Ok(on_de_initialize())
        } else {
            Ok(())
        }
    }
}

pub fn console(context: &mut Context) {
    let log_function: NativeFunction = NativeFunction::from_fn_ptr(move |_arg0: &JsValue, arg1: &[JsValue], _arg2: &mut Context| {
        if let Some(arg) = arg1.first() {
            if let Ok(message) = arg.to_string(_arg2) {
                if let Ok(msg_str) = message.to_std_string() {
                    println!("{}", msg_str);
                }
            }
        }
        Ok(JsValue::Undefined)
    });

    let log_function_native = FunctionObjectBuilder::new(context.realm(), log_function)
        .name("log")
        .length(1)
        .build();

    let console: JsObject = ObjectInitializer::new(context)
        .property(
            PropertyKey::from(JsString::from("log")),
            log_function_native,
            boa_engine::property::Attribute::all(),
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

pub fn require(context: &mut Context) {
    let require_function: NativeFunction = NativeFunction::from_fn_ptr(move |_arg0: &JsValue, arg1: &[JsValue], _arg2: &mut Context| {
        if let Some(arg) = arg1.first() {
            if let Ok(module_path) = arg.to_string(_arg2) {
                if let Ok(module_path_str) = module_path.to_std_string() {
                    let module_script = fs::read_to_string(module_path_str)
                        .expect("Unable to read module file");
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

    let require: JsObject = ObjectInitializer::new(context)
        .property(
            PropertyKey::from(JsString::from("require")),
            require_function_native,
            boa_engine::property::Attribute::all(),
        )
        .build();

    context
        .register_global_property(
            PropertyKey::from(JsString::from("require")),
            require,
            boa_engine::property::Attribute::all(),
        )
        .expect("[V12]: Fatal: Oh, 💩! Something went wrong!");
}

pub fn stdout(context: &mut Context) {
    let debug_function: NativeFunction = NativeFunction::from_fn_ptr(move |_arg0: &JsValue, arg1: &[JsValue], _arg2: &mut Context| {
        if let Some(arg) = arg1.first() {
            if let Ok(message) = arg.to_string(_arg2) {
                if let Ok(msg_str) = message.to_std_string() {
                    println!("[V12 Debug]: {}", msg_str);
                }
            }
        }
        Ok(JsValue::Undefined)
    });

    let info_function: NativeFunction = NativeFunction::from_fn_ptr(move |_arg0: &JsValue, arg1: &[JsValue], _arg2: &mut Context| {
        if let Some(arg) = arg1.first() {
            if let Ok(out) = arg.to_string(_arg2) {
                if let Ok(msg_str) = out.to_std_string() {
                    println!("[V12 Info]: {}", msg_str);
                }
            }
        }
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

    let stdout: JsObject = ObjectInitializer::new(context)
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
      .build();

    context
      .register_global_property(
          PropertyKey::from(JsString::from("stdout")),
          stdout,
          boa_engine::property::Attribute::all(),
      )
      .expect("[V12]: Fatal: Oh, 💩! Something went wrong!");
}