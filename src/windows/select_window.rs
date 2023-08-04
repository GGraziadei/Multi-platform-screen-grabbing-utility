use eframe::epaint::{ColorImage, hex_color, Stroke};
use egui::{CentralPanel, Color32, Context, Id, LayerId, Order, pos2, Pos2, Rect, Vec2};
use egui_extras::RetainedImage;
use screenshots::{DisplayInfo, };
use crate::window::{Content, };

impl Content{
	pub fn select_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){

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
      let mut painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("image")));
      painter.set_clip_rect(Rect::from_min_size(pos2(0.0, 0.0), window_size));

			if screenshot_ok {
      	painter.image(r_image.texture_id(ctx), Rect { min: pos2(0.0, 0.0), max: pos2(_frame.info().window_info.size.x, _frame.info().window_info.size.y) }, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
			}

			if ctx.input(|i| i.pointer.primary_released()){
				ctx.memory_mut(|mem|{
					let init_pos = mem.data.get_temp::<Pos2>(Id::new("init_pos"));
					let curr_pos = mem.data.get_temp::<Pos2>(Id::new("curr_pos"));
					if init_pos.is_some() && curr_pos.is_some(){
						let region = Rect::from_min_max(init_pos.unwrap(), curr_pos.unwrap());
						self.set_region(region);
						_frame.request_screenshot();
					}
				});
			}
	    
	    else {
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
	    }
    });
	}
}
