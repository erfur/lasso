use crate::utils::hexdump_file;

mod remote_mem;
mod remote_module;
mod remote_proc;
mod shellcode;
pub mod utils;

#[macro_use]
extern crate log;
extern crate android_logger;

pub fn inject_code_to_pid(pid: i32, file_path: String) {
    hexdump_file(file_path.as_str());

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
        file_path,
        malloc_sym,
    );

    let first_stage = shellcode::main_shellcode(timezone_sym, second_stage.len());

    let malloc_original_bytes = proc.rm.read_mem(malloc_sym, first_stage.len());
    let timezone_original_bytes = proc.rm.read_mem(timezone_sym, 0x8);

    info!("write first stage shellcode");
    proc.rm.write_mem(timezone_sym, &vec![0x0; 0x8]);
    proc.rm.write_mem(malloc_sym, &first_stage);

    info!("wait for shellcode to trigger");
    let mut new_map: u64;
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
