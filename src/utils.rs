#[inline]
pub fn set_once_console_log() {
    use std::sync::Once;
    static SET_CONSOLE: Once = Once::new();
    SET_CONSOLE.call_once(|| {
        console_log::init().unwrap();
    });
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    #[cfg(feature = "console_log")]
    set_once_console_log();
}
