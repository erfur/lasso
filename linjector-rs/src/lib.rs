mod remote_mem;
mod remote_module;
mod remote_proc;
mod shellcode;
mod utils;
mod linjector;

#[macro_use]
extern crate log;
extern crate android_logger;

use std::panic;
use android_logger::Config;
use log::LevelFilter;
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jint;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_github_erfur_lasso_AppProcessFinderService_initLasso<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
) {
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Debug));
    debug!("init logger");

    panic::set_hook(Box::new(|panic_info| {
        // Call the custom function to handle the panic
        utils::handle_panic(panic_info);
    }));
    debug!("init panic");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_github_erfur_lasso_AppProcessFinderService_injectCode<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    pid: jint,
) {
    let pid: i32 = pid as i32;
    debug!("pid: {}", pid);

    linjector::inject_code_to_pid(pid);
}
