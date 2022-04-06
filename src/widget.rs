use std::collections::BTreeMap;
use crate::pixel_font::{FontPixel, PixelFont};

//TODO: click handling
//TODO: redraw only if necessary

///A trait defining functions every widget must have. A widget is the basic building block of
/// Modesto Desktop. Everything from the top level (MainWidget) to, say, a basic text block (TextWidget)
/// a widget.
pub trait Widget{
    ///Renders the widget by compositing rendered children.
    fn render(&self, width: usize, height: usize) -> Vec<[u8; 4]>;
    ///Gets a Vec of the children of the widget.
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>>;
    ///Gets minimum recommended bounds for widget.
    fn get_min_bounds(&self) -> WidgetBounds;
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

///A widget representing a piece of text in a given pixel font.
pub struct TextWidget{
    font: Box<PixelFont>,
    wrap: bool,
    text: &'static str,
    foreground_col: Color,
    background_col: Color,
}
impl Widget for TextWidget{
    fn render(&self, width: usize, height: usize) -> Vec<[u8; 4]> {
        let mut buf = vec![[255u8; 4]; width * height];
        let mut xoff : usize = 0;
        for char in self.text.chars(){
            let mut w = 8;
            let mut h = 16;
            let mut right_off = 1;
            buf = draw_on_top_at(
                xoff, 0, buf, width, height,
                &match self.font.charset.get(&char){
                    Some(c) =>
                        {
                            w = c.width; h = c.height; right_off = c.right_offset;
                            from_font_to_pixbuf(self.foreground_col, self.background_col, &*c.pixels)
                        },
                    None =>
                        vec![[0u8;4]; w*h]
                }, w, h);
            xoff += (w + right_off) as usize;
        }
        buf
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
}

///Enum representing text alignment. Possible values: Right, Left, Center.
pub enum TextAlignment{
    Right,
    Left,
    Center
}


///Widget representing buttons on the top bar/global menu
pub struct TopBarButton{
    text: Box<TextWidget>,
    actions: Box<BTreeMap<Box<str>, Box<dyn Fn()>>>,
    opened: bool,
}
impl TopBarButton{
    ///Create a new TopBarButton with the given label and fold-down actions.
    pub fn new(label: Box<&'static str>, actions: Box<BTreeMap<Box<str>, Box<dyn Fn()>>>) -> Self{
        TopBarButton{
            text: Box::new(TextWidget{
                font: Box::new(PixelFont::default()),
                wrap: false,
                text: &label,
                foreground_col: Color::black(),
                background_col: Color::white()
            }),
            actions,
            opened: false
        }
    }
    ///Calculate the needed with for the fold-out button box
    fn get_max_action_box_width(&self) -> usize{
        todo!()
    }
    ///Calculate the height of the fold-out button box
    fn get_max_action_box_height(&self) -> usize{
        todo!()
    }
}
impl Widget for TopBarButton{
    fn render(&self, width: usize, height: usize) -> Vec<[u8; 4]> {
        self.text.render(width, height)
    }
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>> {
        todo!()
    }
    fn get_min_bounds(&self) -> WidgetBounds{
        if !self.opened{
            self.text.get_min_bounds()
        } else{
            WidgetBounds{
                width: self.get_max_action_box_width(),
                height: self.get_max_action_box_height(),
            }
        }
    }
}

///Widget representing the top bar/global menu.
pub struct TopBarWidget {
    //logo will be done by a special character
    buttons: Box<Vec<Box<dyn Widget>>>,
    //parent: Option<Box<dyn Widget>>,
}
impl TopBarWidget {
    ///Create a new top bar with the given set of buttons. Takes Box<Vec<Box<dyn Widget>>> but
    /// should only be used with TopBarButton.
    pub fn new(buttons: Box<Vec<Box<dyn Widget>>>) ->Self{
        TopBarWidget {
            buttons,
        }
    }
}
impl Widget for TopBarWidget {
    fn render(&self, width: usize, height: usize) -> Vec<[u8; 4]> {
        let mut buf = vec![[255u8;4]; width * height];
        let mut button_widths :Vec<usize> = Vec::new();
        let mut button_height = 0usize;
        let mut button_bufs : Vec<Vec<[u8;4]>> = Vec::new();
        for button in &*self.buttons{
            let bounds: WidgetBounds = button.get_min_bounds();
            button_bufs.push(button.render(bounds.width, bounds.height));
            button_widths.push(bounds.width);
            button_height = bounds.height;
        }
        let mut xoff = 10;
        let yoff = 2;
        for (idx, button_buf) in button_bufs.iter().enumerate(){
            buf = draw_on_top_at(
                xoff, yoff,
                buf, width, height,
                &button_buf, button_widths[idx], button_height);
            xoff += button_widths[idx] + 10;
        }
        //add line at the bottom of the top bar
        for i in 0..width{
            buf[(height - 1) * width + i] = [0u8, 0u8, 0u8, 255u8];
        }
        buf
    }
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>> {
        Some(&self.buttons)
    }
    fn get_min_bounds(&self) -> WidgetBounds {
        todo!()
    }
}

///Widget representing the title bar of a window.
pub struct WindowTopBarWidget{
    button: Box<WindowTopBarButton>,
    title: Box<TextWidget>,
}
impl Widget for WindowTopBarWidget{
    fn render(&self, width: usize, height: usize) -> Vec<[u8; 4]> {
        let mut buf = vec![[255u8; 4]; width * height];
        //make the borders black
        for i in 0..width{
            buf[i] = [0u8, 0u8, 0u8, 255u8];
            for j in 4..(height-4){
                if j % 2 == 0 && i > 4 && i < width-4{
                    buf[j * width + i] = [0u8, 0u8, 0u8, 255u8];
                }
            }
            buf[(height - 1) * width + i] = [0u8, 0u8, 0u8, 255u8];
        }
        for i in 0..height{
            buf[i * width + 0] = [0u8, 0u8, 0u8, 255u8];
            buf[i * width + (width - 1)] = [0u8, 0u8, 0u8, 255u8];
        }
        let button_bounds = self.button.get_min_bounds();
        buf = draw_on_top_at(
            4, 4,
            buf, width, height,
            &self.button.render(button_bounds.width, button_bounds.height),
            button_bounds.width, button_bounds.height);
        let text_bounds = self.title.get_min_bounds();
        let title = self.title.render(text_bounds.width, text_bounds.height);
        let title_x_offset = width/2 - text_bounds.width / 2;
        draw_on_top_at(
            title_x_offset, 2,
            buf, width, height,
            &title, text_bounds.width, text_bounds.height)
    }
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>> {
        todo!()
    }
    fn get_min_bounds(&self) -> WidgetBounds {
        todo!()
    }
}

///Widget representing the singular button in a window top bar (close)
pub struct WindowTopBarButton{
    pressed: bool,
}
impl Widget for WindowTopBarButton{
    fn render(&self, width: usize, height: usize) -> Vec<[u8; 4]> {
        if self.pressed {
            vec![[0u8, 0u8, 0u8, 255u8]; width * height]
        } else {
            let mut buf = vec![[255u8; 4]; width * height];
            for i in 0..width {
                buf[i] = [0u8, 0u8, 0u8, 255u8];
                buf[(height - 1) * width + i] = [0u8, 0u8, 0u8, 255u8]
            }
            for i in 1..(height - 1) {
                buf[i * width] = [0u8, 0u8, 0u8, 255u8];
                buf[i * width + (width - 1)] = [0u8, 0u8, 0u8, 255u8];
            }
            buf
        }

    }

    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>> {
        None
    }

    fn get_min_bounds(&self) -> WidgetBounds {
        WidgetBounds{
            width: 11,
            height: 11
        }
    }
}

///Widget representing a window in Modesto Desktop.
pub struct WindowWidget{
    pub is_moving: bool,
    top_bar: Box<TopBarWidget>,
    window_top_bar: WindowTopBarWidget,
    window_body: Box<dyn Widget>,
    pub x_position: usize,
    pub y_position: usize,
    width: usize,
    height: usize,
}
impl Widget for WindowWidget{
    fn render(&self, width: usize, height: usize) -> Vec<[u8; 4]> {
        //TODO: render widgets after layouting
        if self.is_moving {
            let mut buf = vec![[128u8, 128u8, 128u8, 255u8]; width * height];
            for i in 0..width{
                buf[i] = [0u8,0u8,0u8,0u8];
                buf[(height - 1)*width + i] = [0u8, 0u8, 0u8, 0u8];
            }
            for i in 0..height{
                buf[i * width] = [0u8,0u8,0u8,0u8];
                buf[i* width + width - 1] = [0u8, 0u8, 0u8, 0u8];
            }
            return buf
        }
        draw_on_top_at(
            0, 0,
            vec![[0u8, 0u8, 0u8, 255u8];width * height],
            width, height,
            &self.window_top_bar.render(width, 20),
            width, 20)

    }
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>> {
        todo!()
    }
    fn get_min_bounds(&self) -> WidgetBounds {
        WidgetBounds{
            width: self.width, height : self.height,
        }
    }
}
impl WindowWidget{
    ///Create a new WindowWidget with the given title, dimensions and position
    pub fn new(title: &'static str, width: usize, height: usize, xpos: usize, ypos: usize)-> Self{
        WindowWidget{
            is_moving: false,
            top_bar: Box::new(TopBarWidget {
                buttons: Box::new(vec![])
            }),
            window_top_bar: WindowTopBarWidget {
                button: Box::new(WindowTopBarButton{pressed: false}),
                title: Box::new(TextWidget{
                    font: Box::new(PixelFont::default()),
                    wrap: false,
                    text: title,
                    foreground_col: Color::black(),
                    background_col: Color::white()
                })
            },
            window_body: Box::new(RectWidget{
                recHeight: height, recWidth: width
            }),
            width,
            height,
            x_position: xpos,
            y_position: ypos,
        }
    }
    ///Register a top bar/global menu for the window.
    pub fn register_top_bar(&mut self, top_bar: Box<TopBarWidget>){
        self.top_bar = top_bar;
    }
    ///Render the top bar associated with the window.
    fn render_top_bar(&self, width: usize, height: usize) -> Vec<[u8; 4]>{
        self.top_bar.render(width, height)
    }
}

///Simple widget returning a rectangle of the given dimensions.
pub struct RectWidget{
    recWidth: usize,
    recHeight: usize,
}
impl Widget for RectWidget{
    fn render(&self, width: usize, height: usize) -> Vec<[u8; 4]> {
        vec![[128u8;4];width * height]
    }
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>> {
        todo!()
    }
    fn get_min_bounds(&self) -> WidgetBounds {
        WidgetBounds{
            width: self.recWidth,
            height: self.recHeight,
        }
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
}
impl Widget for MainWidget{
    fn render(&self, width: usize, height: usize) -> Vec<[u8; 4]> {
        if self.windows.len() > 0 {
            //TODO: render active top bar and window
            let mut buf = vec![[128u8; 4]; width * height];
            let top_bar = self.windows[0].render_top_bar(width, 20);
            buf = draw_on_top_at(
                0, 0,
                buf, width, height,
                &top_bar, width, 20);
            for window in self.windows.iter().rev(){
                let bounds = window.get_min_bounds();
                buf= draw_on_top_at(
                    window.x_position, window.y_position,
                    buf, width, height,
                    &window.render(bounds.width, bounds.height), bounds.width, bounds.height);
            }
            buf
        } else {
            vec![[255u8;4];width * height]
        }
    }
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>> {
        todo!()
    }
    fn get_min_bounds(&self) -> WidgetBounds {
        WidgetBounds{
            width: self.width, height: self.height
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