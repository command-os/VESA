//! Copyright (c) ChefKiss Inc 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives license.

pub mod shapes;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Framebuffer {
    pub base: *mut u32,
    pub width: usize,
    pub height: usize,
    pub bitmask: crate::pixel::Bitmask,
    pub pitch: usize,
}

impl Framebuffer {
    pub fn new(
        base: *mut u32,
        width: usize,
        height: usize,
        bitmask: crate::pixel::Bitmask,
        pitch: usize,
    ) -> Self {
        Self {
            base,
            width,
            height,
            bitmask,
            pitch,
        }
    }

    #[inline]
    pub fn clear(&self, colour: u32) -> Result<(), &'static str> {
        let colour = ((colour as u64) << 32) | colour as u64;
        for i in 0..((self.height * self.pitch + 1) / 2) {
            unsafe { (self.base as *mut u64).add(i).write_volatile(colour) }
        }

        Ok(())
    }

    #[inline]
    pub fn plot_pixel(&self, x: usize, y: usize, colour: u32) -> Result<(), &'static str> {
        if x > self.width || y > self.height {
            Err("x and/or y are out of screen bounds")
        } else {
            unsafe { self.base.add(x).add(self.pitch * y).write_volatile(colour) }

            Ok(())
        }
    }
}
