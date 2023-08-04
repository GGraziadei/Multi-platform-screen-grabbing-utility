use egui::{Align, Button, Context, Layout, SidePanel, Vec2, Frame, Widget, Margin, hex_color, TopBottomPanel};
use egui_extras::RetainedImage;
use crate::window::Content;

impl Content {
	pub fn settings_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){

		_frame.set_window_size(Vec2::new(1000.0, 600.0));
    let bg_color = ctx.style().visuals.panel_fill;

		SidePanel::left("tabs")
			.frame(Frame{inner_margin: Margin::same(10.0), fill: bg_color, ..Default::default()})
			.resizable(false)
			.show(ctx, |ui| {
			ui.with_layout(Layout::top_down_justified(Align::Center), |ui|{
				let mut save_icon = RetainedImage::from_svg_bytes_with_size(
											"save",
											include_bytes!("../images/save_black.svg"),
											egui_extras::image::FitTo::Original).unwrap();
				ui.spacing_mut().item_spacing.y = 10.0;
				Button::image_and_text(save_icon.texture_id(ctx), Vec2::new(50.0,50.0), "Generale").rounding(8.0).fill(hex_color!("#2C5933")).ui(ui);
				Button::image_and_text(save_icon.texture_id(ctx), Vec2::new(50.0,50.0), "Salvataggio").rounding(8.0).ui(ui);
				Button::image_and_text(save_icon.texture_id(ctx), Vec2::new(50.0,50.0), "Scorciatoie").rounding(8.0).ui(ui);
			})
		});
		TopBottomPanel::top("settings")
			.resizable(false)
			.frame(Frame {fill: bg_color, ..Default::default()})
			.show(ctx, |ui| {
				ui.with_layout(Layout::top_down_justified(Align::Center), |ui|{
					ui.spacing_mut().item_spacing.y = 10.0;
					ui.heading("Impostazioni");
				})
			});
	}
}
