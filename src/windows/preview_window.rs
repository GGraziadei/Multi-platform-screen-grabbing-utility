use eframe::Theme;
use egui::{Align, Button, Color32, ColorImage, Context, Direction, Frame, hex_color, Id, Image, LayerId, Layout, Margin, Order, pos2, Rect, RichText, SidePanel, TopBottomPanel, Vec2, Widget};
use egui_extras::RetainedImage;
use crate::window::Content;
use crate::window::WindowType::Settings;

impl Content {
	pub fn screenshot_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
		let window_size = _frame.info().window_info.size;
    let bg_color = ctx.style().visuals.panel_fill;
		let w = 0.6;
		let margin = 20.0;
		let mut first_render = true;
		let mut r_image = RetainedImage::from_color_image("screenshot", ColorImage::example());
		let mut screenshot_ok = false;

		_frame.set_window_size(Vec2::new(1000.0, 600.0));
		_frame.set_fullscreen(false);
		_frame.set_decorations(true);

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

		if self.get_colorimage().is_some(){
			r_image = RetainedImage::from_color_image("screenshot", self.get_colorimage().clone().unwrap());
			screenshot_ok = true;
		}
		else {

			let fast_bytes = self.get_gui_screenshot_data().unwrap().img_bytes_fast;
			r_image = RetainedImage::from_image_bytes(
				"screenshot",
				fast_bytes.as_slice()
			).unwrap();
			screenshot_ok = true;

			// ctx.memory(|mem|{
			// 	let fast_bytes = mem.data.get_temp::<Vec<u8>>(Id::from("bytes"));
			// 	if fast_bytes.is_some(){
			// 		r_image = RetainedImage::from_image_bytes(
			// 			"screenshot",
			// 			fast_bytes.unwrap().as_slice()
			// 		).unwrap();
			// 		screenshot_ok = true;
			// 	}
			// });
		}

    TopBottomPanel::top("top")
      .frame(Frame{fill: bg_color, inner_margin: Margin::same(margin), ..Default::default()})
      .show_separator_line(false)
      .resizable(false)
      .show(ctx, |ui| {
        ui.with_layout(
					Layout::left_to_right(Align::LEFT),
					|ui| {
						ui.spacing_mut().button_padding = Vec2::new(10.0, 10.0);
						ui.spacing_mut().item_spacing.x = 10.0;
						let icon_size = Vec2::new(16.0,16.0);

						let mut save_icon = RetainedImage::from_svg_bytes_with_size(
									"save",
									include_bytes!("../images/save_black.svg"),
									egui_extras::image::FitTo::Original).unwrap();
						let mut save_as_icon = RetainedImage::from_svg_bytes_with_size(
								"save_as",
								include_bytes!("../images/save_as_black.svg"),
								egui_extras::image::FitTo::Original).unwrap();
						let mut copy_icon = RetainedImage::from_svg_bytes_with_size(
								"copy",
								include_bytes!("../images/copy_black.svg"),
								egui_extras::image::FitTo::Original).unwrap();
						let mut edit_icon = RetainedImage::from_svg_bytes_with_size(
								"edit",
								include_bytes!("../images/edit_black.svg"),
								egui_extras::image::FitTo::Original).unwrap();

						if _frame.info().system_theme.is_none() || _frame.info().system_theme.unwrap() == Theme::Dark{
							save_icon = RetainedImage::from_svg_bytes_with_size(
									"save",
									include_bytes!("../images/save_white.svg"),
									egui_extras::image::FitTo::Original).unwrap();
							save_as_icon = RetainedImage::from_svg_bytes_with_size(
									"save_as",
									include_bytes!("../images/save_as_white.svg"),
									egui_extras::image::FitTo::Original).unwrap();
							copy_icon = RetainedImage::from_svg_bytes_with_size(
									"copy",
									include_bytes!("../images/copy_white.svg"),
									egui_extras::image::FitTo::Original).unwrap();
							edit_icon = RetainedImage::from_svg_bytes_with_size(
									"edit",
									include_bytes!("../images/edit_white.svg"),
									egui_extras::image::FitTo::Original).unwrap();
						}
						if ui.add(
							Button::image_and_text(
								save_icon.texture_id(ctx),
								icon_size,
								"Salva")).clicked()
						{
							self.save_image(ctx, None);
						}
						if ui.add(
							Button::image_and_text(
								save_as_icon.texture_id(ctx),
								icon_size,
								"Salva come...")).clicked()
						{
							let path = native_dialog::FileDialog::new()
								.add_filter("png", &["png"])
								.add_filter("jpg", &["jpg"])
								.add_filter("gif", &["gif"])
								.show_save_single_file().unwrap();
							if path.is_some(){
								self.save_image(ctx, path);
							}
						}
						if ui.add(
							Button::image_and_text(
								copy_icon.texture_id(ctx),
								icon_size,
								"Copia")).clicked(){
							self.copy_image(ctx);
						}
						if ui.add(
							Button::image_and_text(
								edit_icon.texture_id(ctx),
								icon_size,
								"Annota")).clicked(){}
					}
				);
      });
		SidePanel::left("left")
			.frame(Frame{inner_margin: Margin::same(20.0), fill: bg_color, ..Default::default()})
			.show_separator_line(false)
			.resizable(false)
			.exact_width(window_size.x*w)
			.show(ctx, |ui| {
				ui.with_layout(
					Layout::top_down_justified( Align::Center),
					|ui| {
						let mut painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("screenshot")));
						let size_x = ui.available_width();
						let size_y = ui.available_height();
						let painter_rect = Rect::from_min_size(pos2(margin,window_size.y-size_y - margin), Vec2::new(size_x, size_y));

						painter.set_clip_rect(painter_rect);
						// painter.rect_filled(painter_rect, 0.0, Color32::RED);

						if screenshot_ok {
							let uv_rect = Rect::from_min_max(pos2(0.0,0.0), pos2(1.0,1.0));
							let mut image_rect = Rect::from_min_size(pos2(0.0,0.0), Vec2::new(0.0,0.0));
							if r_image.width() > r_image.height(){
								let image_rect_w = size_x;
								let image_rect_h = image_rect_w/r_image.width() as f32 * r_image.height() as f32;
								let start_y = (window_size.y-size_y) - margin + (size_y/2.0) - (image_rect_h/2.0);
								// let start_y = (window_size.y-size_y) - margin;
								image_rect = Rect::from_min_size(pos2(margin,start_y), Vec2::new(image_rect_w, image_rect_h));
							}
							else {
								let image_rect_h = size_y;
								let image_rect_w = image_rect_h/r_image.height() as f32 * r_image.width() as f32;
								let start_x = margin + (size_x/2.0) - (image_rect_w/2.0);
								let start_y = (window_size.y-size_y) - margin;
								image_rect = Rect::from_min_size(pos2(start_x,start_y), Vec2::new(image_rect_w, image_rect_h));
							}
							painter.image(r_image.texture_id(ctx), image_rect, uv_rect, Color32::WHITE);
						}
					}
				);
			});
		SidePanel::right("right")
			.frame(Frame{inner_margin: Margin::same(margin), fill: bg_color, ..Default::default()})
			.show_separator_line(false)
			.resizable(false)
			.exact_width(window_size.x*(1.0-w))
			.show(ctx, |ui| {
				ui.with_layout(
					Layout::top_down( Align::LEFT),
					|ui| {
						ui.spacing_mut().item_spacing.y = 10.0;
						ui.label(RichText::new("Acquisisci una nuova schermata").size(16.0));
						ui.spacing_mut().button_padding = Vec2::new(10.0, 10.0);
						if ui.button("Regione rettangolare").clicked(){
							// self.select(ctx, _frame);
						};
						if ui.button("Tutti gli schermi").clicked(){
							// self.all_screens(ctx, _frame);
						};
						if ui.button("Schermo attuale").clicked(){
							// self.current_screen(ctx, _frame);
						};
						if ui.button("Impostazioni").clicked(){
              self.set_win_type(Settings);
            };
					});
			});
	}
}

