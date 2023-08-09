use eframe::epaint::{ColorImage, hex_color, Stroke};
use egui::{Align, CentralPanel, Color32, Context, CursorIcon, Id, LayerId, Layout, Order, pos2, Pos2, Rect, Response, Sense, Vec2};
use egui_extras::RetainedImage;
use crate::window::{Content, };

impl Content{
	pub fn select_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
		
		CentralPanel::default().show(ctx, |ui| {
			let mut r_image = RetainedImage::from_color_image("", ColorImage::example());
			let window_size: Vec2 = Vec2::new(_frame.info().window_info.size.x, _frame.info().window_info.size.y);
			let mut screenshot_ok = false;
            let config = self.configuration.read().unwrap();
            let green = hex_color!("#16A085");
		
            let region = match config.get_save_region() {
                true => {
                    match ctx.memory(|mem| mem.data.get_temp::<Rect>(Id::from("region"))){
                        Some(r) => {
                            Some(r)
                        },
                        None => {
                            config.get_region()
                        }
                    }
                },
                false => {
                    None
                }
            };
            
            let init_grab_pos = ctx.memory(|mem| mem.data.get_temp::<Pos2>(Id::from("init_grab_pos")));
            let visible = match ctx.memory(|mem| mem.data.get_temp::<bool>(Id::from("visible"))){
                Some(v) => {
                    v
                },
                None => {
                    true
                }
            };
            
		    drop(config);
			
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
			
			_frame.set_decorations(false);
			_frame.set_fullscreen(true);
			
            match region {
                Some(r) => {
                    let mut init_pos = r.min;
                    let mut final_pos = r.max;
                    let reg_width = final_pos.x - init_pos.x;
                    let reg_height = final_pos.y - init_pos.y;
                    
                    let handle_tl_pos = init_pos;
                    let handle_tm_pos = pos2(init_pos.x + (reg_width/2.0), init_pos.y);
                    let handle_tr_pos = pos2(final_pos.x, init_pos.y);
                    let handle_ml_pos = pos2(init_pos.x, init_pos.y + (reg_height/2.0));
                    let handle_mr_pos = pos2(final_pos.x, init_pos.y + (reg_height/2.0));
                    let handle_bl_pos = pos2(init_pos.x, final_pos.y);
                    let handle_bm_pos = pos2(init_pos.x + (reg_width/2.0), final_pos.y);
                    let handle_br_pos = final_pos;
                    
                    let handle_tl_rect = Rect::from_center_size(handle_tl_pos, Vec2::splat(10.0));
                    let handle_tm_rect = Rect::from_center_size(handle_tm_pos, Vec2::splat(10.0));
                    let handle_tr_rect = Rect::from_center_size(handle_tr_pos, Vec2::splat(10.0));
                    let handle_ml_rect = Rect::from_center_size(handle_ml_pos, Vec2::splat(10.0));
                    let handle_mr_rect = Rect::from_center_size(handle_mr_pos, Vec2::splat(10.0));
                    let handle_bl_rect = Rect::from_center_size(handle_bl_pos, Vec2::splat(10.0));
                    let handle_bm_rect = Rect::from_center_size(handle_bm_pos, Vec2::splat(10.0));
                    let handle_br_rect = Rect::from_center_size(handle_br_pos, Vec2::splat(10.0));
                    
                    let mut button_id = Id::new("button");
                    
                    let button = ui.with_layer_id(LayerId::new(Order::Foreground, Id::from("")), |ui|{
                        ui.set_visible(visible);
                        if !visible {
                            self.set_region(r);
                            let mut config = self.configuration.write().unwrap();
                            if config.get_save_region() {
                                config.set_region(r);
                            }
                            ctx.memory_mut(|mem| {
                                mem.data.remove::<Rect>(Id::from("region"));
                                mem.data.remove::<bool>(Id::from("visible"));
                            });
                            drop(config);
                            _frame.request_screenshot();
                        }
                        ui.with_layout(Layout::right_to_left(Align::BOTTOM), |ui|{
                            ui.visuals_mut().widgets.inactive.weak_bg_fill = Color32::RED;
                            ui.visuals_mut().widgets.hovered.weak_bg_fill = green;
                            ui.spacing_mut().button_padding = Vec2::splat(10.0);
                            let button_widget = ui.button("Salva");
                            button_id = button_widget.id;
                            button_widget
                        })
                    });
                    
                    let button_rect = button.inner.inner.rect;
                    
                    let mouse_pos = ctx.input(|i| i.pointer.hover_pos());
                    match mouse_pos {
                        Some(pos) => {
                            if button_rect.contains(pos){
                                ctx.highlight_widget(button_id);
                                ctx.set_cursor_icon(CursorIcon::PointingHand);
                                if ctx.input(|i| i.pointer.primary_clicked()){
                                    ctx.memory_mut(|mem|{
                                        mem.data.insert_temp(Id::new("visible"), false);
                                    });
                                }
                            }
                            else if handle_tl_rect.contains(pos){
                                ctx.set_cursor_icon(CursorIcon::Grab);
                            }
                            else if handle_tm_rect.contains(pos){
                                ctx.set_cursor_icon(CursorIcon::Grab);
                            }
                            else if handle_tr_rect.contains(pos){
                                ctx.set_cursor_icon(CursorIcon::Grab);
                            }
                            else if handle_ml_rect.contains(pos){
                                ctx.set_cursor_icon(CursorIcon::Grab);
                            }
                            else if handle_mr_rect.contains(pos){
                                ctx.set_cursor_icon(CursorIcon::Grab);
                            }
                            else if handle_bl_rect.contains(pos){
                                ctx.set_cursor_icon(CursorIcon::Grab);
                            }
                            else if handle_bm_rect.contains(pos){
                                ctx.set_cursor_icon(CursorIcon::Grab);
                            }
                            else if handle_br_rect.contains(pos){
                                ctx.set_cursor_icon(CursorIcon::Grab);
                            }
                            else if r.contains(pos){
                                ctx.set_cursor_icon(CursorIcon::Grab);
                                if ctx.input(|i| i.pointer.primary_down()){
                                    ctx.set_cursor_icon(CursorIcon::Grabbing);
                                    ctx.memory_mut(|mem|{
                                        mem.data.insert_temp(Id::new("init_grab_pos"), pos);
                                    });
                                    let curr_grab_pos = ctx.input(|i| i.pointer.hover_pos());
                                    if init_grab_pos.is_some(){
                                        let dx = curr_grab_pos.unwrap().x - init_grab_pos.unwrap().x;
                                        let dy = curr_grab_pos.unwrap().y - init_grab_pos.unwrap().y;
                                        let mut new_init_x = init_pos.x + dx;
                                        let mut new_init_y = init_pos.y + dy;
                                        
                                        if new_init_x < 0.0 {
                                            new_init_x = 0.0;
                                        }
                                        if new_init_y < 0.0 {
                                            new_init_y = 0.0;
                                        }
                                        
                                        if new_init_x + reg_width > _frame.info().window_info.size.x {
                                            new_init_x = _frame.info().window_info.size.x - reg_width;
                                        }
                                        
                                        if new_init_y + reg_height > _frame.info().window_info.size.y {
                                            new_init_y = _frame.info().window_info.size.y - reg_height;
                                        }
                                        
                                        let new_init_pos = pos2(new_init_x, new_init_y);
                                        let new_region = Rect::from_min_size(new_init_pos, Vec2::new(reg_width, reg_height));
                                        ctx.memory_mut(|mem|{
                                            mem.data.insert_temp(Id::new("region"), new_region);
                                        });
                                    }
                                }
                                else {
                                    ctx.memory_mut(|mem|{
                                        mem.data.remove::<Pos2>(Id::new("init_grab_pos"));
                                    });
                                }
                            }
                        },
                        None => {
                            ctx.set_cursor_icon(CursorIcon::default());
                        }
                    }
                    
                    painter.rect_filled(Rect::from_min_max(pos2(0.0,0.0), pos2(init_pos.x, window_size.y)), 0.0, hex_color!("#00000064"));
                    painter.rect_filled(Rect::from_min_max(pos2(init_pos.x, 0.0), pos2(final_pos.x, init_pos.y)), 0.0, hex_color!("#00000064"));
                    painter.rect_filled(Rect::from_min_max(pos2(init_pos.x, final_pos.y), pos2(final_pos.x, window_size.y)), 0.0, hex_color!("#00000064"));
                    painter.rect_filled(Rect::from_min_max(pos2(final_pos.x, 0.0), pos2(window_size.x, window_size.y)), 0.0, hex_color!("#00000064"));
                    init_pos = pos2(init_pos.x-1.5, init_pos.y-1.5);
                    final_pos = pos2(final_pos.x+1.5, final_pos.y+1.5);
                    painter.rect_stroke(Rect::from_min_max(init_pos, final_pos), 0.0, Stroke::new(0.5, green));
                    
                    if visible{
                        painter.circle_filled(handle_tl_pos, 10.0, green);
                        painter.circle_filled(handle_tm_pos, 10.0, green);
                        painter.circle_filled(handle_tr_pos, 10.0, green);
                        painter.circle_filled(handle_ml_pos, 10.0, green);
                        painter.circle_filled(handle_mr_pos, 10.0, green);
                        painter.circle_filled(handle_bl_pos, 10.0, green);
                        painter.circle_filled(handle_bm_pos, 10.0, green);
                        painter.circle_filled(handle_br_pos, 10.0, green);
                    }
                    
                },
                None => {
			        ctx.set_cursor_icon(egui::CursorIcon::Crosshair);
                    if ctx.input(|i| i.pointer.primary_released()){
                        ctx.memory_mut(|mem|{
                            let init_pos = mem.data.get_temp::<Pos2>(Id::new("init_pos"));
                            let curr_pos = mem.data.get_temp::<Pos2>(Id::new("curr_pos"));
                            if init_pos.is_some() && curr_pos.is_some(){
                                let region = Rect::from_min_max(init_pos.unwrap(), curr_pos.unwrap());
                                self.set_region(region);
                                let mut config = self.configuration.write().unwrap();
                                if config.get_save_region() {
                                    config.set_region(region);
                                }
                                drop(config);
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
                            painter.rect_stroke(Rect::from_min_max(init_pos, curr_pos), 0.0, Stroke::new(0.5, green));
                        }
                        else {
                            painter.rect_filled(Rect::from_min_size(pos2(0.0,0.0), window_size), 0.0, hex_color!("#00000064"));
                        }
                    }
                }
            }
			
		});
	}
}
