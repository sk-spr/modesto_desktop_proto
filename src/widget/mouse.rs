#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MouseEvent{
    LMBDown,
    LMBUp,
    RMBDown,
    RMBUp,
    ScrollDown,
    ScrollUp,
    ScrollClick,
}
#[derive(Clone, Copy)]
pub struct MousePosition{
    pub x_position: f32,
    pub y_position: f32,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MouseQueueResult{
    KeepMe,
    DiscardMe
}
pub struct MouseCallbackRegistrar{
    pub callbacks: Vec<Box<dyn FnMut(MousePosition, MouseEvent) -> MouseQueueResult>>,
}
