#![no_std]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Foo {
    thing: u8,
}

pub fn foo() -> u16 {
    0
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
