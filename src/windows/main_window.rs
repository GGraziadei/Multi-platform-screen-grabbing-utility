use eframe::emath::{Align, Vec2};
use egui::{Context, Direction, Frame, Id, Layout, Margin, RichText, SidePanel, TopBottomPanel};
use crate::window::{Content};
use crate::window::WindowType::*;

impl Content {
	pub fn main_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
    let bg_color = ctx.style().visuals.panel_fill;

    _frame.set_decorations(true);

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
              self.current_screen(ctx, _frame);
            };
            if ui.button("Tutti gli schermi").clicked(){
              self.all_screens(ctx, _frame);
            };
            if ui.button("Regione rettangolare").clicked(){
              self.select(ctx, _frame);
            };
            if ui.button("Impostazioni").clicked(){
              self.set_win_type(Settings);
            };
          }
        );
      });
	}
}
