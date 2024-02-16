#[macro_use]
extern crate log;
extern crate android_logger;

use android_logger::Config;
use backtrace::Backtrace;
use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::JNIEnv;
use linjector_rs::inject_code_to_pid;
use log::LevelFilter;
use std::panic;

fn set_panic_handler() {
    panic::set_hook(Box::new(|panic_info| {
        // Call the custom function to handle the panic
        handle_panic(panic_info);
    }));
}

fn handle_panic(panic_info: &panic::PanicInfo) {
    // You can extract and format the panic information here
    let panic_message = match panic_info.payload().downcast_ref::<&str>() {
        Some(s) => *s,
        None => "Panic occurred but no message available",
    };

    debug!("Custom Panic Handler: {}", panic_message);

    // You can also get the location of the panic if available
    if let Some(location) = panic_info.location() {
        debug!(
            "Panic occurred in file '{}' at line {}",
            location.file(),
            location.line()
        );
    }

    // print the stack
    let backtrace = Backtrace::new();
    debug!("Backtrace:\n{:?}", backtrace);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_github_erfur_lasso_InjectorService_initLasso<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
) {
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Debug));
    debug!("init logger");

    set_panic_handler();
    debug!("init panic");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_github_erfur_lasso_InjectorService_injectCode<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    pid: jint,
    file_path: JString<'local>,
) {
    let pid: i32 = pid as i32;
    debug!("pid: {}", pid);

    let file_path_str: String = _env.get_string(&file_path).unwrap().into();
    debug!("file_path: {}", file_path_str);

    inject_code_to_pid(pid, file_path_str);
}
