use std::sync::{Arc, Mutex, RwLock};
use std::thread::{JoinHandle, spawn};
use eframe::emath::{Align, Vec2};
use eframe::epaint::hex_color;
use egui::{Direction, Frame, Key, Layout, Margin, RichText, ScrollArea, SidePanel, TopBottomPanel};
use eframe::Result;
use egui::accesskit::Role::Image;
use screenshots::DisplayInfo;
use crate::configuration::{Configuration, ImageFmt};
use crate::image_formatter::{EncoderThread, ImageFormatter};
use crate::screenshots::ScreenshotExecutor;

struct Content {
    configuration : Arc<RwLock<Configuration>>,
    screenshot_executor : ScreenshotExecutor,
    encoders : Arc<Mutex<Vec<EncoderThread>>>,
    text: String,
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let window_size = _frame.info().window_info.size;

        TopBottomPanel::top("top")
          .frame(Frame{fill: hex_color!("#2B2D30"), ..Default::default()})
          .show_separator_line(false)
          .resizable(false)
          .min_height(window_size.y*0.2)
          .show(ctx, |ui| {
            ui.allocate_ui_with_layout(
              Vec2::new(window_size.x, window_size.y*0.2),
              Layout::centered_and_justified(Direction::TopDown),
              |ui| {
                ui.heading("Acquisisci una nuova schermata")
              }
            );
          });
        TopBottomPanel::top("bottom")
          .frame(Frame{fill: hex_color!("#2B2D30"), ..Default::default()})
          .show_separator_line(false)
          .resizable(false)
          .min_height(window_size.y*0.8)
          .show(ctx, |ui| {
            let w = 0.3;
            SidePanel::left("left")
              .frame(Frame{inner_margin: Margin::same(20.0), fill: hex_color!("#2B2D30"), ..Default::default()})
              .show_separator_line(false)
              .resizable(false)
              .exact_width(window_size.x*w)
              .show(ctx, |ui| {
                ui.allocate_ui_with_layout(
                  Vec2::new(window_size.x*w, window_size.y*0.8),
                  Layout::top_down_justified( Align::Center),
                  |ui| {
                    ui.label(RichText::new("Modalità di acquisizione").size(16.0));
                    ui.add_space(10.0);
                    ui.spacing_mut().button_padding = Vec2::new(10.0, 10.0);
                    ui.spacing_mut().item_spacing.y = 10.0;
                    if ui.button("Regione rettangolare").clicked(){};
                    if ui.button("Tutti gli schermi").clicked(){};
                    if ui.button("Schermo attuale").clicked(){};
                    if ui.button("Finestra attiva").clicked(){};
                    if ui.button("Finestra sotto al cursore").clicked(){};
                  }
                );
              });
            SidePanel::right("right")
              .frame(Frame{inner_margin: Margin::same(20.0), fill: hex_color!("#2B2D30"), ..Default::default()})
              .show_separator_line(false)
              .resizable(false)
              .exact_width(window_size.x*(1.0-w))
              .show(ctx, |ui| {
                ui.allocate_ui_with_layout(
                Vec2::new(window_size.x*(1.0-w), window_size.y*0.8),
                Layout::top_down( Align::LEFT),
                |ui| {
                    ui.spacing_mut().item_spacing.y = 10.0;
                    ui.label(RichText::new("Opzioni di acquisizione").size(16.0));
                    ui.checkbox(&mut true, "Includi il puntatore del mouse");
                    ui.checkbox(&mut true, "Includi la barra del titolo e i bordi della finestra");
                    ui.checkbox(&mut true, "Cattura solo la finestra attuale");
                    ui.checkbox(&mut true, "Esci dopo il salvataggio o la copia manuali");
                    ui.checkbox(&mut true, "Cattura al click");
                    let painter = ui.painter();
                  }
                );
              });
          });
    }
}

/*Marker struct. The event loop is executed over the main thread.*/
pub struct GuiThread;

impl GuiThread {

    pub fn new(configuration: Arc<RwLock<Configuration>>, encoders: Arc<Mutex<Vec<EncoderThread>>>, s : ScreenshotExecutor) -> Self
    {
        let configuration_read = configuration.read()
            .expect("Error. Cannot run gui thread without configuration file.");

        let app_name_tmp = configuration_read.get_app_name().unwrap().clone();

        drop(configuration_read);

        let options = eframe::NativeOptions{
            resizable: false,
            follow_system_theme: true,
            initial_window_size: Some(egui::Vec2::new(640.0, 360.0)),
            ..Default::default()
        };

        let content = Content{
            configuration,
            screenshot_executor: s,
            encoders,
            text: "".to_string(),
        };

        eframe::run_native(
            &*app_name_tmp,
            options,
            Box::new(move |_cc| Box::<Content>::new(content)),
        ).expect("Error during gui thread init.");

        Self
    }

}

