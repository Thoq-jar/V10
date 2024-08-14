use std::fs;

pub fn strip_types(ts_file_path: &str) -> String {
    let ts_content = fs::read_to_string(ts_file_path).expect("[V12]: Unable to read TypeScript file");
    let re = regex::Regex::new(r":\s*\w+(\[])?|<\w+>").unwrap();
    let js_content = re.replace_all(&ts_content, "").to_string();
    let temp_file_path = format!("{}.js", ts_file_path);
    fs::write(&temp_file_path, js_content).expect("[V12]: Unable to write temporary JavaScript file");
    temp_file_path
}

pub fn run_temp_file(temp_file_path: &str) {
    let script = fs::read_to_string(temp_file_path).expect("[V12]: Unable to read temporary JavaScript file");
    let mut context = boa::Context::new();

    let console = boa::object::ObjectInitializer::new(&mut context)
        .function(|_, args, _| {
            if let Some(arg) = args.get(0) {
                println!("{}", arg.to_string(&mut Default::default()).unwrap_or_default());
            }
            Ok(boa::JsValue::Undefined)
        }, "log", 1)
        .build();

    context.register_global_property("console", console, boa::property::Attribute::all());

    context.eval(&script).expect("[V12]: Failed to execute temporary JavaScript file");
}

pub fn process_typescript_file(ts_file_path: &str) {
    let temp_file_path = strip_types(ts_file_path);
    run_temp_file(&temp_file_path);
    fs::remove_file(temp_file_path).expect("[V12]: Unable to delete temporary JavaScript file");
}