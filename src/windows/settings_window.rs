use eframe::Theme;
use egui::{Align, Button, Context, Layout, SidePanel, Vec2, Frame, Widget, Margin, hex_color, TopBottomPanel, CentralPanel, Area, Align2, Color32, Order, LayerId, Id, pos2, TextStyle, RichText, Stroke, Direction, TextEdit, ImageButton, Rect, text_edit, Slider, ComboBox, Sense, CursorIcon};
use egui_extras::RetainedImage;
use log::info;
use native_dialog::FileDialog;
use crate::configuration::{AcquireAction, ImageFmt};
use crate::configuration::ImageFmt::{GIF, JPG, PNG};
use crate::window::Content;
use crate::window::WindowType::Main;

#[derive(Clone)]
enum Tab {
	General,
	Save,
	Shortcuts
}

impl Content {
	pub fn settings_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
    let bg_color = ctx.style().visuals.panel_fill;
    let (mut r, mut g, mut b, mut a) = bg_color.to_tuple();
		let selected_color = ctx.style().visuals.widgets.active.bg_fill;
		let mut path = match ctx.memory(|mem| mem.data.get_temp::<String>(Id::from("path"))) {
			Some(s) => s,
			None => self.configuration.read().unwrap().get_save_path().unwrap()
		};
		let mut jpeg_quality = match ctx.memory(|mem| mem.data.get_temp::<u8>(Id::from("quality"))) {
			Some(q) => q,
			// None => self.configuration.read().unwrap().get_jpeg_quality().unwrap()
			None => 100
		};
		let mut filename_pattern = match ctx.memory(|mem| mem.data.get_temp::<String>(Id::from("pattern"))) {
			Some(s) => s,
			None => self.configuration.read().unwrap().get_filename_pattern().unwrap()
		};
		let mut format = match ctx.memory(|mem| mem.data.get_temp::<ImageFmt>(Id::from("format"))) {
			Some(format) => format,
			None => self.configuration.read().unwrap().get_image_fmt().unwrap(),
		};
		let mut when_acquire = match ctx.memory(|mem| mem.data.get_temp::<AcquireAction>(Id::from("when_acquire"))) {
			Some(wa) => wa,
			None => self.configuration.read().unwrap().get_when_capture().unwrap()
		};
		let mut save_region = match ctx.memory(|mem| mem.data.get_temp::<bool>(Id::from("save_region"))) {
			Some(sr) => sr,
			None => self.configuration.read().unwrap().get_save_region().unwrap()
		};


		_frame.set_window_size(Vec2::new(800.0, 400.0));
		r = r - 10;
		g = g - 10;
		b = b - 10;

		let bg_dark_color = Color32::from_rgba_unmultiplied(r,g,b,a);

		let mut tab = ctx.memory_mut(|mem|{
			if mem.data.get_temp::<Tab>(Id::from("tab")).is_none(){
				mem.data.insert_temp(Id::from("tab"), Tab::General);
				Tab::General
			}
			else {
				mem.data.get_temp::<Tab>(Id::from("tab")).unwrap()
			}
		});

		CentralPanel::default().show(ctx, |ui| {
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
									if pos.x > rect1.min.x && pos.x < rect1.max.x && pos.y > rect1.min.y && pos.y < rect1.max.y {
										if i.pointer.primary_clicked(){
											tab = Tab::General;
											tab_changed = true;
										}
										return 1_u8
									}
									else if pos.x > rect2.min.x && pos.x < rect2.max.x && pos.y > rect2.min.y && pos.y < rect2.max.y {
										if i.pointer.primary_clicked(){
											tab = Tab::Save;
											tab_changed = true;
										}
										return 2_u8
									}
									else if pos.x > rect3.min.x && pos.x < rect3.max.x && pos.y > rect3.min.y && pos.y < rect3.max.y {
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
								colors[(hover-1) as usize].linear_multiply(2.0)
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
								if _frame.info().system_theme.is_none() || _frame.info().system_theme.unwrap() == Theme::Dark{
									icon = RetainedImage::from_svg_bytes("", include_bytes!("../images/folder_white.svg")).unwrap();
								}
								let button_dim = text_edit.rect.height() - 8.0;
								if ImageButton::new(icon.texture_id(ctx), Vec2::new(button_dim,button_dim)).ui(ui).clicked(){
									let new_path = match FileDialog::new().show_open_single_dir().unwrap(){
										Some(path) => path.to_str().unwrap().to_string(),
										None => path.clone(),
									};
									ctx.memory_mut(|mem|{
										mem.data.insert_temp(Id::from("path"), new_path.clone());
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
									let time_buttons = vec!["%Y", "%m", "%d", "%H", "%M", "%S"];
									let time_strings = vec!["Anno", "Mese", "Giorno", "Ora", "Minuto", "Secondo"];
										for (i, time_button) in time_buttons.iter().enumerate() {
											ui.allocate_ui_with_layout(right_size,Layout::left_to_right(Align::TOP), |ui| {
												if ui.label(RichText::new(time_button.to_string()).color(hex_color!("#005500")))
													.interact(Sense::click())
													.on_hover_cursor(CursorIcon::PointingHand)
													.clicked() {
													filename_pattern.push_str(time_button);
												}
												ui.label(time_strings[i]);
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
						ui.with_layout(Layout::top_down(Align::LEFT), |ui|{
						});
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

						if Button::new("Conferma").rounding(8.0).ui(ui).clicked(){
							ctx.memory_mut(|mem| {
								mem.data.remove::<Tab>(Id::from("tab"));
								mem.data.remove::<String>(Id::from("path"))
							});
							self.set_win_type(Main);
						}
						Button::new("Applica").rounding(8.0).ui(ui);
						if Button::new("Annulla").rounding(8.0).ui(ui).clicked(){
							ctx.memory_mut(|mem| {
								mem.data.remove::<Tab>(Id::from("tab"));
								mem.data.remove::<String>(Id::from("path"))
							});
							self.set_win_type(Main);
						}
					});
				})
		});
	}
}

pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    // Widget code can be broken up in four steps:
    //  1. Decide a size for the widget
    //  2. Allocate space for it
    //  3. Handle interactions with the widget (if any)
    //  4. Paint the widget

    // 1. Deciding widget size:
    // You can query the `ui` how much space is available,
    // but in this example we have a fixed size widget based on the height of a standard button:
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);

    // 2. Allocating space:
    // This is where we get a region of the screen assigned.
    // We also tell the Ui to sense clicks in the allocated region.
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    // 3. Interact: Time to check for clicks!
    if response.clicked() {
        *on = !*on;
        response.mark_changed(); // report back that the value changed
    }

    // Attach some meta-data to the response which can be used by screen readers:
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    // 4. Paint!
    // Make sure we need to paint:
    if ui.is_rect_visible(rect) {
        // Let's ask for a simple animation from egui.
        // egui keeps track of changes in the boolean associated with the id and
        // returns an animated value in the 0-1 range for how much "on" we are.
        let how_on = ui.ctx().animate_bool(response.id, *on);
        // We will follow the current style by asking
        // "how should something that is being interacted with be painted?".
        // This will, for instance, give us different colors when the widget is hovered or clicked.
        let visuals = ui.style().interact_selectable(&response, *on);
        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        // Paint the circle, animating it from left to right with `how_on`:
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}

/// Here is the same code again, but a bit more compact:
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

// A wrapper that allows the more idiomatic usage pattern: `ui.add(toggle(&mut my_bool))`
/// iOS-style toggle switch.
///
/// ## Example:
/// ``` ignore
/// ui.add(toggle(&mut my_bool));
/// ```
pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}
