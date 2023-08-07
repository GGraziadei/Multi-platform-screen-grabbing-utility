use eframe::emath::{Align, Vec2};
use egui::{CentralPanel, Color32, ColorImage, Context, Frame, Id, LayerId, Layout, Margin, Order, RichText, TopBottomPanel};
use egui_extras::RetainedImage;
use crate::window::Content;

impl Content {
	pub fn select_screen_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
		let mut first_render = true;
    let bg_color = ctx.style().visuals.panel_fill;
		let margin = 20.0;
		let mut images = vec![];

		match ctx.memory(|mem| { mem.data.get_temp::<Vec<Vec<u8>>>(Id::from("r_bytes"))}){
			Some(bytes) => {
				for b in bytes {
					let r_image = RetainedImage::from_image_bytes(
						"screenshot",
						b.as_slice()
					).unwrap();
					images.push(r_image);
				}
			},
			None => images.push(RetainedImage::from_color_image("screenshot", ColorImage::example()))
		};

		_frame.set_window_size(Vec2::new(800.0, 500.0));

		ctx.memory(|mem|{
			let res = mem.data.get_temp::<bool>(Id::from("first_render"));
			if res.is_some() {
				first_render = res.unwrap();
			}
		});

		if first_render {
			_frame.set_centered();
			ctx.memory_mut(|mem| {
				mem.data.insert_temp(Id::from("first_render"), false);
			});
		}

		CentralPanel::default()
      .frame(Frame{fill: bg_color, inner_margin: Margin::same(margin), ..Default::default()})
      .show(ctx, |ui| {
				ui.with_layout(Layout::top_down(Align::Center), |ui|{
					ui.heading(RichText::new("Seleziona lo schermo").size(24.0));
					ui.add_space(60.0);
					ui.with_layout(Layout::left_to_right(Align::LEFT), |ui|{
						ui.spacing_mut().item_spacing.x = 10.0;
						let window_width = ui.available_width();
						for r_image in images {
							let image_width = (window_width - 20.0)/3.0;
							let image_height = r_image.height() as f32 /r_image.width() as f32 * (image_width);
							let mut painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::from("1")));
							// painter.set_clip_rect(ui.available_rect_before_wrap());
							// painter.rect_filled(ui.available_rect_before_wrap(), 0.0, Color32::from_rgb(255, 0, 0));

							ui.image(r_image.texture_id(ctx), Vec2::new(image_width, image_height));
						}
						// let mut painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::from("1")));
						// painter.set_clip_rect(ui.available_rect_before_wrap());
						// painter.rect_filled(ui.available_rect_before_wrap(), 0.0, Color32::from_rgb(255, 0, 0));

						// ui.image(r_image.texture_id(ctx), Vec2::new(image_width, image_height));
						// ui.image(r_image.texture_id(ctx), Vec2::new(image_width, image_height));
						// ui.image(r_image.texture_id(ctx), Vec2::new(image_width, image_height));
					})
				})
			});

	}

}
