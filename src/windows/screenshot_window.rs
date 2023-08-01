use eframe::emath::{Align, Vec2};
use egui::{Button, Color32, ColorImage, Context, Direction, Frame, hex_color, Id, LayerId, Layout, Margin, Order, pos2, Rect, RichText, SidePanel, TopBottomPanel};
use egui_extras::RetainedImage;
use screenshots::{Compression, DisplayInfo};
use crate::draw_window::Content;
use crate::draw_window::WindowType::Screenshot;
use crate::image_combiner::ImageCombiner;
use crate::screenshots::CaptureArea;

impl Content {
	pub fn screenshot_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
		let window_size = _frame.info().window_info.size;
    let bg_color = ctx.style().visuals.panel_fill;
		let w = 0.6;
		let margin = 20.0;
		let mut first_render = true;

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

    TopBottomPanel::top("top")
      .frame(Frame{fill: bg_color, inner_margin: Margin::same(margin), ..Default::default()})
      .show_separator_line(false)
      .resizable(false)
      .show(ctx, |ui| {
        ui.with_layout(
					Layout::left_to_right(Align::LEFT),
					|ui| {
						ui.spacing_mut().button_padding = Vec2::new(10.0, 10.0);
						ui.add(
							Button::image_and_text(
								RetainedImage::from_color_image(
									"",
									ColorImage::example())
									.texture_id(ctx),
								Vec2::new(10.0,10.0),
								"Button"));
						ui.button("Button 1");
						ui.button("Button 1");
						ui.button("Button 1");
						ui.button("Button 1");
						ui.button("Button 1");
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
						let mut r_image = RetainedImage::from_color_image("screenshot", ColorImage::example());
						let mut screenshot_ok = false;
						let painter_rect = Rect::from_min_size(pos2(margin,window_size.y-size_y - margin), Vec2::new(size_x, size_y));

						painter.set_clip_rect(painter_rect);
						// painter.rect_filled(painter_rect, 0.0, Color32::RED);
						ctx.memory(|mem|{
							let mem_image = mem.data.get_temp::<Vec<u8>>(Id::from("screenshot"));
							if mem_image.is_some(){
								r_image = RetainedImage::from_image_bytes("screenshot", mem_image.unwrap().as_slice()).unwrap();
								screenshot_ok = true;
							}
						});

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
						ui.add_space(20.0);
						ui.label(RichText::new("Opzioni di acquisizione").size(16.0));
						ui.checkbox(&mut true, "Includi il puntatore del mouse");
						ui.checkbox(&mut true, "Includi la barra del titolo e i bordi della finestra");
						ui.checkbox(&mut true, "Cattura solo la finestra attuale");
						ui.checkbox(&mut true, "Esci dopo il salvataggio o la copia manuali");
						ui.checkbox(&mut true, "Cattura al click");
					});
			});
	}



}
