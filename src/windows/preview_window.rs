use eframe::Theme;
use egui::{Align, Button, Color32, ColorImage, Context,Frame, Id, LayerId, Layout, Margin, Order, pos2, Rect, RichText, SidePanel, TopBottomPanel, Vec2};
use egui_extras::RetainedImage;
use crate::configuration::AcquireMode;
use crate::window::Content;
use crate::window::WindowType::{Drawing, Settings};
use crate::windows::drawing_window::Drawings;

impl Content {
	pub fn screenshot_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
		let window_size = _frame.info().window_info.size;
		let bg_color = ctx.style().visuals.panel_fill;
		let w = 0.7;
		let margin = 20.0;
		let mut r_image = RetainedImage::from_color_image("screenshot", ColorImage::example());
		let mut screenshot_ok = false;

		_frame.set_window_size(Vec2::new(1000.0, 550.0));
		_frame.set_visible(true);
		_frame.set_fullscreen(false);
		_frame.set_decorations(true);

		ctx.memory_mut(|mem| mem.data.remove::<Vec<Drawings>>(Id::from("drawings")));
		ctx.memory_mut(|mem| mem.data.remove::<Vec<Drawings>>(Id::from("drawings")));

		if self.get_color_image().is_some(){
			r_image = RetainedImage::from_color_image("screenshot", self.get_color_image().clone().unwrap());
			screenshot_ok = true;
		}
		else {
			ctx.memory(|mem|{
				let fast_bytes = mem.data.get_temp::<Vec<u8>>(Id::from("bytes"));
				if let Some(screenshot) = fast_bytes{
					r_image = RetainedImage::from_image_bytes(
						"screenshot",
						screenshot.as_slice()
					).unwrap();
					screenshot_ok = true;
				}
			});
		}

        /*
            Acquire action: action when acquire screenshot
        */
		let aa_done = match ctx.memory(|mem| mem.data.get_temp::<bool>(Id::from("aa_done"))){
			Some(a) => a,
			None => false
		};
		
		if !aa_done {
			let config = self.configuration.read().unwrap();
			let aa = config.get_when_capture();
			drop(config);
			
			
			match aa {
				Some(a) => {
					if a.save_file{
						self.save_image(ctx, None);
					}
					if a.copy_file{
						self.copy_image(ctx);
					}
					ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("aa_done"), true));
				}
				None => {}
			}
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
							match native_dialog::FileDialog::new()
								.add_filter("png", &["png"])
								.add_filter("jpg", &["jpg"])
								.add_filter("gif", &["gif"])
								.show_save_single_file() {
								Ok(path) => {
									self.save_image(ctx, path);
								}
								Err(error) => {
									notifica::notify("Error during FileDialog open", &error.to_string())
										.expect("OS API error.");
								}
							};
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
								"Annota")).clicked(){
							self.set_win_type(Drawing);
						}
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
						let uv = Rect::from_min_max(pos2(0.0,0.0), pos2(1.0,1.0));
						let painter_rect = Rect::from_min_size(pos2(margin,window_size.y-size_y - margin), Vec2::new(size_x, size_y));
						
						if screenshot_ok {
							painter.set_clip_rect(painter_rect);
							
							let aspect_ratio = r_image.width() as f32 / r_image.height() as f32;
							let mut width = painter_rect.width();
							let mut height = width / aspect_ratio;
							
							if height > painter_rect.height() {
								height = painter_rect.height();
								width = height * aspect_ratio;
							}
							
							let new_rect = Rect::from_min_size(painter_rect.center() - Vec2::new(width/2.0, size_y/2.0), Vec2::new(width, height));
							painter.image(r_image.texture_id(ctx), new_rect, uv, Color32::WHITE);
						}
					}
				);
			});
		SidePanel::right("right")
			.frame(Frame{inner_margin: Margin {left: 0.0, ..Margin::same(20.0)}, fill: bg_color, ..Default::default()})
			.show_separator_line(false)
			.resizable(false)
			.exact_width(window_size.x*(1.0-w))
			.show(ctx, |ui| {
				ui.with_layout(
					Layout::top_down( Align::LEFT),
					|ui| {
						ui.spacing_mut().item_spacing.y = 10.0;
						ui.label(RichText::new("Acquisisci una nuova schermata").size(16.0));
						ui.allocate_ui_with_layout(
							Vec2::new(180.0, window_size.y),
							Layout::top_down_justified(Align::Center),
							|ui|{
								ui.spacing_mut().button_padding = Vec2::new(10.0, 10.0);
								if ui.button("Schermo attuale").clicked(){
									//self.current_screen(ctx, _frame);
                                    _frame.set_visible(false);
                                    self.set_acquire_mode(Some(AcquireMode::CurrentScreen));
									ctx.memory_mut(|mem| mem.data.remove::<bool>(Id::from("aa_done")));
								};
								if ui.button("Selziona schermo").clicked(){
                                    _frame.set_visible(false);
                                    self.set_acquire_mode(Some(AcquireMode::SelectScreen));
									ctx.memory_mut(|mem| mem.data.remove::<bool>(Id::from("aa_done")));
								};
								if ui.button("Tutti gli schermi").clicked(){
                                    _frame.set_visible(false);
                                    self.set_acquire_mode(Some(AcquireMode::AllScreens));
									ctx.memory_mut(|mem| mem.data.remove::<bool>(Id::from("aa_done")));
								};
								if ui.button("Regione rettangolare").clicked(){
                                    _frame.set_visible(false);
                                    self.set_acquire_mode(Some(AcquireMode::Portion));
									ctx.memory_mut(|mem| mem.data.remove::<bool>(Id::from("aa_done")));
								};
								if ui.button("Impostazioni").clicked(){
									self.set_win_type(Settings);
									ctx.memory_mut(|mem| mem.data.remove::<bool>(Id::from("aa_done")));
								};
								ui.with_layout(Layout::left_to_right(Align::TOP), |ui|{
									let config = self.configuration.read().unwrap();
									match config.get_delay() {
										Some(d) => {
											ui.label(format!("Ritardo: {} {}", d.as_secs(), match d.as_secs() { 1 => "secondo", _ => "secondi" }));
										},
										_=>{}
									}
									if config.get_delay().is_some() {
									}
								})
							}
						)
					});
			});
	}
}

