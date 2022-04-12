use std::collections::BTreeMap;
use crate::pixel_font::PixelFont;
use crate::widget;
use crate::widget::text_widget::TextWidget;
use crate::widget::{Color, Widget, WidgetBounds};
use crate::widget::mouse::{MouseCallbackRegistrar, MouseEvent, MousePosition};

///Widget representing buttons on the top bar/global menu.
pub struct TopBarButton{
    text: Box<TextWidget>,
    actions: Box<BTreeMap<Box<str>, Box<dyn Fn()>>>,
    opened: bool,
    cache: Box<Vec<[u8;4]>>,
    cache_width: usize,
    cache_height: usize,
    needs_redraw: bool,
}

impl TopBarButton{
    ///Create a new TopBarButton with the given label and fold-down actions.
    pub fn new(label: Box<&'static str>, actions: Box<BTreeMap<Box<str>, Box<dyn Fn()>>>) -> Self{
        let mut tpb = TopBarButton{
            text: Box::new(TextWidget::new(
                Box::new(PixelFont::default()),
                false,
                &label,
                Color::black(),
                Color::white(),
            )),
            actions,
            opened: false,
            cache: Box::new(vec![]),
            cache_width: 0,
            cache_height: 0,
            needs_redraw: true,
        };
        let bounds = tpb.get_min_bounds();
        tpb.cache_width = bounds.width;
        tpb.cache_height = bounds.height;
        let _ = match tpb.render(bounds.width, bounds.height){
            Some(v) => Box::new(v),
            None => panic!("Should not return None, since needs_redraw is true")
        };
        tpb
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
    fn render(&mut self, width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
        if !self.needs_redraw && self.cache.len() > 0{
            return None
        } else if self.cache.len() == 0{
            self.cache = Box::new(self.text.get_cache());
        }
        self.needs_redraw = false;
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

    fn get_cache(&mut self) -> Vec<[u8; 4]> {
        *self.cache.clone()
    }

    fn handle_mouse_event(&mut self, mouse_position: MousePosition, relative_mouse_position: MousePosition, mouse_event: MouseEvent, registrar: &mut MouseCallbackRegistrar) -> () {
        todo!()
    }
}

///Widget representing the top bar/global menu.
pub struct TopBarWidget {
    buttons: Box<Vec<Box<dyn Widget>>>,
    cache: Box<Vec<[u8;4]>>,
    cache_width: usize,
    cache_height: usize,
    needs_redraw: bool,
}

impl TopBarWidget {
    ///Create a new top bar with the given set of buttons. Takes Box<Vec<Box<dyn Widget>>> but
    /// should only be used with TopBarButton.
    pub fn new(buttons: Box<Vec<Box<dyn Widget>>>) ->Self{
        TopBarWidget {
            buttons,
            cache: Box::new(vec![]),
            cache_width: 0,
            cache_height: 0,
            needs_redraw: true,
        }
    }
}

impl Widget for TopBarWidget {
    fn render(&mut self, width: usize, height: usize) -> Option<Vec<[u8; 4]>> {
        if !self.needs_redraw && self.cache.len() > 0{
            return None
        }
        let mut buf = vec![[255u8;4]; width * height];
        let mut button_widths :Vec<usize> = Vec::new();
        let mut button_height = 0usize;
        let mut button_bufs : Vec<Vec<[u8;4]>> = Vec::new();
        for button in &mut *self.buttons{
            let bounds: WidgetBounds = button.get_min_bounds();
            button_bufs.push(match button.render(bounds.width, bounds.height){
                Some(v) => v,
                None => button.get_cache()});
            button_widths.push(bounds.width);
            button_height = bounds.height;
        }
        let mut xoff = 10;
        let yoff = 7;
        for (idx, button_buf) in button_bufs.iter().enumerate(){
            buf = widget::draw_on_top_at(
                xoff, yoff,
                buf, width, height,
                &button_buf, button_widths[idx], button_height);
            xoff += button_widths[idx] + 20;
        }
        //add line at the bottom of the top bar
        for i in 0..width{
            buf[(height - 1) * width + i] = [0u8, 0u8, 0u8, 255u8];
        }
        self.cache_width = width;
        self.cache_height = height;
        self.needs_redraw = false;
        self.cache = Box::new(buf.clone());
        Some(buf)
    }
    fn get_children(&self) -> Option<&Box<Vec<Box<dyn Widget>>>> {
        Some(&self.buttons)
    }
    fn get_min_bounds(&self) -> WidgetBounds {
        todo!()
    }

    fn get_cache(&mut self) -> Vec<[u8; 4]> {
        *self.cache.clone()
    }

    fn handle_mouse_event(&mut self, mouse_position: MousePosition, relative_mouse_position: MousePosition, mouse_event: MouseEvent, registrar: &mut MouseCallbackRegistrar) -> () {
        println!("Handling top bar mouse event, abs x={};y={}; rel x={};y={}", mouse_position.x_position, mouse_position.y_position, relative_mouse_position.x_position, relative_mouse_position.y_position);

    }
}
