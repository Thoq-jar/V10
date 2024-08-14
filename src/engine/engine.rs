use std::fs;
use boa::{Context, JsValue};
use boa::object::ObjectInitializer;

pub struct Engine {
    state: String,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            state: String::from("initialized"),
        }
    }

    pub fn run(&self) {
        println!("[V12]: Engine running with state: {}", self.state);
    }

    pub fn interpret_js(&self, script_path: &str) {
        let script = fs::read_to_string(script_path).expect("[V12]: Unable to read file");
        let mut context = Context::new();

        let console = ObjectInitializer::new(&mut context)
            .function(|_, args, _| {
                if let Some(arg) = args.get(0) {
                    println!("{}", arg.to_string(&mut Default::default()).unwrap_or_default());
                }
                Ok(JsValue::Undefined)
            }, "log", 1)
            .build();

        context.register_global_property("console", console, boa::property::Attribute::all());

        context.eval(&script).expect("[V12]: Failed to execute script");
    }
}
