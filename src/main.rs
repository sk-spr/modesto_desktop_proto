use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use lazy_static::lazy_static;
use desktop_minifb::widget::Widget;


const WIDTH : usize = 513;
const HEIGHT: usize = 343;

lazy_static!{
    static ref FRAMEBUFFER : Mutex<Vec<[u8; 3]>> = Mutex::new(vec![[0u8;3]; WIDTH * HEIGHT]);
}

fn main() {
    let mut window = Window::new(
        "DESKTOP",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: minifb::Scale::X2,
            scale_mode: minifb::ScaleMode::Stretch,
            topmost: false,
            transparency: false,
            none: false
        },
    ).unwrap_or_else(|e| panic!("failed unwrapping window, error: {}", e));
    /*for (idx, elem) in buffer.iter_mut().enumerate(){
        *elem = converted[idx];
    }*/
    window.limit_update_rate(Some(Duration::from_micros(16666)));
    let mut x_off = 0;
    let mut main_widget = desktop_minifb::widget::MainWidget::new(WIDTH, HEIGHT);

    let mut window1 = desktop_minifb::widget::window::WindowWidget::new("Title", 400, 200, 50, 50);
    window1.register_top_bar(Box::new(desktop_minifb::widget::top_bar::TopBarWidget::new(
        Box::new(vec![
            Box::new(desktop_minifb::widget::top_bar::TopBarButton::new(
                Box::new("{}"), Box::new(BTreeMap::new()),
            )),
            Box::new(desktop_minifb::widget::top_bar::TopBarButton::new(
            Box::new("Button"), Box::new(BTreeMap::new()))),
            Box::new(desktop_minifb::widget::top_bar::TopBarButton::new(
              Box::new("Second Button"), Box::new(BTreeMap::new()),
              )),

        ])
    )));
    main_widget.reg_window(Box::new(window1));
    while window.is_open() && !(window.is_key_down(Key::LeftAlt) && window.is_key_down(Key::F4)){
        //MAIN LOOP - FUTURE: IN USERSPACE PROGRAM
        if x_off == 0{
            main_widget.windows[0].set_moving(true);
        }
        if x_off == 60{
            main_widget.windows[0].set_moving(false);
        }
        x_off = (x_off + 1) % 120;

        if x_off < 60 {
            main_widget.windows[0].x_position = 50 + x_off;
            main_widget.windows[0].y_position = 50 + x_off;
        } else {
            main_widget.windows[0].x_position = 110;
            main_widget.windows[0].y_position = 110;
        }
        let newfb = main_widget.render(WIDTH, HEIGHT);
        //draw to buffer - REPLACE
        let converted:Vec<u32> = newfb.iter().map(compute_col_u32_alpha).collect();//FRAMEBUFFER.lock().unwrap().iter().map(compute_col_u32.no_alpha).collect();
        //draw buffer to screen - REPLACE
        window.update_with_buffer(&converted, WIDTH, HEIGHT).unwrap();
    }
}
///Combines the u8 components (order RGB) and 255 into a 32 bit unsigned integer (order: ARGB)
fn compute_col_u32_no_alpha(components: &[u8; 3])-> u32{
    ((255 as u32) << 24) | ((components[0] as u32) << 16) | ((components[1] as u32) << 8) | (components[2] as u32)
}
///Combines the components (order: RGBA) into a u32 (order: ARGB)
fn compute_col_u32_alpha(components: &[u8; 4]) -> u32 {
    ((components[3] as u32) << 24) | ((components[0] as u32) << 16) | ((components[1] as u32) << 8) | (components[2] as u32)
}
