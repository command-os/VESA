#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Bitmask {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// This function is expensive, as it needs to turn the bitmask into a bit offset
    pub fn to_u32(&self, bitmask: Bitmask) -> u32 {
        let red_pixel = bitmask.r.leading_zeros();
        let green_pixel = bitmask.g.leading_zeros();
        let blue_pixel = bitmask.b.leading_zeros();
        let alpha_pixel = bitmask.a.leading_zeros();

        ((self.r as u32) << red_pixel)
            | ((self.g as u32) << green_pixel)
            | ((self.b as u32) << blue_pixel)
            | ((self.a as u32) << alpha_pixel)
    }
}
