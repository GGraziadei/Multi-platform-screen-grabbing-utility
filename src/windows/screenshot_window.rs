use eframe::emath::{Align, Vec2};
use egui::{Color32, ColorImage, Context, Direction, Frame, hex_color, Id, LayerId, Layout, Margin, Order, pos2, Rect, RichText, SidePanel, TopBottomPanel};
use egui_extras::RetainedImage;
use screenshots::{Compression, DisplayInfo};
use crate::draw_window::Content;
use crate::draw_window::WindowType::Screenshot;
use crate::screenshots::CaptureArea;

impl Content {
	pub fn screenshot_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
		let window_size = _frame.info().window_info.size;
    let bg_color = ctx.style().visuals.panel_fill;
		let w = 0.6;
		let margin = 20.0;

    TopBottomPanel::top("top")
      .frame(Frame{fill: bg_color, ..Default::default()})
      .show_separator_line(false)
      .resizable(false)
      .min_height(window_size.y*0.1)
      .show(ctx, |ui| {
        ui.allocate_ui_with_layout(
          Vec2::new(window_size.x, window_size.y*0.1),
          Layout::centered_and_justified(Direction::TopDown),
          |ui| {
            ui.heading("Acquisisci una nuova schermata")
          }
        );
      });
		_frame.set_window_size(Vec2::new(1000.0, 600.0));
		// _frame.set_centered();
    // TopBottomPanel::top("bottom")
    //   .frame(Frame{fill: bg_color, ..Default::default()})
    //   .show_separator_line(false)
    //   .resizable(false)
    //   .min_height(window_size.y*0.9)
    //   .show(ctx, |ui| {
    //     let w = 0.4;
    //   });
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
						let size_x = (window_size.x*w) + 2.0 * margin;
						let size_y = window_size.y - margin;
						painter.set_clip_rect(Rect::from_min_max(pos2(margin,margin), pos2(size_x, size_y)));
						let mut r_image = RetainedImage::from_color_image("screenshot", ColorImage::example());let mut screenshot_ok = false;
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
								let start_y = margin + ((size_y-margin)/2.0) - (image_rect_h/2.0);
								image_rect = Rect::from_min_size(pos2(margin,start_y), Vec2::new(image_rect_w, image_rect_h));
							}
							else {
								let image_rect_h = size_y;
								let image_rect_w = image_rect_h/r_image.height() as f32 * r_image.width() as f32;
								let start_x = margin + ((size_x-margin)/2.0) - (image_rect_w/2.0);
								image_rect = Rect::from_min_size(pos2(start_x,margin), Vec2::new(image_rect_w, image_rect_h));
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
						if ui.button("Regione rettangolare").clicked(){};
						if ui.button("Tutti gli schermi").clicked(){};
						if ui.button("Schermo attuale").clicked(){
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
								}
							}
							let screenshot = self.get_se().screenshot(di, None, CaptureArea::new(0,0, di.width, di.height)).unwrap();
							let imgf = screenshot.to_png(Some(Compression::Best)).unwrap();
							ctx.memory_mut(|mem|{
								mem.data.insert_temp(Id::from("screenshot"), imgf);
								self.set_win_type(Screenshot);
							});
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
