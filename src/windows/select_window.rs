use eframe::epaint::{ColorImage, hex_color, Rounding, Shadow, Stroke};
use egui::{CentralPanel, Color32, Context, Id, LayerId, Order, pos2, Pos2, Rect, Vec2};
use egui::accesskit::Role::Caption;
use egui_extras::RetainedImage;
use egui_modal::{Icon, Modal};
use env_logger::init;
use log::{error, info};
use screenshots::{Compression, Image};
use crate::window::{Content, WindowType};
use crate::window::WindowType::{Main, Screenshot};
use crate::screenshots::CaptureArea;

#[derive(Clone, Copy, Debug)]
struct LineSegment {
  points: [Pos2; 2],
  stroke: Stroke,
}

impl LineSegment {
  fn new(points: [Pos2; 2], stroke: Stroke) -> Self {
    Self { points, stroke }
  }
}

impl Content{
	pub fn select_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
		let p_frame = egui::Frame{
      inner_margin: Default::default(),
      outer_margin: Default::default(),
      rounding: Default::default(),
      shadow: Default::default(),
      fill: Color32::GOLD,
      stroke: Default::default(),
    };

    CentralPanel::default().show(ctx, |ui| {
      let mut r_image = RetainedImage::from_color_image("", ColorImage::example());
			let window_size: Vec2 = Vec2::new(_frame.info().window_info.size.x, _frame.info().window_info.size.y);
			let mut screenshot_ok = false;

      _frame.set_decorations(false);
      _frame.set_fullscreen(true);
			ctx.set_cursor_icon(egui::CursorIcon::Crosshair);

			ctx.memory(|mem|{
				let mem_image = mem.data.get_temp::<Vec<u8>>(Id::from("bytes"));
				if mem_image.is_some(){
					r_image = RetainedImage::from_image_bytes("bytes", mem_image.unwrap().as_slice()).unwrap();
					screenshot_ok = true;
				}
			});
      let mut painter = ctx.layer_painter(LayerId::new(Order::Background, Id::new("image")));
      painter.set_clip_rect(Rect::from_min_size(pos2(0.0, 0.0), window_size));

			if screenshot_ok {
      	painter.image(r_image.texture_id(ctx), Rect { min: pos2(0.0, 0.0), max: pos2(_frame.info().window_info.size.x, _frame.info().window_info.size.y) }, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
			}


			if ctx.input(|i| i.pointer.primary_released()){
				ctx.memory_mut(|mem|{
					let init_pos = mem.data.get_temp::<Pos2>(Id::new("init_pos"));
					let curr_pos = mem.data.get_temp::<Pos2>(Id::new("curr_pos"));
					if init_pos.is_some() && curr_pos.is_some(){
						let di = self.get_current_screen_di(_frame);
						let x = di.x + init_pos.unwrap().x as i32;
						let y = di.y + init_pos.unwrap().y as i32;
						let width = (curr_pos.unwrap().x - init_pos.unwrap().x) as u32;
						let height = (curr_pos.unwrap().y - init_pos.unwrap().y) as u32;
						let ca = CaptureArea::new(x, y, width, height);
						info!("Capture area: {:?}", di);
						match self.get_se().screenshot(di,ca) {
							Ok(screenshot) => {
								let img_bytes = screenshot.rgba().clone();
								let img_bytes_fast = screenshot.to_png(None).unwrap();
								mem.data.insert_temp(Id::from("screenshot"), img_bytes);
								mem.data.insert_temp(Id::from("bytes"), img_bytes_fast.clone());
								mem.data.insert_temp(Id::from("width"), screenshot.width());
								mem.data.insert_temp(Id::from("height"), screenshot.height());
								self.set_win_type(Screenshot);
							}
							Err(error) => {
								error!("{}",error);
								/*
								Modal::new(ctx, "error_alert").open_dialog(
									Some("Error during screenshot print."),
									Some(error),
									Some(Icon::Error));

								 */
								self.set_win_type(WindowType::Main);
							}
						};
					}
				});
			}


      let mut init_pos = pos2(0.0,0.0);
      let mut curr_pos = ctx.input(|i| {
        if i.pointer.primary_down(){
          if i.pointer.press_origin().is_some(){
            init_pos = i.pointer.press_origin().unwrap();
          }
          if i.pointer.hover_pos().is_some(){
            let curr_pos = i.pointer.hover_pos().unwrap();
            curr_pos
          }
          else { pos2(0.0,0.0) }
        }
        else { pos2(0.0,0.0) }
      });
      if curr_pos != init_pos {
				if curr_pos.x < init_pos.x{
					let tmp = init_pos.x;
					init_pos.x = curr_pos.x;
					curr_pos.x = tmp;
				}
				if curr_pos.y < init_pos.y {
					let tmp = init_pos.y;
					init_pos.y = curr_pos.y;
					curr_pos.y = tmp;
				}
				ctx.memory_mut(|mem| {
					mem.data.insert_temp(Id::new("init_pos"), init_pos);
					mem.data.insert_temp(Id::new("curr_pos"), curr_pos);
				});
				painter.rect_filled(Rect::from_min_max(pos2(0.0,0.0), pos2(init_pos.x, window_size.y)), 0.0, hex_color!("#00000064"));
				painter.rect_filled(Rect::from_min_max(pos2(init_pos.x, 0.0), pos2(curr_pos.x, init_pos.y)), 0.0, hex_color!("#00000064"));
				painter.rect_filled(Rect::from_min_max(pos2(init_pos.x, curr_pos.y), pos2(curr_pos.x, window_size.y)), 0.0, hex_color!("#00000064"));
				painter.rect_filled(Rect::from_min_max(pos2(curr_pos.x, 0.0), pos2(window_size.x, window_size.y)), 0.0, hex_color!("#00000064"));
				init_pos = pos2(init_pos.x-1.5, init_pos.y-1.5);
				curr_pos = pos2(curr_pos.x+1.5, curr_pos.y+1.5);
				painter.rect_stroke(Rect::from_min_max(init_pos, curr_pos), 0.0, Stroke::new(0.5, Color32::GREEN));
      }
      else {
        painter.rect_filled(Rect::from_min_size(pos2(0.0,0.0), window_size), 0.0, hex_color!("#00000064"));
      }
    });
	}
}
