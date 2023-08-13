use eframe::emath::{Align, Vec2};
use egui::{CentralPanel, Context, Direction, Frame, Layout, Margin, TopBottomPanel};
use crate::configuration::AcquireMode;
use crate::window::{Content};
use crate::window::WindowType::*;

impl Content {
    pub fn main_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
        let bg_color = ctx.style().visuals.panel_fill;
		let config = self.configuration.read().unwrap();
		
		let delay = match config.get_delay() {
			Some(d) => d.as_secs(),
			None => 0
		};
		
		drop (config);
        
        _frame.set_decorations(true);
        _frame.set_window_size(Vec2::new(350.0, 300.0));
        
        TopBottomPanel::top("top")
            .frame(Frame{fill: bg_color, inner_margin: Margin { bottom: match delay { 0 => 20.0, _ => 10.0}, ..Margin::same(20.0) }, ..Default::default()})
            .show_separator_line(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.with_layout(
                    Layout::centered_and_justified(Direction::TopDown),
                    |ui| {
                        ui.heading("Acquisisci una nuova schermata")
                    }
                );
            });
        CentralPanel::default()
            .frame(Frame{fill: bg_color, inner_margin: Margin {bottom: 20.0, left: 80.0, right: 80.0, top: 0.0}, ..Default::default()})
            .show(ctx, |ui| {
                ui.with_layout(
                    Layout::top_down_justified( Align::Center),
                    |ui| {
                        ui.spacing_mut().button_padding = Vec2::new(10.0, 10.0);
                        ui.spacing_mut().item_spacing.y = 10.0;
						
						if delay > 0 {
							ui.label(format!("Ritardo: {} {}", delay, match delay { 1 => "secondo", _ => "secondi"}));
						}
						
                        if ui.button("Schermo attuale").clicked(){
                            _frame.set_visible(false);
                            self.set_acquire_mode(Some(AcquireMode::CurrentScreen));
                        };
                        if ui.button("Selziona schermo").clicked(){
                            _frame.set_visible(false);
                            self.set_acquire_mode(Some(AcquireMode::SelectScreen));
                        };
                        if ui.button("Tutti gli schermi").clicked(){
                            _frame.set_visible(false);
                            self.set_acquire_mode(Some(AcquireMode::AllScreens));
                            
                        };
                        if ui.button("Regione rettangolare").clicked(){
                            _frame.set_visible(false);
                            self.set_acquire_mode(Some(AcquireMode::Portion));
                        };
                        if ui.button("Impostazioni").clicked(){
                            self.set_win_type(Settings);
                        };
                    }
                );
            });
    }
}
