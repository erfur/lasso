use backtrace::Backtrace;
use std::{io::Read, panic};

pub fn set_panic_handler() {
    panic::set_hook(Box::new(|panic_info| {
        // Call the custom function to handle the panic
        handle_panic(panic_info);
    }));
}

pub(crate) fn handle_panic(panic_info: &panic::PanicInfo) {
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

pub fn hexdump(bytes: &[u8]) {
    // using debug!
    debug!("hexdump:");
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let addr = i * 16;
        let hex = chunk
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        let text = chunk
            .iter()
            .map(|b| if *b >= 32 && *b <= 126 {
                *b as char
            } else {
                '.'
            })
            .collect::<String>();
        debug!("{:08x}: {:<48} {}", addr, hex, text);
    }
}

pub fn hexdump_file(file_path: &str) {
    let mut file = match std::fs::File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Error opening file: {}", e);
            return;
        }
    };

    let mut buffer = [0; 0x200];
    file.read_exact(&mut buffer).unwrap();
    hexdump(&buffer);
}
