#![no_main]

#[macro_use]
extern crate libfuzzer_sys;
extern crate parity_subtle;
extern crate core;

use core::intrinsics::transmute;

use parity_subtle::ConditionallySelectable;

fuzz_target!(|data: &[u8]| {
    for y in data.iter() {
        unsafe {
            let mut x: i8 = 0;
            let y: i8 = transmute::<u8, i8>(*y);

            x.conditional_assign(&y, 0.into());
            assert_eq!(x, 0);

            x.conditional_assign(&y, 1.into());
            assert_eq!(x, y);
        }
    }
});
