//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

impl crate::framebuffer::Framebuffer {
    pub fn draw_line(
        &self,
        x: usize,
        y: usize,
        len: usize,
        horizontal: bool,
        colour: u32,
    ) -> Result<(), &'static str> {
        if x + len > self.width || y + len > self.height {
            Err("x + len and/or y + len are out of screen bounds")
        } else {
            if horizontal {
                for i in 0..len {
                    self.draw_pixel(x + i, y, colour)?
                }
            } else {
                for i in 0..len {
                    self.draw_pixel(x, y + i, colour)?
                }
            }

            Ok(())
        }
    }
}
