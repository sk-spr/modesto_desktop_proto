use crate::pixel_font::PixelFont;
use crate::widget;
use crate::widget::{Color, RectWidget, Widget, WidgetBounds};
use crate::widget::text_widget::TextWidget;
use crate::widget::top_bar::TopBarWidget;

///Widget representing the title bar of a window.
pub struct WindowTopBarWidget{
    button: Box<WindowTopBarButton>,
    title: Box<TextWidget>,
    cache: Box<Vec<[u8; 4]>>,
    cache_width: usize,
    cache_height: usize,
    needs_redraw: bool,
}

impl WindowTopBarWidget{
    fn new(title: &'static str) -> Self{
        WindowTopBarWidget{
            title: Box::new(TextWidget::new(
                Box::new(PixelFont::default()),
                false,
                title,
                Color::black(),
                Color::white())),
            button: Box::new(WindowTopBarButton{
                pressed: false
            }),
            cache: Box::new(vec![]),
            cache_height: 0,
            cache_width: 0,
            needs_redraw: true,
        }
    }
}

impl Widget for WindowTopBarWidget{
    fn render(&mut self, width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
        if !self.needs_redraw && self.cache.len() > 1{
            if self.cache_height == height && self.cache_width == width{
                return None
            }
        }
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
        buf = widget::draw_on_top_at(
            4, 4,
            buf, width, height,
            &match self.button.render(button_bounds.width, button_bounds.height){
                Some(v) => v,
                None => todo!()
            },
            button_bounds.width, button_bounds.height);
        let text_bounds = self.title.get_min_bounds();
        let title = match self.title.render(text_bounds.width, text_bounds.height){
            Some(v) => v,
            None => todo!()
        };
        let title_x_offset = width/2 - text_bounds.width / 2;
        let out = widget::draw_on_top_at(
            title_x_offset, 2,
            buf, width, height,
            &title, text_bounds.width, text_bounds.height);
        self.cache = Box::new(out.clone());
        self.cache_height = height;
        self.cache_width = width;
        Some(out)
    }
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>> {
        todo!()
    }
    fn get_min_bounds(&self) -> WidgetBounds {
        todo!()
    }

    fn get_cache(&mut self) -> Vec<[u8; 4]> {
        *self.cache.clone()
    }
}

///Widget representing the singular button in a window top bar (close)
pub struct WindowTopBarButton{
    pressed: bool,
}

impl Widget for WindowTopBarButton{
    fn render(&mut self, width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
        if self.pressed {
            Some(vec![[0u8, 0u8, 0u8, 255u8]; width * height])
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
            Some(buf)
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

    fn get_cache(&mut self) -> Vec<[u8; 4]> {
        todo!()
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
    cache: Box<Vec<[u8;4]>>,
    cache_width: usize,
    cache_height: usize,
    needs_redraw : bool,
}

impl Widget for WindowWidget{
    fn render(&mut self, width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
        if !self.needs_redraw && self.cache.len() > 0
            && self.cache_width == width && self.cache_height == height{
            return Some(*self.cache.clone())
        }
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
            self.cache = Box::new(buf.clone());
            self.cache_width = width;
            self.cache_height = height;
            self.needs_redraw = false;
            return Some(buf)
        }
        let buf = widget::draw_on_top_at(
            0, 0,
            vec![[0u8, 0u8, 0u8, 255u8];width * height],
            width, height,
            &match self.window_top_bar.render(width, 20){
                Some(v) => v,
                None => self.window_top_bar.get_cache()
            },
            width, 20);
        self.cache_height = height;
        self.cache_width = width;
        self.cache = Box::new(buf.clone());
        self.needs_redraw = false;
        Some(buf)
    }
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>> {
        todo!()
    }
    fn get_min_bounds(&self) -> WidgetBounds {
        WidgetBounds{
            width: self.width, height : self.height,
        }
    }

    fn get_cache(&mut self) -> Vec<[u8; 4]> {
        todo!()
    }
}

impl WindowWidget{
    ///Create a new WindowWidget with the given title, dimensions and position
    pub fn new(title: &'static str, width: usize, height: usize, xpos: usize, ypos: usize)-> Self{
        WindowWidget{
            is_moving: false,
            top_bar: Box::new(TopBarWidget::new(
                Box::new(vec![])
            )),
            window_top_bar: WindowTopBarWidget::new(title),
            window_body: Box::new(RectWidget{
                rec_height: height, rec_width: width
            }),
            width,
            height,
            x_position: xpos,
            y_position: ypos,
            cache: Box::new(vec![]),
            cache_height: 0,
            cache_width: 0,
            needs_redraw: true,
        }
    }
    ///Register a top bar/global menu for the window.
    pub fn register_top_bar(&mut self, top_bar: Box<TopBarWidget>){
        self.top_bar = top_bar;
    }
    ///Render the top bar associated with the window.
    pub(crate) fn render_top_bar(&mut self, width: usize, height: usize) -> Vec<[u8; 4]>{
        match self.top_bar.render(width, height){
            Some(v) => v,
            None => self.top_bar.get_cache()
        }
    }
    pub fn set_moving(&mut self, new_status: bool){
        self.is_moving = new_status;
        self.needs_redraw = true;
    }
}
