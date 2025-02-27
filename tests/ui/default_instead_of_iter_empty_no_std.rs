#![warn(clippy::default_instead_of_iter_empty)]
#![allow(dead_code)]
#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[derive(Default)]
struct Iter {
    iter: core::iter::Empty<usize>,
}

fn main() {
    // Do lint.
    let _ = core::iter::Empty::<usize>::default();
    //~^ default_instead_of_iter_empty
    let _foo: core::iter::Empty<usize> = core::iter::Empty::default();
    //~^ default_instead_of_iter_empty

    // Do not lint.
    let _ = Iter::default();
}
