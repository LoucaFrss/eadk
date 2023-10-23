use super::{Point, Rect};

pub trait Random: Sized {
    fn random() -> Self {
        unsafe {
            let mut value: Self = core::mem::zeroed();
            let value_bytes = core::slice::from_raw_parts_mut(
                &mut value as *mut Self as *mut u8,
                core::mem::size_of::<Self>(),
            );
            {
                let length = value_bytes.len();
                for i in (0..value_bytes.len()).step_by(4) {
                    let data = eadk_random();
                    let data: [u8; 4] = data.to_ne_bytes();
                    let chunk = &mut value_bytes[i..(i + 4).min(length)];
                    chunk.clone_from_slice(&data[..chunk.len()]);
                }
            }
            value
        }
    }
}

macro_rules! impl_random {
    ($($t: ty),*) => {
        $(
            impl Random for $t {

            }

        )*

    };

}
impl_random!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl Random for Rect {
    fn random() -> Self {
        Self {
            x: (random::<u32>() % 320) as u16,
            y: (random::<u32>() % 240) as u16,
            w: (random::<u32>() % 320) as u16,
            h: (random::<u32>() % 320) as u16,
        }
    }
}
impl Random for Point {
    fn random() -> Self {
        Self {
            x: (random::<u32>() % 320) as u16,
            y: (random::<u32>() % 240) as u16,
        }
    }
}
pub fn random<T: Random>() -> T {
    T::random()
}

extern "C" {
    fn eadk_random() -> u32;
}
