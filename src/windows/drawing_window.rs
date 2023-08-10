use eframe::emath::Vec2;
use egui::{Context, pos2};
use crate::window::Content;

impl Content{
	pub fn drawing_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
        let di = _frame.info().window_info.monitor_size;
        _frame.set_window_size(di.unwrap());
        _frame.set_window_pos(pos2(0.0,0.0));
        println!("{:?}", di);
    }
}
