#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(finn_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use finn_os::executor::{Executor, Task};
use finn_os::serial_print;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    finn_os::init(boot_info);

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    finn_os::test_panic_handler(info)
}

#[test_case]
fn test_executor() {
    async fn example_task() {
        serial_print!("async print ")
    }

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.test_run()
}
