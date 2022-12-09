#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader_api::config::Mapping;
use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use core::panic::PanicInfo;
use kernel::executor::{Executor, Task};
use kernel::render::render;
use kernel::serial_println;

pub static CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.mappings.dynamic_range_start = Some(0);
    config.mappings.dynamic_range_end = Some(0xffff_ffff_ffff_f000);
    config.kernel_stack_size = 100 * 1024;
    config
};

entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init(boot_info);
    let x = 0x1 as *mut u8;
    unsafe {
        x.write(0x01);
    }

    #[cfg(test)]
    test_main();

    // let mut executor = Executor::new();
    // executor.spawn(Task::new(render()));
    // executor.run();
    loop {}
}
/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use kernel::serial_println;
    serial_println!("{}", info);
    kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
