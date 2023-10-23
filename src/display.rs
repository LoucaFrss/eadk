use crate::Color;

use super::Point;
use super::Rect;

pub fn push_rect(rect: Rect, pixels: &[Color]) {
    unsafe {
        eadk_display_push_rect(rect, pixels.as_ptr());
    }
}

pub fn push_rect_uniform(rect: Rect, color: Color) {
    unsafe {
        eadk_display_push_rect_uniform(rect, color);
    }
}

pub fn wait_for_vblank() {
    unsafe {
        eadk_display_wait_for_vblank();
    }
}
pub fn draw_string(
    text: &[u8],
    point: Point,
    large_format: bool,
    text_color: Color,
    background_color: Color,
) {
    unsafe {
        eadk_display_draw_string(
            text.as_ptr(),
            point,
            large_format,
            text_color,
            background_color,
        )
    }
}

extern "C" {
    fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
    fn eadk_display_push_rect(rect: Rect, color: *const Color);
    fn eadk_display_wait_for_vblank();
    fn eadk_display_draw_string(
        text: *const u8,
        point: Point,
        large_format: bool,
        text_color: Color,
        background_color: Color,
    );
}
pub struct TextBuf<'a> {
    buf: &'a mut [u8],
    pub offset: usize,
}
impl<'a> TextBuf<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, offset: 0 }
    }
}

impl<'a> core::fmt::Write for TextBuf<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();

        // Skip over already-copied data
        let remainder = &mut self.buf[self.offset..];
        // Check if there is space remaining (return error instead of panicking)
        if remainder.len() < bytes.len() {
            return Err(core::fmt::Error);
        }
        // Make the two slices the same length
        let remainder = &mut remainder[..bytes.len()];
        // Copy
        remainder.copy_from_slice(bytes);

        // Update offset to avoid overwriting
        self.offset += bytes.len();

        Ok(())
    }
}
#[macro_export]
macro_rules! println {
    ($($arg:expr),*) => {
        {
            let mut buf = [0;256];

            use ::core::fmt::Write;
            let offset = {
                let len = buf.len();
                let mut text_buf  =$crate::TextBuf::new(&mut buf[..len-1]);
                write!(text_buf, $($arg),*).unwrap();
                text_buf.offset
            };
            $crate::display::draw_string(&buf[..offset+1], $crate::Point{x: 0, y:30}, false, rgb!(0, 0, 0), u16::MAX);
        }
    };
}
#[macro_export]
macro_rules! eprintln {
    ($($arg:expr),*) => {
{
    let mut buf = [0;256];

    use ::core::fmt::Write;
    let offset = {
        let len = buf.len();
        let mut text_buf  =$crate::TextBuf::new(&mut buf[..len-1]);
        write!(text_buf, $($arg),*).unwrap();
        text_buf.offset
    };
    $crate::display::draw_string(&buf[..offset+1], $crate::Point{x: 0, y:30}, false, rgb!(255, 0, 0), u16::MAX);
}
    };
}
