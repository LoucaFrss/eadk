#![cfg_attr(not(test), no_std)]
pub use eadk_macros::*;

pub mod random;
pub use random::*;

pub mod color;
pub use color::*;

pub mod input;
pub use input::*;

pub mod prelude;

pub mod timing;
pub use timing::*;

pub mod backlight;
pub use backlight::*;

pub mod display;
pub use display::*;

#[repr(C)]
#[derive(Default, Clone, PartialEq, PartialOrd)]

pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}
#[repr(C)]
#[derive(Default, Clone, PartialEq, PartialOrd, Copy)]

pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub fn external_data() -> &'static [u8] {
    unsafe { core::slice::from_raw_parts(eadk_external_data, eadk_external_data_size) }
}

extern "C" {
    static eadk_external_data: *const u8;
    static eadk_external_data_size: usize;
}
