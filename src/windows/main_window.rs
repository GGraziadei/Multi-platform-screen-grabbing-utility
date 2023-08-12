use eframe::emath::{Align, Vec2};
use egui::{CentralPanel, Context, Direction, Frame, Layout, Margin, TopBottomPanel};
use crate::configuration::AcquireMode;
use crate::window::{Content};
use crate::window::WindowType::*;

impl Content {
	pub fn main_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
    let bg_color = ctx.style().visuals.panel_fill;

        match self.get_acquire_mode() {
            None => {}
            Some(am) => {
                match am {
                    AcquireMode::CurrentScreen => {
                        self.set_acquire_mode(None);
                        self.current_screen(ctx, _frame);
                    }
                    AcquireMode::SelectScreen => {
                        self.set_acquire_mode(None);
                        self.select_screen(ctx, _frame);
                    }
                    AcquireMode::AllScreens => {
                        self.set_acquire_mode(None);
                        self.all_screens(ctx, _frame);
                    }
                    AcquireMode::Portion => {
                        self.set_acquire_mode(None);
                        self.portion(ctx, _frame);
                    }
                }
            }
        }

    _frame.set_decorations(true);
    _frame.set_window_size(Vec2::new(350.0, 300.0));

    TopBottomPanel::top("top")
      .frame(Frame{fill: bg_color, inner_margin: Margin::symmetric(0.0, 20.0), ..Default::default()})
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
      .frame(Frame{fill: bg_color, inner_margin: Margin {bottom: 20.0, left: 80.0, right: 80.0, ..Default::default()}, ..Default::default()})
      .show(ctx, |ui| {
        ui.with_layout(
          Layout::top_down_justified( Align::Center),
          |ui| {
            ui.spacing_mut().button_padding = Vec2::new(10.0, 10.0);
            ui.spacing_mut().item_spacing.y = 10.0;

            if ui.button("Schermo attuale").clicked(){
                _frame.set_visible(false);
                self.set_acquire_mode(Some(AcquireMode::CurrentScreen));
                //self.current_screen(ctx, _frame);
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
