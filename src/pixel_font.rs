use std::collections::BTreeMap;

#[derive(Clone, Copy, Eq, PartialEq)]
///struct representing an 8 bit font pixel (Alpha only)
pub struct FontPixel{
    pub(crate) alpha: u8,
}

#[derive(Clone, Eq, PartialEq)]
pub struct PixelFontChar{
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub right_offset: usize,
    pub(crate) pixels: Box<Vec<FontPixel>>,
}

#[derive(Clone, Eq, PartialEq)]
///Structure representing a pixel font with a set of characters in a specific size
pub struct PixelFont {
    size_in_pts: u32,
    pub(crate) charset: Box<BTreeMap<char, PixelFontChar>>,
}

const B:FontPixel = FontPixel{alpha: 1};
const W:FontPixel = FontPixel{alpha: 255};
//hard-coded for now, TODO: load pixel fonts
impl PixelFont {
    pub fn default() -> Self{
        PixelFont{
            size_in_pts: 12,
            charset: Box::new(BTreeMap::from(
                [('B', PixelFontChar{
                        width: 8,
                        height: 16,
                        right_offset: 1,
                        pixels: Box::new(vec![
                            B, B, B, B, B, B, W, W,
                            B, B, B, B, B, B, B, W,
                            B, B, W, W, W, B, B, B,
                            B, B, W, W, W, W, B, B,
                            B, B, W, W, W, W, B, B,
                            B, B, W, W, W, W, B, B,
                            B, B, W, W, W, B, B, B,
                            B, B, B, B, B, B, B, W,
                            B, B, B, B, B, B, B, W,
                            B, B, W, W, W, B, B, B,
                            B, B, W, W, W, W, B, B,
                            B, B, W, W, W, W, B, B,
                            B, B, W, W, W, W, B, B,
                            B, B, W, W, W, B, B, B,
                            B, B, B, B, B, B, B, W,
                            B, B, B, B, B, B, W, W
                        ])
                    }),
                    ('u', PixelFontChar{
                        width: 8,
                        height: 16,
                        right_offset: 1,
                        pixels: Box::new(vec![
                            W, W, W, W, W, W, W, W,
                            W, W, W, W, W, W, W, W,
                            W, W, W, W, W, W, W, W,
                            W, W, W, W, W, W, W, W,
                            W, W, W, W, W, W, W, W,
                            W, W, W, W, W, W, W, W,
                            W, W, W, W, W, W, W, W,
                            W, W, W, W, W, W, W, W,
                            B, B, W, W, W, W, B, B,
                            B, B, W, W, W, W, B, B,
                            B, B, W, W, W, W, B, B,
                            B, B, W, W, W, W, B, B,
                            B, B, W, W, W, W, B, B,
                            B, B, W, W, W, W, B, B,
                            W, B, B, W, W, B, B, B,
                            W, W, B, B, B, B, W, B,
                        ])
                    }),
                    ('t', PixelFontChar{
                        width: 6,
                        height: 16,
                        right_offset: 0,
                        pixels: Box::new(vec![
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            B, B, B, B, B, B,
                            B, B, B, B, B, B,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                            W, W, B, B, W, W,
                        ])
                    })
                ]
            ))
        }

    }
}