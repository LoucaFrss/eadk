pub type Color = u16;

#[macro_export]
macro_rules! rgb {
    ($r: expr, $g: expr, $b: expr) => {
        ((($r as u16 & 0b11111000) << 8) + (($g as u16 & 0b11111100) << 3) + ($b as u16 >> 3))
    };
}

pub const BLACK: Color = rgb!(0, 0, 0);
pub const WHITE: Color = rgb!(255, 255, 255);
pub const RED: Color = rgb!(255, 0, 0);
pub const GREEN: Color = rgb!(0, 255, 0);
pub const BLUE: Color = rgb!(0, 0, 255);
