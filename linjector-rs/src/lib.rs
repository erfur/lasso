mod remote_mem;
mod remote_module;
mod remote_proc;
mod shellcode;
mod utils;

#[macro_use]
extern crate log;
extern crate android_logger;

use proc_maps::get_process_maps;
use proc_maps::Pid;
use std::panic;

use android_logger::Config;
use log::LevelFilter;

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jint, jstring};

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

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_github_erfur_lasso_AppProcessFinderService_hello<'local>(
    mut env: JNIEnv<'local>,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    _class: JClass<'local>,
    input: JString<'local>,
) -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env
        .new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_raw()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_github_erfur_lasso_AppProcessFinderService_getMaps<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    pid: jint,
) {
    let pid: i32 = pid as i32;
    debug!("pid: {}", pid);

    match get_process_maps(pid as Pid) {
        Ok(maps) => {
            for map in maps {
                debug!("map: {:?}", map);
            }
        }
        Err(e) => {
            debug!("error: {:?}", e);
        }
    }
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

    let proc = remote_proc::RemoteProc::new(pid as u16);
    let libc = proc.get_module("libc.so");
    let libdl = proc.get_module("libdl.so");
    // let liblasso = proc.get_module("liblasso.so");

    debug!("{}, 0x{:x}", &libc.name, &libc.vm_addr);
    debug!("{}, 0x{:x}", &libdl.name, &libdl.vm_addr);

    let timezone_sym = libc.dlsym_from_fs("timezone");
    let malloc_sym = libc.dlsym_from_fs("malloc");
    // let timezone_sym = liblasso.dlsym_from_fs("test_var");
    // let malloc_sym = liblasso.dlsym_from_fs("Java_com_github_erfur_lasso_MainActivity_testFunction");
    let dlopen_sym = libdl.dlsym_from_fs("dlopen");
    let sprintf_sym = libc.dlsym_from_fs("sprintf");

    debug!("timezone_sym: 0x{:x}", timezone_sym);
    debug!("malloc_sym:   0x{:x}", malloc_sym);
    debug!("dlopen_sym:   0x{:x}", dlopen_sym);
    debug!("sprintf_sym:  0x{:x}", sprintf_sym);

    let second_stage = shellcode::raw_dlopen_shellcode(
        dlopen_sym,
        "/data/local/tmp/frida-gadget.so".to_string(),
        malloc_sym,
    );

    let first_stage = shellcode::main_shellcode(timezone_sym, second_stage.len());

    let malloc_original_bytes = proc.rm.read_mem(malloc_sym, first_stage.len());
    let timezone_original_bytes = proc.rm.read_mem(timezone_sym, 0x8);

    info!("write first stage shellcode");
    proc.rm.write_mem(timezone_sym, &vec![0x0; 0x8]);
    proc.rm.write_mem(malloc_sym, &first_stage);

    info!("wait for shellcode to trigger");
    let mut new_map: u64 = 0;
    loop {
        let data = proc.rm.read_mem(timezone_sym, 0x8);
        // u64 from val
        new_map = u64::from_le_bytes(data[0..8].try_into().unwrap());
        if (new_map & 0x1 != 0) && (new_map & 0xffff_ffff_ffff_fff0 != 0) {
            break;
        }
    }

    new_map &= 0xffff_ffff_ffff_fff0;
    info!("new map: 0x{:x}", new_map);

    info!("overwrite malloc with loop");
    proc.rm.write_mem(malloc_sym, &shellcode::self_jmp());

    // wait for 100ms
    std::thread::sleep(std::time::Duration::from_millis(100));

    info!("restore original bytes");
    proc.rm.write_code(malloc_sym, &malloc_original_bytes, 1);
    proc.rm.write_mem(timezone_sym, &timezone_original_bytes);

    info!("overwrite new map");
    proc.rm.write_code(new_map as usize, &second_stage, 1);

    info!("injectCode");
}
