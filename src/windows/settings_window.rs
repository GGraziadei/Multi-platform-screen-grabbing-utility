use eframe::Theme;
use egui::{Align, Button, Context, Layout, SidePanel, Vec2, Frame, Widget, Margin, hex_color, TopBottomPanel, CentralPanel, Area, Align2, Color32, Order, LayerId, Id, pos2, TextStyle, RichText, Stroke};
use egui::plot::Text;
use egui_extras::RetainedImage;
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

		_frame.set_window_size(Vec2::new(800.0, 600.0));
		_frame.set_centered();
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
					ui.with_layout(Layout::top_down(Align::LEFT), |ui|{
					});
				});
				}
				Tab::Save => {
					CentralPanel::default()
				.frame(Frame{ fill: bg_color, inner_margin: Margin::same(20.0), ..Default::default()})
				.show(ctx, |ui|{
					ctx.available_rect();
					ui.with_layout(Layout::top_down(Align::Center), |ui|{
						ui.spacing_mut().item_spacing.y = 10.0;
						ui.heading(RichText::new("Salvataggio").size(24.0));
					});
					ui.add_space(20.0);
					ui.with_layout(Layout::top_down(Align::LEFT), |ui|{
					});
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
							self.set_win_type(Main);
						}
						Button::new("Applica").rounding(8.0).ui(ui);
						if Button::new("Annulla").rounding(8.0).ui(ui).clicked(){
							self.set_win_type(Main);
						}
					});
				})
		});
	}
}
