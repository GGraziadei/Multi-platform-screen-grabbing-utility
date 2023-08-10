use eframe::emath::{Align, Vec2};
use egui::{CentralPanel, ColorImage, Context, Frame, Id, LayerId, Layout, Margin, Order, pos2, Rect, RichText, ScrollArea, TopBottomPanel};
use egui_extras::RetainedImage;
use crate::window::Content;
use crate::window::WindowType::{Main, Preview};

impl Content {
	pub fn select_screen_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
		let mut first_render = true;
		let bg_color = ctx.style().visuals.panel_fill;
		let margin = 20.0;
		let mut images = vec![];
		let image_bytes = ctx.memory(|mem| { mem.data.get_temp::<Vec<Vec<u8>>>(Id::from("bytes"))}).unwrap();
		let bytes = match ctx.memory(|mem| { mem.data.get_temp::<Vec<Vec<u8>>>(Id::from("r_bytes"))}){
			Some(bytes) => {
				for b in bytes.iter() {
					let r_image = RetainedImage::from_image_bytes(
						"screenshot",
						b.as_slice()
					).unwrap();
					images.push(r_image);
				}
				bytes.clone()
			},
			None => {
				images.push(RetainedImage::from_color_image("screenshot", ColorImage::example()));
				vec![]
			}
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
				ui.vertical_centered(|ui|{
					ui.heading(RichText::new("Seleziona lo schermo").size(24.0));
				});
				ui.add_space(40.0);
				ScrollArea::vertical().max_height(ui.available_height() - 58.0).auto_shrink([true, true]).show(ui, |ui|{
					ui.horizontal_wrapped( |ui|{
						ui.spacing_mut().item_spacing.x = 9.9;
						ui.spacing_mut().item_spacing.y = 20.0;
						let container_height = 180.0;
						let mut rects = vec![];
						for (i, image) in images.iter().enumerate() {
							rects.push(
								ui.allocate_ui_with_layout(
									Vec2::new(ui.available_width()/3.0 - 10.0, container_height),
									Layout::top_down(Align::Center),
									|ui|{
										ui.with_layer_id(LayerId::new(Order::Foreground, Id::from(format!("image_{}", i))), |ui|{
											ui.spacing_mut().item_spacing.y = 0.0;
											let mut image_height = container_height - 30.0;
											let aspect_ratio = image.width() as f32/ image.height() as f32;
											let mut image_width = aspect_ratio * image_height;
											if image_width > ui.available_width(){
												image_width = ui.available_width();
												image_height = image_width/aspect_ratio;
												ui.add_space((150.0 - image_height)/2.0);
											}
											ui.image(image.texture_id(ctx), Vec2::new(image_width, image_height));
											ui.add_space((150.0 - image_height)/2.0 + 10.0);
											ui.label(format!("Schermo: {}", i+1));
										});
									}).response.rect);
						}
						let mut selected_screen = -1;
						for (n, rect) in rects.iter().enumerate() {
							if ctx.input(|i| {
								match i.pointer.hover_pos(){
									Some(pos) => {
										if rect.contains(pos){
											if i.pointer.primary_clicked(){
												selected_screen = n as i32;
											}
											return true;
										}
										return false;
									},
									None => return false
								}
							})
							{
								let new_rect = Rect::from_min_size(pos2(rect.min.x-5.0, rect.min.y-5.0), Vec2::new(rect.width()+10.0, rect.height()+10.0));
								let color = ctx.style().visuals.widgets.inactive.bg_fill;
								ctx.layer_painter(LayerId::background()).rect_filled(new_rect, 8.0, color);
							}
							if selected_screen != -1 {
								let r_image = bytes[selected_screen as usize].clone();
								let image = image_bytes[selected_screen as usize].clone();
								let width = images[selected_screen as usize].width() as u32;
								let height = images[selected_screen as usize].height() as u32;
								ctx.memory_mut(|mem|{
									mem.data.insert_temp(Id::from("bytes"), r_image);
									mem.data.insert_temp(Id::from("screenshot"), image);
									mem.data.insert_temp(Id::from("width"), width);
									mem.data.insert_temp(Id::from("height"), height);
									self.set_win_type(Preview);
								});
							}
						}
					});
				});
			});
		TopBottomPanel::bottom("bottom_panel")
			.resizable(false)
			.show_separator_line(false)
			.frame(Frame{fill: bg_color, inner_margin: Margin::same(20.0), ..Default::default()})
			.show(ctx, |ui|{
				ui.with_layout(Layout::top_down(Align::RIGHT), |ui|{
					ui.spacing_mut().button_padding = Vec2::splat(10.0);
					if ui.button("Annulla").clicked(){
						self.set_win_type(Main);
					}
				});
			});
	}
	
}
