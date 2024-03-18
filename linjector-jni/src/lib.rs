#[macro_use]
extern crate log;
extern crate android_logger;

use android_logger::Config;
use backtrace::Backtrace;
use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::JNIEnv;
use log::LevelFilter;
use std::panic;

fn set_panic_handler() {
    panic::set_hook(Box::new(|panic_info| {
        handle_panic(panic_info);
    }));
}

fn handle_panic(panic_info: &panic::PanicInfo) {
    error!("Panic occurred: {}", panic_info.to_string());

    // You can also get the location of the panic if available
    if let Some(location) = panic_info.location() {
        error!(
            "Panic occurred in file '{}' at line {}",
            location.file(),
            location.line()
        );
    }

    if panic_info.payload().is::<String>() {
        let payload = panic_info.payload().downcast_ref::<String>().unwrap();
        error!("Panic payload: {}", payload);
    }

    // print the stack
    let backtrace = Backtrace::new();
    error!("Backtrace:\n{:?}", backtrace);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_github_erfur_lasso_InjectorService_initLasso<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
) {
    debug!("init logger");
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Debug));

    debug!("init panic");
    set_panic_handler();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_github_erfur_lasso_InjectorService_injectCode<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    pid: jint,
    file_path: JString<'local>,
) {
    debug!("pid: {}", pid);

    let file_path_str: String = _env.get_string(&file_path).unwrap().into();
    debug!("file_path: {}", file_path_str);

    linjector_rs::Injector::new(pid)
        .unwrap()
        .use_raw_dlopen()
        .unwrap()
        .set_file_path(file_path_str)
        .unwrap()
        .set_default_syms()
        .unwrap()
        .inject()
        .unwrap();
}
