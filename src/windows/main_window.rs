use std::thread::current;
use eframe::emath::{Align, Vec2};
use egui::{Context, Direction, Frame, Id, Layout, Margin, RichText, SidePanel, TopBottomPanel};
use screenshots::{Compression, DisplayInfo};
use crate::draw_window::{Content};
use crate::draw_window::WindowType::*;
use crate::image_combiner::ImageCombiner;
use crate::screenshots::CaptureArea;

impl Content {
	pub fn main_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
    let window_size = _frame.info().window_info.size;
    let bg_color = ctx.style().visuals.panel_fill;

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
      .frame(Frame{fill: bg_color, ..Default::default()})
      .show_separator_line(false)
      .resizable(false)
      .show(ctx, |ui| {
        let w = 0.3;
        SidePanel::left("left")
          .frame(Frame{inner_margin: Margin::symmetric(20.0, 0.0), fill: bg_color, ..Default::default()})
          .show_separator_line(false)
          .resizable(false)
          .exact_width(window_size.x*w)
          .show(ctx, |ui| {
            ui.with_layout(
              Layout::top_down_justified( Align::Center),
              |ui| {
                ui.label(RichText::new("Modalità di acquisizione").size(16.0));
                ui.add_space(10.0);
                ui.spacing_mut().button_padding = Vec2::new(10.0, 10.0);
                ui.spacing_mut().item_spacing.y = 10.0;
                if ui.button("Regione rettangolare").clicked(){
                  self.select(ctx, _frame);
                };
                if ui.button("Tutti gli schermi").clicked(){
                  self.all_screens(ctx, _frame);
                };
                if ui.button("Schermo attuale").clicked(){
                  self.current_screen(ctx, _frame);
                };
                if ui.button("Finestra attiva").clicked(){};
                if ui.button("Finestra sotto al cursore").clicked(){};
              }
            );
          });
        SidePanel::right("right")
          .frame(Frame{inner_margin: Margin::symmetric(20.0, 0.0), fill: bg_color, ..Default::default()})
          .show_separator_line(false)
          .resizable(false)
          .exact_width(window_size.x*(1.0-w))
          .show(ctx, |ui| {
            ui.with_layout(
              Layout::top_down( Align::LEFT),
              |ui| {
                ui.spacing_mut().item_spacing.y = 10.0;
                ui.label(RichText::new("Opzioni di acquisizione").size(16.0));
                ui.checkbox(&mut true, "Includi il puntatore del mouse");
                ui.checkbox(&mut true, "Includi la barra del titolo e i bordi della finestra");
                ui.checkbox(&mut true, "Cattura solo la finestra attuale");
                ui.checkbox(&mut true, "Esci dopo il salvataggio o la copia manuali");
                ui.checkbox(&mut true, "Cattura al click");
              });
          });
      });
	}

  pub fn current_screen(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
    let mut di = self.get_current_screen_di(_frame);
    let screenshot = self.get_se().screenshot(di, None, CaptureArea::new(0,0, di.width, di.height)).unwrap();
    let imgf = screenshot.to_png(Some(Compression::Best)).unwrap();
    ctx.memory_mut(|mem|{
      mem.data.insert_temp(Id::from("screenshot"), imgf);
      self.set_win_type(Screenshot);
    });
  }

  pub fn select(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
    let mut di = self.get_current_screen_di(_frame);
    let screenshot = self.get_se().screenshot(di, None, CaptureArea::new(0,0, di.width, di.height)).unwrap();
    let imgf = screenshot.to_png(Some(Compression::Best)).unwrap();
    ctx.memory_mut(|mem|{
      mem.data.insert_temp(Id::from("screenshot"), imgf);
      self.set_win_type(Select);
    });
  }

  pub fn all_screens(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
    let images = self.get_se().screenshot_all(None);
    let screenshot = ImageCombiner::combine(images.unwrap()).unwrap();
    let imgf = screenshot.to_png(Some(Compression::Best)).unwrap();
    ctx.memory_mut(|mem|{
      mem.data.insert_temp(Id::from("screenshot"), imgf);
      self.set_win_type(Screenshot);
    });
  }

  pub fn get_current_screen_di(&mut self, _frame: &mut eframe::Frame) -> DisplayInfo {
    let mut di = DisplayInfo::from_point(0,0).unwrap();
    for (i, display) in DisplayInfo::all().unwrap().iter().enumerate(){
      let frame_pos = _frame.info().window_info.position.unwrap();
      if
         (display.x < frame_pos.x as i32) &&
         (frame_pos.x as i32) < (display.x + display.width as i32) &&
         (display.y < frame_pos.y as i32) &&
         (frame_pos.y as i32) < (display.y + display.height as i32)
      {
        di = display.clone();
        break;
      }
    }
    di
  }
}
