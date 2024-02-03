#[macro_use]
extern crate log;
extern crate android_logger;

use android_logger::Config;
use jni::objects::JClass;
use jni::sys::jint;
use jni::JNIEnv;
use log::LevelFilter;
use linjector_rs::inject_code_to_pid;
use linjector_rs::utils::set_panic_handler;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_github_erfur_lasso_AppProcessFinderService_initLasso<'local>(
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
pub extern "system" fn Java_com_github_erfur_lasso_AppProcessFinderService_injectCode<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    pid: jint,
) {
    let pid: i32 = pid as i32;
    debug!("pid: {}", pid);

    inject_code_to_pid(pid);
}

fn main() {
    ()
}