/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

pub mod shapes;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    pub fn draw_pixel(&self, x: usize, y: usize, colour: u32) -> Result<(), &'static str> {
        if x > self.width || y > self.height {
            Err("x and/or y are out of screen bounds")
        } else {
            unsafe { self.base.add(x).add(self.pitch * y).write(colour) }

            Ok(())
        }
    }

    #[inline]
    pub fn clear(&self, colour: u32) -> Result<(), &'static str> {
        for y in 0..(self.height) {
            for x in 0..(self.width) {
                self.draw_pixel(x, y, colour)?;
            }
        }

        Ok(())
    }
}
