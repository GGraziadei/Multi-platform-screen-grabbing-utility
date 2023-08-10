use egui::{Context, Id, pos2};
use crate::window::Content;

impl Content{
	pub fn drawing_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
        let monitor_size = _frame.info().window_info.monitor_size.unwrap();
        _frame.set_window_size(monitor_size);
        match ctx.memory(|mem| mem.data.get_temp::<bool>(Id::from("first_moved"))){
            Some(_) => {}
            None => {
                _frame.set_window_pos(pos2(0.0, 0.0));
                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("first_moved"), true));
            }
        };
    }
}
