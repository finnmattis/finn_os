#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(finn_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use finn_os::graphics::VGA;
use finn_os::render::RENDERER;
use finn_os::task::keyboard;
use finn_os::task::{executor::Executor, Task};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    finn_os::init(boot_info);

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    // executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.spawn(Task::new(RENDERER.render()));
    executor.run();
}
/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use finn_os::serial_println;
    serial_println!("{}", info);
    finn_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    finn_os::test_panic_handler(info)
}
