use crate::main_menu::mainmenu::Message;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::mint::Point2;
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub(crate) struct Button {
    pub(crate) text: String,
    pub(crate) img: Option<graphics::Image>,
    pub(crate) message: Message,
    pub(crate) sender: Sender<Message>,
    pub(crate) rect: graphics::Rect,
    pub(crate) color: Color,
}

impl Button {
    fn pressed(&self) {
        dbg!("Pressed {:?}", self.message);
    }

    fn is_clicked(&self, mouse_pos: Point2<f32>) -> bool {
        self.rect.contains(mouse_pos)
    }
    pub(crate) fn click(&mut self, mouse_pos: Point2<f32>) {
        if self.is_clicked(mouse_pos) {
            self.pressed();
            self.sender.send(self.message).unwrap();
        }
    }
}
