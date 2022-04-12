use std::cell::Cell;
use std::sync::Arc;
use crate::pixel_font::PixelFont;
use crate::widget;
use crate::widget::{Color, RectWidget, TOP_BAR_HEIGHT, Widget, WidgetBounds};
use crate::widget::mouse::{MouseCallbackRegistrar, MouseEvent, MousePosition, MouseQueueResult};
use crate::widget::text_widget::TextWidget;
use crate::widget::top_bar::TopBarWidget;

const WINDOW_TOP_BAR_HEIGHT: usize = 30;

///Widget representing the title bar of a window.
pub struct WindowTopBarWidget{
    button: Box<WindowTopBarButton>,
    title: Box<TextWidget>,
    cache: Box<Vec<[u8; 4]>>,
    cache_width: usize,
    cache_height: usize,
    needs_redraw: bool,
    button_pressed_old: bool,
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
                pressed: Arc::new(Cell::new(false))
            }),
            cache: Box::new(vec![]),
            cache_height: 0,
            cache_width: 0,
            needs_redraw: true,
            button_pressed_old: false
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
            for j in 0..16{
                if j % 3 == 0 && i > 4 && i < width-4{
                    buf[(j+(height/4)) * width + i] = [0u8, 0u8, 0u8, 255u8];
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
            4, 7,
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
            title_x_offset, 7,
            buf, width, height,
            &title, text_bounds.width, text_bounds.height);
        self.cache = Box::new(out.clone());
        self.cache_height = height;
        self.cache_width = width;
        self.needs_redraw = false;
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

    fn handle_mouse_event(&mut self, mouse_position: MousePosition, relative_mouse_position: MousePosition, mouse_event: MouseEvent, registrar: &mut MouseCallbackRegistrar) -> () {
        println!("Handling window top bar mouse event, abs x={};y={} rel x={};y={}", mouse_position.x_position, mouse_position.y_position, relative_mouse_position.x_position, relative_mouse_position.y_position);
        if relative_mouse_position.x_position > 4 as f32 && relative_mouse_position.x_position < (4 + 16) as f32 &&
            relative_mouse_position.y_position > 7 as f32 && relative_mouse_position.y_position < (7+16) as f32{
            let rel_mouse_position = MousePosition{
                x_position: relative_mouse_position.x_position - 4 as f32,
                y_position: relative_mouse_position.y_position - 7 as f32,
            };
            self.needs_redraw = true;
            self.button.handle_mouse_event(mouse_position, rel_mouse_position, mouse_event, registrar);
            //TODO: add callback to queue
        }
    }
}

///Widget representing the singular button in a window top bar (close)
pub struct WindowTopBarButton{
    pressed: Arc<Cell<bool>>,
}

impl Widget for WindowTopBarButton{
    fn render(&mut self, width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
        if self.pressed.get() {
            println!("is pressed");
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
            width: 16,
            height: 16
        }
    }

    fn get_cache(&mut self) -> Vec<[u8; 4]> {
        todo!()
    }

    fn handle_mouse_event(&mut self, mouse_position: MousePosition, relative_mouse_position: MousePosition, mouse_event: MouseEvent, registrar: &mut MouseCallbackRegistrar) -> () {
        println!("Mouse event on WindowTopBarButton at abs{};{} rel {};{}", mouse_position.x_position, mouse_position.y_position, relative_mouse_position.x_position, relative_mouse_position.y_position);
        if mouse_event == MouseEvent::LMBDown {
            self.pressed.set(true);
        }
        /* Causes the error
        registrar.callbacks.push(Box::new(|mp, me|{
            if me == MouseEvent::LMBUp{
                self.pressed.set(false);
                return MouseQueueResult::DiscardMe
            }
            MouseQueueResult::KeepMe
        }));
         */

        println!("WTBB: pressed = {}", self.pressed.get())
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
    pub(crate) width: usize,
    pub(crate) height: usize,
    cache: Box<Vec<[u8;4]>>,
    cache_width: usize,
    cache_height: usize,
    needs_redraw : bool,
}

impl Widget for WindowWidget{
    fn render(&mut self, width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
        let topbar = self.window_top_bar.render(width, WINDOW_TOP_BAR_HEIGHT);
        let body = self.window_body.render(width, height - WINDOW_TOP_BAR_HEIGHT);
        if !self.needs_redraw && self.cache.len() > 0
            && self.cache_width == width && self.cache_height == height
            && topbar != None && body != None{

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
        //todo: add body
        let buf = widget::draw_on_top_at(
            0, 0,
            vec![[0u8, 0u8, 0u8, 255u8];width * height],
            width, height,
            &match topbar{
                Some(v) => v,
                None => self.window_top_bar.get_cache()
            },
            width, WINDOW_TOP_BAR_HEIGHT);
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

    fn handle_mouse_event(&mut self, mouse_position: MousePosition, relative_mouse_position: MousePosition, mouse_event: MouseEvent, registrar: &mut MouseCallbackRegistrar) -> () {
        if mouse_position.y_position < super::TOP_BAR_HEIGHT as f32{
            //mouse_position is within my top bar
            self.top_bar.handle_mouse_event(mouse_position, mouse_position, mouse_event, registrar);
        } else{
            //mouse_position is within the window proper
            if mouse_position.y_position < (self.y_position + WINDOW_TOP_BAR_HEIGHT) as f32{
                let rel_mouse_position = MousePosition{
                    x_position: mouse_position.x_position - self.x_position as f32,
                    y_position: mouse_position.y_position - self.y_position as f32,
                };
                self.window_top_bar.handle_mouse_event(mouse_position, rel_mouse_position, mouse_event, registrar);
            } else{
                let rel_mouse_position = MousePosition{
                    x_position: mouse_position.x_position - self.x_position as f32,
                    y_position: mouse_position.y_position - (self.y_position - WINDOW_TOP_BAR_HEIGHT) as f32,
                };
                self.window_body.handle_mouse_event(mouse_position, rel_mouse_position, mouse_event, registrar);
            }
        }
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
