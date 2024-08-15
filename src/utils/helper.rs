use boa_engine::{object::FunctionBinding, object::ObjectInitializer, property::PropertyKey, Context, JsObject, JsString, JsValue, NativeFunction};
use rayon::ThreadPoolBuilder;
use std::sync::Arc;

pub struct Logger {
  pool: Arc<rayon::ThreadPool>,
}

impl Logger {
  pub fn new() -> Self {
    let num_cpus = num_cpus::get();
    let pool = Arc::new(ThreadPoolBuilder::new()
      .num_threads(num_cpus / 2)
      .build()
      .expect("Failed to create thread pool"));

    Logger { pool }
  }

  pub fn log_js(&self, message: String) {
    let pool = Arc::clone(&self.pool);
    pool.spawn(move || {
      println!("{}", message);
    });
  }
}

pub fn log(message: &str) {
  println!("[V12]: {}", message);
}

pub fn register_console(context: &mut Context, logger: Arc<Logger>) {
  let log_function = {
    let logger_clone = Arc::clone(&logger);

    move |_arg0: &JsValue, arg1: &[JsValue], _arg2: &mut Context| {
      if let Some(arg) = arg1.first() {
        if let Ok(message) = arg.to_string(_arg2) {
          if let Ok(msg_str) = message.to_std_string() {
            logger_clone.log_js(msg_str);
          }
        }
      }
      Ok(JsValue::Undefined)
    }
  };

  let log_function_native: NativeFunction = unsafe { NativeFunction::from_closure(log_function) };

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