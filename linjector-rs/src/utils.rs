use backtrace::Backtrace;
use std::panic;

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
