use std::collections::BTreeMap;
use top_bar::TopBarWidget;
use window::WindowWidget;
use crate::pixel_font::{FontPixel, PixelFont};
use crate::widget::text_widget::TextWidget;

pub mod text_widget;
pub mod top_bar;
pub mod window;

//TODO: click handling
//TODO: redraw only if necessary (WIP)

///A trait defining functions every widget must have. A widget is the basic building block of
/// Modesto Desktop. Everything from the top level (MainWidget) to, say, a basic text block (TextWidget)
/// is a widget.
pub trait Widget{
    ///Renders the widget by compositing rendered children.
    fn render(&mut self, width: usize, height: usize) -> Option<Vec<[u8; 4]>>;
    ///Gets a Vec of the children of the widget.
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>>;
    ///Gets minimum recommended bounds for widget.
    fn get_min_bounds(&self) -> WidgetBounds;
    ///Gets the cache for the widget (previously drawn)
    fn get_cache(&mut self) -> Vec<[u8; 4]>;
}

///A structure for returning 2d rect boundaries of widgets.
pub struct WidgetBounds{
    width: usize,
    height: usize
}


#[derive(Copy, Clone, Eq, PartialEq)]
///A colour, in RGB. 8-bit depth per component.
pub struct Color{
    r: u8,
    g:u8,
    b:u8,
}
impl Color{
    ///Returns a black Color struct
    pub fn black()->Color{
        Color{
            r: 0u8,
            g: 0u8,
            b: 0u8
        }
    }
    ///Returns a white Color struct
    pub fn white()-> Color{
        Color{
            r: 255u8,
            g: 255u8,
            b: 255u8
        }
    }
}

///Enum representing text alignment. Possible values: Right, Left, Center.
pub enum TextAlignment{
    Right,
    Left,
    Center
}

///Simple widget returning a rectangle of the given dimensions.
pub struct RectWidget{
    rec_width: usize,
    rec_height: usize,
}
impl Widget for RectWidget{
    fn render(&mut self, width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
        Some(vec![[128u8;4];width * height])
    }
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>> {
        todo!()
    }
    fn get_min_bounds(&self) -> WidgetBounds {
        WidgetBounds{
            width: self.rec_width,
            height: self.rec_height,
        }
    }

    fn get_cache(&mut self) -> Vec<[u8; 4]> {
        todo!()
    }
}

///Master widget holding the open windows in Modesto Desktop. Should only be instantiated once.
pub struct MainWidget{
    width: usize,
    height: usize,
    pub windows: Box<Vec<Box<WindowWidget>>>,
}
impl MainWidget{
    pub fn new(width: usize, height: usize)->Self{
        MainWidget{
            width, height, windows: Box::new(Vec::new()),
        }
    }
    pub fn reg_window(&mut self, window: Box<WindowWidget>){
        self.windows.push(window);
    }
    ///Renders the main widget
    pub fn render(&mut self, width: usize, height: usize) -> Vec<[u8; 4]> {
        if self.windows.len() > 0 {
            let mut buf = vec![[128u8; 4]; width * height];
            let top_bar = self.windows[0].render_top_bar(width, 20);
            buf = draw_on_top_at(
                0, 0,
                buf, width, height,
                &top_bar, width, 20);
            for window in self.windows.iter_mut().rev(){
                let bounds = window.get_min_bounds();
                buf= draw_on_top_at(
                    window.x_position, window.y_position,
                    buf, width, height,
                    &match window.render(bounds.width, bounds.height){
                        Some(v) => v,
                        None => panic!("Window.render should never return None")
                    }, bounds.width, bounds.height);
            }
            buf
        } else {
            vec![[255u8;4];width * height]
        }
    }
}

///Overwrites base vector(assumed to be 2d folded into 1d as row sequence) at given offsets with given top. Must be truncated to fit.
//TODO: optimize this function, it is called a lot!
pub fn draw_on_top_at(
    x_offset: usize,
    y_offset: usize,
    mut base: Vec<[u8; 4]>,
    base_width: usize,
    base_height: usize,
    top: &Vec<[u8; 4]>,
    top_width: usize,
    top_height: usize
) -> Vec<[u8; 4]>{
    //check if bounds are ok, else return a red rect
    if x_offset + top_width > base_width || y_offset + top_height > base_height{
        return vec![[255u8, 0u8, 0u8, 255u8]; base_height * base_width]
    }
    //works (surprisingly) first time ah yes, is there something wrong????
    for y in 0..top_height{
        for x in 0..top_width{
            base[(y_offset + y) * base_width + (x_offset + x)] = top[y * top_width + x];
        }
    }
    base
}

///Convert a buffer from FontPixels to an interpolation between the given foreground and background colours.
pub fn from_font_to_pixbuf(
    foreground: Color,
    background: Color,
    buffer: &Vec<FontPixel>
) -> Vec<[u8;4]>{
    buffer.iter().map(|p| {
        [
            ((foreground.r as i32 - background.r as i32) * p.alpha as i32 + background.r as i32) as u8,
            ((foreground.g as i32 - background.g as i32) * p.alpha as i32 + background.g as i32) as u8,
            ((foreground.b as i32 - background.b as i32) * p.alpha as i32 + background.b as i32) as u8,
            255u8
        ]

        //[p.alpha, p.alpha, p.alpha, 255u8]
    }).collect()
}
