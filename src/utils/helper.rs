use boa_engine::{object::FunctionBinding, object::ObjectInitializer, property::PropertyKey, Context, JsObject, JsString, JsValue, NativeFunction};

pub fn log(message: &str) {
  println!("[V12]: {}", message);
}

pub fn register_console(context: &mut Context) {
  let log_function = move |_arg0: &JsValue, arg1: &[JsValue], _arg2: &mut Context| {
    if let Some(arg) = arg1.first() {
      let message: JsString = arg.to_string(&mut Default::default()).unwrap_or_default();
      let message_str = message.to_std_string().unwrap_or_default();
      println!("{}", message_str);
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
