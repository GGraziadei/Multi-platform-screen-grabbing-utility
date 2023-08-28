use std::collections::HashMap;
use std::default::Default;
use std::time::Duration;
use eframe::Theme;
use egui::{Align, Button, Context, Layout, SidePanel, Vec2, Frame, Widget, Margin, hex_color, TopBottomPanel, CentralPanel, Color32, Order, LayerId, Id, RichText, TextEdit, ImageButton, ComboBox, Sense, CursorIcon, DragValue};
use egui_extras::RetainedImage;
use log::error;
use native_dialog::FileDialog;
use crate::configuration::{AcquireAction, AcquireMode, ImageFmt, KeyCombo};
use crate::configuration::ImageFmt::{GIF, JPG, PNG};
use crate::window::Content;
use crate::window::WindowType::Main;
use core::option::Option;

#[derive(Clone)]
enum Tab {
    General,
    Save,
    Shortcuts
}

const TIME_BUTTONS: [&str; 6] = ["%Y", "%m", "%d", "%H", "%M", "%S"];
const TIME_STRINGS: [&str; 6] = ["Anno", "Mese", "Giorno", "Ora", "Minuto", "Secondo"];

impl Content {
    pub fn settings_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
        let bg_color = ctx.style().visuals.panel_fill;
        let ( r,  g, b,  a) = bg_color.to_tuple();
        let selected_color = ctx.style().visuals.widgets.active.bg_fill;

        let configuration_read = self.configuration.read().unwrap();

        let mut path = match ctx.memory(|mem| mem.data.get_temp::<String>(Id::from("path"))) {
            Some(s) => s,
            None => configuration_read.get_save_path().unwrap()
        };
        let mut filename_pattern = match ctx.memory(|mem| mem.data.get_temp::<String>(Id::from("pattern"))) {
            Some(s) => s,
            None => configuration_read.get_filename_pattern().unwrap()
        };
        let mut format = match ctx.memory(|mem| mem.data.get_temp::<ImageFmt>(Id::from("format"))) {
            Some(format) => format,
            None => configuration_read.get_image_fmt().unwrap(),
        };
        let mut when_acquire = match ctx.memory(|mem| mem.data.get_temp::<AcquireAction>(Id::from("when_acquire"))) {
            Some(wa) => wa,
            None => configuration_read.get_when_capture().unwrap()
        };
        let mut save_region = match ctx.memory(|mem| mem.data.get_temp::<bool>(Id::from("save_region"))) {
            Some(sr) => sr,
            None => configuration_read.get_save_region()
        };
        let mut region = configuration_read.get_region();
        let delay = match ctx.memory(|mem| mem.data.get_temp::<Option<Duration>>(Id::from("delay"))) {
            Some(d) => d,
            None => configuration_read.get_delay()
        };
        let mut hot_key_map = match ctx.memory(|mem| mem.data.get_temp::<HashMap<AcquireMode, KeyCombo>>(Id::from("hot_key_map"))) {
            Some(hkm) => hkm,
            None => configuration_read.get_hot_key_map().unwrap()
        };

        let mut tab = ctx.memory_mut(|mem|{
            if mem.data.get_temp::<Tab>(Id::from("tab")).is_none(){
                mem.data.insert_temp(Id::from("tab"), Tab::General);
                Tab::General
            }
            else {
                mem.data.get_temp::<Tab>(Id::from("tab")).unwrap()
            }
        });

        let bg_dark_color = Color32::from_rgba_unmultiplied(r,g,b,a);
       
        drop(configuration_read);

        _frame.set_window_size(Vec2::new(800.0, 400.0));
        //r = r - 10;
        //g = g - 10;
        //b = b - 10;

        CentralPanel::default().show(ctx, |_| {
            SidePanel::left("tabs")
                .frame(Frame{ fill: bg_dark_color, inner_margin: Margin::same(10.0), ..Default::default()})
                .exact_width(100.0)
                .show_separator_line(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.with_layout(Layout::top_down_justified(Align::Center), |ui|{
                        ui.spacing_mut().item_spacing.y = 10.0;

                        //TABS ICONS
                        let mut settings_icon = RetainedImage::from_svg_bytes(
                            "settings",
                            include_bytes!("../images/settings_black.svg")).unwrap();
                        let mut save_icon = RetainedImage::from_svg_bytes(
                            "save",
                            include_bytes!("../images/save_black.svg")).unwrap();
                        let mut keyboard_icon = RetainedImage::from_svg_bytes(
                            "keyboard",
                            include_bytes!("../images/keyboard_black.svg")).unwrap();

                        if _frame.info().system_theme.is_none() || _frame.info().system_theme.unwrap() == Theme::Dark{
                            settings_icon = RetainedImage::from_svg_bytes(
                                "settings",
                                include_bytes!("../images/settings_white.svg")).unwrap();
                            save_icon = RetainedImage::from_svg_bytes(
                                "save",
                                include_bytes!("../images/save_white.svg")).unwrap();
                            keyboard_icon = RetainedImage::from_svg_bytes(
                                "keyboard",
                                include_bytes!("../images/keyboard_white.svg")).unwrap();
                        }

                        //LEFT COLUMN TABS
                        let rect1 = ui.with_layout(Layout::top_down(Align::Center), |ui|{
                            ui.with_layer_id(LayerId::new(Order::Foreground, Id::from("1")), |ui|{
                                ui.add_space(10.0);
                                ui.spacing_mut().item_spacing.y = 8.0;
                                ui.image(settings_icon.texture_id(ctx), Vec2::new(30.0, 30.0));
                                ui.label("Generale");
                                ui.add_space(5.0);
                            });
                        }).response.rect;

                        let rect2 = ui.with_layout(Layout::top_down(Align::Center), |ui|{
                            ui.with_layer_id(LayerId::new(Order::Foreground, Id::from("2")), |ui|{
                                ui.add_space(10.0);
                                ui.spacing_mut().item_spacing.y = 8.0;
                                ui.image(save_icon.texture_id(ctx), Vec2::new(30.0, 30.0));
                                ui.label("Salvataggio");
                                ui.add_space(5.0);
                            });
                        }).response.rect;

                        let rect3 = ui.with_layout(Layout::top_down(Align::Center), |ui|{
                            ui.with_layer_id(LayerId::new(Order::Foreground, Id::from("3")), |ui|{
                                ui.add_space(10.0);
                                ui.spacing_mut().item_spacing.y = 8.0;
                                ui.image(keyboard_icon.texture_id(ctx), Vec2::new(30.0, 30.0));
                                ui.label("Scorciatoie");
                                ui.add_space(5.0);
                            });
                        }).response.rect;

                        let rects = vec![rect1, rect2, rect3];

                        let colors = match tab {
                            Tab::General => vec![selected_color, bg_dark_color, bg_dark_color],
                            Tab::Save => vec![bg_dark_color, selected_color, bg_dark_color],
                            Tab::Shortcuts => vec![bg_dark_color, bg_dark_color, selected_color]
                        };


                        ctx.layer_painter(LayerId::new(Order::Background, Id::from("1")))
                            .rect_filled(
                                rects[0],
                                8.0,
                                colors[0]
                            );
                        ctx.layer_painter(LayerId::new(Order::Background, Id::from("2")))
                            .rect_filled(
                                rects[1],
                                8.0,
                                colors[1]
                            );
                        ctx.layer_painter(LayerId::new(Order::Background, Id::from("3")))
                            .rect_filled(
                                rects[2],
                                8.0,
                                colors[2]
                            );

                        let mut tab_changed = false;
                        let hover = ctx.input(|i| {
                            let mouse_pos = i.pointer.hover_pos();
                            match mouse_pos {
                                Some(pos) => {
                                    if rect1.contains(pos) {
                                        if i.pointer.primary_clicked(){
                                            tab = Tab::General;
                                            tab_changed = true;
                                        }
                                        return 1_u8
                                    }
                                    else if rect2.contains(pos) {
                                        if i.pointer.primary_clicked(){
                                            tab = Tab::Save;
                                            tab_changed = true;
                                        }
                                        return 2_u8
                                    }
                                    else if rect3.contains(pos) {
                                        if i.pointer.primary_clicked(){
                                            tab = Tab::Shortcuts;
                                            tab_changed = true;
                                        }
                                        return 3_u8
                                    }
                                    else {
                                        return 0_u8
                                    }
                                }
                                None => { return 0_u8 }
                            }
                        });

                        if tab_changed{
                            ctx.memory_mut(|mem|{
                                mem.data.insert_temp(Id::from("tab"), tab.clone());
                            })
                        }

                        if hover > 0_u8{
                            ctx.layer_painter(LayerId::new(Order::Background, Id::from("3")))
                                .rect_filled(
                                    rects[(hover-1) as usize],
                                    8.0,
                                    ctx.style().visuals.widgets.inactive.bg_fill
                                );
                        }

                    })
                });
            match tab {
                Tab::General => {
                    CentralPanel::default()
                        .frame(Frame{ fill: bg_color, inner_margin: Margin::same(20.0), ..Default::default()})
                        .show(ctx, |ui|{
                            ctx.available_rect();
                            ui.with_layout(Layout::top_down(Align::Center), |ui|{
                                ui.spacing_mut().item_spacing.y = 10.0;
                                ui.heading(RichText::new("Generale").size(24.0));
                            });
                            ui.add_space(20.0);
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui|{
                                let left_size = Vec2::new(ui.available_size()[0]*0.3, ui.available_size()[1]);
                                let right_size = Vec2::new(ui.available_size()[0]*0.7, ui.available_size()[1]);
                                ui.allocate_ui_with_layout(left_size,Layout::top_down(Align::RIGHT), |ui|{
                                    ui.add_space(2.0);
                                    ui.label("Dopo aver acquisito una schermata");
                                });
                                ui.add_space(20.0);
                                ui.allocate_ui_with_layout(right_size,Layout::top_down(Align::LEFT), |ui|{
                                    ui.spacing_mut().item_spacing.x = 10.0;
                                    if ui.checkbox(&mut when_acquire.save_file, "Salva automaticamente il file nel percorso predefinito").changed(){
                                        ctx.memory_mut(|mem|{
                                            mem.data.insert_temp(Id::from("when_acquire"), when_acquire.clone());
                                        });
                                    }
                                    if ui.checkbox(&mut when_acquire.copy_file, "Copia automaticamente il file nella clipboard").changed(){
                                        ctx.memory_mut(|mem|{
                                            mem.data.insert_temp(Id::from("when_acquire"), when_acquire.clone());
                                        });
                                    }
                                });
                            });
                            ui.add_space(30.0);
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui|{
                                let left_size = Vec2::new(ui.available_size()[0]*0.3, ui.available_size()[1]);
                                let right_size = Vec2::new(ui.available_size()[0]*0.7, ui.available_size()[1]);
                                ui.allocate_ui_with_layout(left_size,Layout::top_down(Align::RIGHT), |ui|{
                                    ui.add_space(2.0);
                                    ui.label("Ricorda l'area selezionata");
                                });
                                ui.add_space(20.0);
                                ui.allocate_ui_with_layout(right_size,Layout::top_down(Align::LEFT), |ui|{
                                    ui.add(toggle(&mut save_region));
                                    ctx.memory_mut(|mem|{
                                        mem.data.insert_temp(Id::from("save_region"), save_region.clone());
                                    });
                                });
                            });
                            ui.add_space(30.0);
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui|{
                                let left_size = Vec2::new(ui.available_size()[0]*0.3, ui.available_size()[1]);
                                let right_size = Vec2::new(ui.available_size()[0]*0.7, ui.available_size()[1]);
                                ui.allocate_ui_with_layout(left_size,Layout::top_down(Align::RIGHT), |ui|{
                                    ui.add_space(2.0);
                                    ui.label("Ritardo prima di acquisire");
                                });
                                ui.add_space(20.0);
                                ui.allocate_ui_with_layout(right_size,Layout::left_to_right(Align::TOP), |ui|{

                                    let mut delay_tmp = match delay {
                                        Some(d) => d.as_secs(),
                                        None => 0
                                    };
                                    let text_edit = DragValue::new(&mut delay_tmp).ui(ui);
                                    ui.spacing_mut().button_padding.x = 8.0;
                                    let _ = ui.button("Salva");
                                    if text_edit.changed() {

                                        ctx.memory_mut(|mem|{
                                            if delay_tmp == 0 {
                                                mem.data.insert_temp::<Option<Duration>>(Id::from("delay"), None);
                                            }
                                            else {
                                                mem.data.insert_temp(Id::from("delay"), Some(Duration::from_secs(delay_tmp)));
                                            }
                                        });
                                    }
                                });
                            });
                        });
                }
                Tab::Save => {
                    CentralPanel::default()
                        .frame(Frame{ fill: bg_color, inner_margin: Margin::same(20.0), ..Default::default()})
                        .show(ctx, |ui|{
                            ctx.available_rect();
                            ui.with_layout(Layout::top_down(Align::Center), |ui|{
                                ui.spacing_mut().item_spacing.y = 20.0;
                                ui.heading(RichText::new("Salvataggio").size(24.0));
                            });
                            ui.add_space(20.0);

                            //DEFAULT SAVE PATH SETTINGS
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui|{
                                let left_size = Vec2::new(ui.available_size()[0]*0.3, ui.available_size()[1]);
                                let right_size = Vec2::new(ui.available_size()[0]*0.7, ui.available_size()[1]);
                                ui.allocate_ui_with_layout(left_size,Layout::top_down(Align::RIGHT), |ui|{
                                    ui.add_space(8.0);
                                    ui.label("Percorso di salvataggio");
                                });
                                ui.add_space(20.0);
                                ui.allocate_ui_with_layout(right_size,Layout::left_to_right(Align::TOP), |ui|{
                                    ui.spacing_mut().item_spacing.x = 10.0;
                                    let text_edit = TextEdit::singleline(&mut path).margin(Vec2::splat(8.0)).ui(ui);
                                    if text_edit.changed(){
                                        ctx.memory_mut(|mem|{
                                            mem.data.insert_temp(Id::from("path"), path.clone());
                                        })
                                    }
                                    let mut icon = RetainedImage::from_svg_bytes("", include_bytes!("../images/folder_black.svg")).unwrap();
                                    if _frame.info().system_theme.is_none() || _frame.info().system_theme == Some(Theme::Dark){
                                        icon = RetainedImage::from_svg_bytes("", include_bytes!("../images/folder_white.svg")).unwrap();
                                    }
                                    let button_dim = text_edit.rect.height() - 8.0;
                                    if ImageButton::new(icon.texture_id(ctx), Vec2::new(button_dim,button_dim)).ui(ui).clicked(){
                                        ctx.memory_mut(|mem|{
                                            mem.data.insert_temp(Id::from("path"), match FileDialog::new().show_open_single_dir() {
                                                Ok(p) => match p{
                                                        Some(path) => path.to_str().unwrap().to_string(),
                                                        None => path.clone(),
                                                    }
                                                Err(error) => {
                                                    panic!("{}", error);
                                                }
                                            });
                                        });
                                    };
                                });
                            });
                            ui.add_space(30.0);

                            //DEFAULT FILE NAME
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui|{
                                let left_size = Vec2::new(ui.available_size()[0]*0.3, ui.available_size()[1]);
                                let right_size = Vec2::new(ui.available_size()[0]*0.7, ui.available_size()[1]);
                                ui.allocate_ui_with_layout(left_size,Layout::top_down(Align::RIGHT), |ui|{
                                    ui.add_space(8.0);
                                    ui.label("Nome File");
                                });
                                ui.add_space(20.0);
                                ui.allocate_ui_with_layout(right_size,Layout::top_down(Align::LEFT), |ui|{
                                    ui.spacing_mut().item_spacing.y = 10.0;
                                    ui.allocate_ui_with_layout(right_size,Layout::left_to_right(Align::TOP), |ui|{
                                        ui.spacing_mut().item_spacing.x = 10.0;
                                        let text_edit = TextEdit::singleline(&mut filename_pattern).margin(Vec2::splat(8.0)).ui(ui);
                                        if text_edit.changed(){
                                            ctx.memory_mut(|mem|{
                                                mem.data.insert_temp(Id::from("pattern"), filename_pattern.clone());
                                            })
                                        }
                                        ui.with_layout(Layout::top_down(Align::LEFT), |ui|{
                                            ui.add_space(6.0);
                                            ComboBox::from_label("")
                                                .selected_text(format.to_string())
                                                .width(50.0)
                                                .show_ui(ui, |ui| {
                                                    ui.selectable_value(&mut format, PNG, "PNG");
                                                    ui.selectable_value(&mut format, JPG, "JPG");
                                                    ui.selectable_value(&mut format, GIF, "GIF");
                                                });
                                        });

                                        ctx.memory_mut(|mem|{
                                            mem.data.insert_temp(Id::from("format"), format);
                                        });
                                    });

                                    ui.allocate_ui_with_layout(right_size,Layout::top_down(Align::LEFT), |ui|{
                                        for i in 0..TIME_BUTTONS.len() {
                                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                                                if ui.label(RichText::new(TIME_BUTTONS[i].to_string()).color(hex_color!("#005500")))
                                                    .interact(Sense::click())
                                                    .on_hover_cursor(CursorIcon::PointingHand)
                                                    .clicked()
                                                {
                                                    filename_pattern.push_str(TIME_BUTTONS[i]);
                                                }
                                                ui.label(TIME_STRINGS[i]);
                                            });
                                        }
                                        ctx.memory_mut(|mem|{
                                            mem.data.insert_temp(Id::from("pattern"), filename_pattern.clone());
                                        })
                                    })
                                });

                            });
                            ui.add_space(30.0);
                        });
                }
                Tab::Shortcuts => {
                    CentralPanel::default()
                        .frame(Frame{ fill: bg_color, inner_margin: Margin::same(20.0), ..Default::default()})
                        .show(ctx, |ui|{
                            ctx.available_rect();
                            ui.with_layout(Layout::top_down(Align::Center), |ui|{
                                ui.spacing_mut().item_spacing.y = 10.0;
                                ui.heading(RichText::new("Scorciatoie").size(24.0));
                            });
                            ui.add_space(20.0);
                            
                            let mut hot_key_map_vec: Vec<(AcquireMode, KeyCombo)> = hot_key_map.clone().into_iter().collect();
                            hot_key_map_vec.sort_by_key(|(am, _kc)| *am);
                            
                            for (am , mut kc) in hot_key_map_vec{
                                ui.with_layout(Layout::left_to_right(Align::TOP), |ui|{
                                    let left_size = Vec2::new(ui.available_size()[0]*0.3, ui.available_size()[1]);
                                    let right_size = Vec2::new(ui.available_size()[0]*0.7, ui.available_size()[1]);
                                    ui.allocate_ui_with_layout(left_size,Layout::top_down(Align::RIGHT), |ui|{
                                        ui.add_space(8.0);
                                        ui.label(am.to_string());
                                    });
                                    ui.add_space(20.0);
                                    
                                    ui.allocate_ui_with_layout(right_size,Layout::left_to_right(Align::TOP), |ui|{
                                        ui.spacing_mut().item_spacing.x = 10.0;
                                        let mut text_edit = TextEdit::singleline(&mut kc.to_string()).margin(Vec2::splat(8.0)).ui(ui);
                                        if text_edit.has_focus(){
                                            if !kc.contains_key(){
                                                kc = ctx.input(|i|{
                                                    KeyCombo::new(i.modifiers.clone(), i.keys_down.clone().iter().nth(0).map(|k| k.clone()))
                                                });
                                                let kc_exists = hot_key_map.values().any(|combo| {
                                                    combo == &kc
                                                });
                                                if !kc_exists {
                                                    hot_key_map.insert(am, kc.clone());
                                                    ctx.memory_mut(|mem|{
                                                        mem.data.insert_temp(Id::from("hot_key_map"), hot_key_map.clone());
                                                    })
                                                }
                                            }
                                            else {
                                                text_edit = text_edit.interact(Sense::hover());
                                            }
                                        }
                                        let mut icon = RetainedImage::from_svg_bytes("", include_bytes!("../images/delete_black.svg")).unwrap();
                                        if _frame.info().system_theme.is_none() || _frame.info().system_theme == Some(Theme::Dark){
                                            icon = RetainedImage::from_svg_bytes("", include_bytes!("../images/delete_white.svg")).unwrap();
                                        }
                                        let button_dim = text_edit.rect.height() - 8.0;
                                        if ImageButton::new(icon.texture_id(ctx), Vec2::new(button_dim,button_dim)).ui(ui).clicked(){
                                            hot_key_map.insert(am, KeyCombo::default() );
                                            ctx.memory_mut(|mem|{
                                                mem.data.insert_temp(Id::from("hot_key_map"), hot_key_map.clone());
                                            })
                                        };
                                    });
                                });
                            ui.add_space(10.0);
                            }
                        });
                }
            }

            TopBottomPanel::bottom("bottom")
                .frame(Frame{ fill: bg_color, inner_margin: Margin::same(20.0), ..Default::default()})
                .show_separator_line(false)
                .show(ctx, |ui|{
                    ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                        ui.spacing_mut().item_spacing.x = 10.0;
                        ui.spacing_mut().button_padding = Vec2::new(10.0, 10.0);

						if Button::new("Conferma").ui(ui).clicked(){

                            ctx.memory_mut(|mem| {
								mem.data.clear();
							});
                            if !save_region{
                                region = None;
                            }

                            match self.configuration.write(){
                                Ok(mut c) => {
                                    c.bulk(None, Some(path.clone()), Some(filename_pattern.clone()), Some(format),
                                           Some(save_region), Some(region), Some(delay), Some(when_acquire), Some(hot_key_map.clone()));
                                }
                                Err(error) => {
                                    panic!("{}", error);
                                }
                            }

                            match self.register_hot_keys(){
                                Ok(()) => {}
                                Err(error) => {
                                    error!("{}", error);
                                }
                            }

							self.set_win_type(Main);
						}
						if Button::new("Applica").ui(ui).clicked(){

                            ctx.memory_mut(|mem| {
                                if let Some(tab) = mem.data.get_temp::<Tab>(Id::from("tab")){
								    mem.data.clear();
                                    mem.data.insert_temp(Id::from("tab"), tab.clone());
                                }
							});
                            if !save_region{
                                region = None;
                            }

                            match self.configuration.write(){
                                Ok(mut c) => {
                                    c.bulk(None, Some(path.clone()), Some(filename_pattern.clone()), Some(format),
                                           Some(save_region), Some(region), Some(delay), Some(when_acquire), Some(hot_key_map.clone()));
                                }
                                Err(error) => {
                                    panic!("{}", error);
                                }
                            }
                            match self.register_hot_keys(){
                                Ok(()) => {}
                                Err(error) => {
                                    error!("{}", error);
                                }
                            }
                        }
						if Button::new("Annulla").ui(ui).clicked(){
							ctx.memory_mut(|mem| {
								mem.data.clear();
							});
							self.set_win_type(Main);
						}
					});
				})
		});
	}
}

pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);

    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if response.clicked() {
        *on = !*on;
        response.mark_changed(); // report back that the value changed
    }

    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

#[allow(dead_code)]
fn toggle_ui_compact(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}
