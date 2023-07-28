use std::default::Default;
use eframe::IntegrationInfo;
use egui::*;

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
		let panel_frame = egui::Frame{
			inner_margin: Default::default(),
			outer_margin: Default::default(),
			rounding: Default::default(),
			shadow: Default::default(),
			fill: Color32::GOLD,
			stroke: Default::default(),
		};



		Window::new("left").frame(panel_frame).default_width(window_size.x*0.5).show(ctx, |ui|{
			ui.label("left");
		});
		Window::new("right").frame(panel_frame).default_width(window_size.x*0.5).show(ctx, |ui|{
			ui.label("right");
		});
	}
}

pub fn simple_window(){
  let native_options = eframe::NativeOptions::default();
  eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc)))).unwrap();
}
