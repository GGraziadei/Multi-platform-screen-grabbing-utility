use std::thread;
use std::time::Duration;
use eframe::emath::{Align, Vec2};
use egui::{Context, Direction, Frame, Id, Layout, Margin, RichText, SidePanel, TopBottomPanel};
use crate::configuration::AcquireMode;
use crate::configuration::AcquireMode::{AllScreen, CurrentScreen, DragDrop, SelectScreen};
use crate::window::{Content};
use crate::window::WindowType::Settings;

impl Content {
	pub fn main_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
    let bg_color = ctx.style().visuals.panel_fill;

    _frame.set_decorations(true);
    _frame.set_window_size(Vec2::new(350.0, 290.0));

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
    TopBottomPanel::top("bottom")
      .frame(Frame{fill: bg_color, inner_margin: Margin {bottom: 20.0, left: 80.0, right: 80.0, ..Default::default()}, ..Default::default()})
      .show_separator_line(false)
      .resizable(false)
      .show(ctx, |ui| {
        ui.with_layout(
          Layout::top_down_justified( Align::Center),
          |ui| {
            ui.spacing_mut().button_padding = Vec2::new(10.0, 10.0);
            ui.spacing_mut().item_spacing.y = 10.0;

            if ui.button("Schermo attuale").clicked(){
              _frame.set_visible(false);
              self.set_acquire(Some(CurrentScreen));
            };
            if ui.button("Tutti gli schermi").clicked(){
              _frame.set_visible(false);
              self.set_acquire(Some(AllScreen));
            };
            if ui.button("Seleziona lo schermo").clicked(){
              _frame.set_visible(false);
              self.set_acquire(Some(SelectScreen));
            };
            if ui.button("Regione rettangolare").clicked(){
              _frame.set_visible(false);
              self.set_acquire(Some(DragDrop));
            };
            if ui.button("Impostazioni").clicked(){
              self.set_win_type(Settings);
            };
          }
        );
      });
	}
}
