#[macro_use]
extern crate log;
extern crate android_logger;

use android_logger::Config;
use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::JNIEnv;
use log::LevelFilter;
use linjector_rs::inject_code_to_pid;
use linjector_rs::utils::set_panic_handler;

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