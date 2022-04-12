use crate::pixel_font::PixelFont;
use crate::widget;
use crate::widget::{Color, Widget, WidgetBounds};
use crate::widget::mouse::{MouseCallbackRegistrar, MouseEvent, MousePosition};

///A widget representing a piece of text in a given pixel font. Non-caching.
pub struct TextWidget{
    font: Box<PixelFont>,
    wrap: bool,
    text: &'static str,
    foreground_col: Color,
    background_col: Color,
}

impl Widget for TextWidget{
    fn render(&mut self, width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
        let mut buf = vec![[255u8; 4]; width * height];
        let mut xoff : usize = 0;
        for char in self.text.chars(){
            let mut w = 8;
            let mut h = 16;
            let mut right_off = 1;
            buf = widget::draw_on_top_at(
                xoff, 0,
                buf, width, height,
                &match self.font.charset.get(&char){
                    Some(c) =>
                        {
                            w = c.width; h = c.height; right_off = c.right_offset;
                            widget::from_font_to_pixbuf(self.foreground_col, self.background_col, &*c.pixels)
                        },
                    None =>
                        vec![[0u8;4]; w*h]
                }, w, h);
            xoff += (w + right_off) as usize;
        }
        Some(buf)
    }
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>> {
        return None
    }
    fn get_min_bounds(&self) -> WidgetBounds {
        let mut w = 0;
        let mut h = 0;
        for c in self.text.chars(){
            w += match self.font.charset.get(&c){
                Some(c) => {
                    if c.height > h{h = c.height};
                    c.width + c.right_offset
                },
                None => {
                    if 16 > h {h = 16}
                    9
                }
            }
        }
        WidgetBounds{
            width:w,
            height: h,
        }
    }

    fn get_cache(&mut self) -> Vec<[u8; 4]> {
        match self.render(self.get_min_bounds().width, self.get_min_bounds().height){
            Some(v) => v,
            None => panic!("TextWidget should never return None for render, since it is a bottom level widget"),
        }
    }

    fn handle_mouse_event(&mut self, mouse_position: MousePosition, relative_mouse_position: MousePosition, mouse_event: MouseEvent, registrar: &mut MouseCallbackRegistrar) -> () {}
}

impl TextWidget{
    pub fn new(
        font: Box<PixelFont>,
        wrap: bool,
        text: &'static str,
        foreground_col: Color,
        background_col: Color,
    ) -> Self{
        TextWidget{
            font, wrap, text, foreground_col, background_col,
        }
    }
}
