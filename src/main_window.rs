use std::default::Default;
use eframe::run_native;
use egui::*;
use egui::{RichText};

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
	fn new(cc: &eframe::CreationContext<'_>) -> Self {
    // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
    // Restore app state using cc.storage (requires the "persistence" feature).
    // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
    // for e.g. egui::PaintCallback.
    Self::default()
  }
}

impl eframe::App for MyEguiApp {
  fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
		let window_size = frame.info().window_info.size;
		frame.set_window_title("MPSGU");


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

							}
						);
					});
			});
	}
}

pub fn simple_window(){
  let mut native_options = eframe::NativeOptions::default();
	native_options.resizable = false;
	native_options.follow_system_theme = true;
	native_options.initial_window_size = Some(egui::Vec2::new(640.0, 360.0));
  eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc)))).unwrap();
}
