mod mem_common;

pub(super) mod dummy;

pub mod config;
pub mod mem;
pub mod serial;
pub mod thread;
pub mod timer;
pub mod vdso;
pub mod vm;

pub use super::hal_fn::{context, cpu, interrupt, rand};

hal_fn_impl_default!(context, cpu, interrupt, rand);

cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod dev;
    } else {
        pub use super::hal_fn::dev;
        hal_fn_impl_default!(dev::fb, dev::input);
    }
}

#[cfg(target_os = "macos")]
include!("macos.rs");

/// Initialize the HAL.
///
/// This function must be called at the beginning.
pub fn init() {
    crate::KHANDLER.init_by(&crate::DummyKernelHandler);

    #[cfg(target_os = "macos")]
    unsafe {
        register_sigsegv_handler();
    }
    // spawn a thread to read stdin
    // TODO: raw mode
    use std::io::Read;
    std::thread::spawn(|| {
        for i in std::io::stdin().bytes() {
            serial::serial_put(i.unwrap());
        }
    });
}