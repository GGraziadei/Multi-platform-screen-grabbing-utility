use eframe::emath::Rect;
use eframe::epaint::Rgba;
use eframe::Theme;
use egui::{Align, CentralPanel, Color32, Context, Frame, Id, LayerId, Layout, Margin, Order, pos2, TopBottomPanel, Vec2, Stroke, Pos2, Button, Widget, hex_color, DragValue, Align2, FontId, FontFamily, KeyboardShortcut, Modifiers, Key};
use egui::color_picker::{Alpha, color_edit_button_rgba};
use egui_extras::RetainedImage;
use crate::window::Content;
use crate::window::WindowType::{Preview};
use crate::windows::drawing_window::Drawings::{Arrow, Circle, Free, Line, Numbers, Rectangle};

#[derive(Clone, Debug, PartialEq)]
pub enum Drawings {
    Line { p1: Pos2, p2: Pos2, s: Stroke },
    Rectangle { r: Rect, s: Stroke, f: bool },
    Circle { c: Pos2, r: f32, s: Stroke, f: bool },
    Arrow { p: Pos2, v: Vec2, s: Stroke },
    Free { points: Vec<Pos2>, s: Stroke, complete: bool },
    Numbers { p: Pos2,  n: u32, c: Rgba }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
enum DrawingMode {
    Line,
    Rectangle,
    Circle,
    Arrow,
    Free,
    Numbers
}

impl Content{
	pub fn drawing_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
        //let monitor_size = _frame.info().window_info.monitor_size.unwrap();
        let bg_color = ctx.style().visuals.panel_fill;
        let green = hex_color!("#16A085");
        let border_color = ctx.style().visuals.widgets.inactive.bg_stroke.color;
        let mut drawings = match ctx.memory(|mem| mem.data.get_temp::<Vec<Drawings>>(Id::from("drawings"))){
            Some(d) => d.clone(),
            None => Vec::<Drawings>::new(),
        };

        let mut drawings_redo = match ctx.memory(|mem| mem.data.get_temp::<Vec<Drawings>>(Id::from("drawings_redo"))){
            Some(d) => d.clone(),
            None => Vec::<Drawings>::new(),
        };
        
        let drawing_mode = match ctx.memory(|mem| mem.data.get_temp::<DrawingMode>(Id::from("drawing_mode"))){
            Some(d) => d,
            None => DrawingMode::Free,
        };

        let fill = match ctx.memory(|mem| mem.data.get_temp::<bool>(Id::from("fill"))){
            Some(d) => d,
            None => false,
        };
        
        let mut color = match ctx.memory(|mem| mem.data.get_temp::<Rgba>(Id::from("color"))){
            Some(c) => c,
            None => Rgba::from(Color32::RED)
        };

        let mut thickness = match ctx.memory(|mem| mem.data.get_temp::<f32>(Id::from("thickness"))){
            Some(t) => t,
            None => 1.0
        };
        
        let color_picker_open = match ctx.memory(|mem| mem.data.get_temp::<bool>(Id::from("color_picker_open"))){
            Some(c) => c,
            None => false
        };
        
        let circle_number = match ctx.memory(|mem| mem.data.get_temp::<u32>(Id::from("circle_number"))){
            Some(n) => n,
            None => 1
        };
        
        let stroke = Stroke::new(thickness, color);
        
        
        let mut draw_icon = RetainedImage::from_svg_bytes_with_size(
            "draw",
            include_bytes!("../images/draw_black.svg"),
            egui_extras::image::FitTo::Original).unwrap();
        let mut line_icon = RetainedImage::from_svg_bytes_with_size(
            "line",
            include_bytes!("../images/line_black.svg"),
            egui_extras::image::FitTo::Original).unwrap();
        let mut rectangle_icon = RetainedImage::from_svg_bytes_with_size(
            "rectangle",
            include_bytes!("../images/rectangle_black.svg"),
            egui_extras::image::FitTo::Original).unwrap();
        let mut rectangle_fill_icon = RetainedImage::from_svg_bytes_with_size(
            "rectangle_fill",
            include_bytes!("../images/rectangle_fill_black.svg"),
            egui_extras::image::FitTo::Original).unwrap();
        let mut circle_icon = RetainedImage::from_svg_bytes_with_size(
            "circle",
            include_bytes!("../images/circle_black.svg"),
            egui_extras::image::FitTo::Original).unwrap();
        let mut arrow_icon = RetainedImage::from_svg_bytes_with_size(
            "arrow",
            include_bytes!("../images/arrow_black.svg"),
            egui_extras::image::FitTo::Original).unwrap();
        let mut undo_icon = RetainedImage::from_svg_bytes_with_size(
            "undo",
            include_bytes!("../images/undo_black.svg"),
            egui_extras::image::FitTo::Original).unwrap();
        let mut redo_icon = RetainedImage::from_svg_bytes_with_size(
            "redo",
            include_bytes!("../images/redo_black.svg"),
            egui_extras::image::FitTo::Original).unwrap();
        let mut delete_all_icon = RetainedImage::from_svg_bytes_with_size(
            "delete_all",
            include_bytes!("../images/delete_all_black.svg"),
            egui_extras::image::FitTo::Original).unwrap();
        let mut save_icon = RetainedImage::from_svg_bytes_with_size(
            "save",
            include_bytes!("../images/save_black.svg"),
            egui_extras::image::FitTo::Original).unwrap();
        let mut close_icon = RetainedImage::from_svg_bytes_with_size(
            "close",
            include_bytes!("../images/close_black.svg"),
            egui_extras::image::FitTo::Original).unwrap();
        let mut counter_icon = RetainedImage::from_svg_bytes_with_size(
            "counter",
            include_bytes!("../images/counter_black.svg"),
            egui_extras::image::FitTo::Original).unwrap();
        
        let icon_size = Vec2::splat(16.0);


        if ctx.input_mut(|i| {
            let shortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Z);
            i.consume_shortcut(&shortcut)
        }){
            undo(ctx, drawings.clone(), drawings_redo.clone(), circle_number);
        }

        if ctx.input_mut(|i| {
            let shortcut_1 = KeyboardShortcut::new(Modifiers::COMMAND, Key::Y);
            let shortcut_2 = KeyboardShortcut::new(Modifiers {command: true, shift: true, ..Default::default()}, Key::Z);
            i.consume_shortcut(&shortcut_1) || i.consume_shortcut(&shortcut_2)
        }){
            redo(ctx, drawings.clone(), drawings_redo.clone(), circle_number);
        }
        
        if _frame.info().system_theme.is_none() || _frame.info().system_theme.unwrap() == Theme::Dark {
            draw_icon = RetainedImage::from_svg_bytes_with_size(
                "draw",
                include_bytes!("../images/draw_white.svg"),
                egui_extras::image::FitTo::Original).unwrap();
            line_icon = RetainedImage::from_svg_bytes_with_size(
                "line",
                include_bytes!("../images/line_white.svg"),
                egui_extras::image::FitTo::Original).unwrap();
            rectangle_icon = RetainedImage::from_svg_bytes_with_size(
                "rectangle",
                include_bytes!("../images/rectangle_white.svg"),
                egui_extras::image::FitTo::Original).unwrap();
            rectangle_fill_icon = RetainedImage::from_svg_bytes_with_size(
                "rectangle_fill",
                include_bytes!("../images/rectangle_fill_white.svg"),
                egui_extras::image::FitTo::Original).unwrap();
            circle_icon = RetainedImage::from_svg_bytes_with_size(
                "circle",
                include_bytes!("../images/circle_white.svg"),
                egui_extras::image::FitTo::Original).unwrap();
            arrow_icon = RetainedImage::from_svg_bytes_with_size(
                "arrow",
                include_bytes!("../images/arrow_white.svg"),
                egui_extras::image::FitTo::Original).unwrap();
            undo_icon = RetainedImage::from_svg_bytes_with_size(
                "undo",
                include_bytes!("../images/undo_white.svg"),
                egui_extras::image::FitTo::Original).unwrap();
            redo_icon = RetainedImage::from_svg_bytes_with_size(
                "redo",
                include_bytes!("../images/redo_white.svg"),
                egui_extras::image::FitTo::Original).unwrap();
            delete_all_icon = RetainedImage::from_svg_bytes_with_size(
                "delete_all",
                include_bytes!("../images/delete_all_white.svg"),
                egui_extras::image::FitTo::Original).unwrap();
            save_icon = RetainedImage::from_svg_bytes_with_size(
                "save",
                include_bytes!("../images/save_white.svg"),
                egui_extras::image::FitTo::Original).unwrap();
            close_icon = RetainedImage::from_svg_bytes_with_size(
                "close",
                include_bytes!("../images/close_white.svg"),
                egui_extras::image::FitTo::Original).unwrap();
            counter_icon = RetainedImage::from_svg_bytes_with_size(
                "counter",
                include_bytes!("../images/counter_white.svg"),
                egui_extras::image::FitTo::Original).unwrap();
        }

        _frame.set_maximized(true);
        
        let r_image = match self.get_colorimage(){
            Some(r) => {
                RetainedImage::from_color_image("screenshot", r)
            }
            None => {
                ctx.memory(|mem|{
                    let bytes = mem.data.get_temp::<Vec<u8>>(Id::from("bytes")).unwrap();
                    RetainedImage::from_image_bytes(
                        "screenshot",
                        bytes.as_slice()
                    ).unwrap()
                })
            }
		};
        TopBottomPanel::top("toolbar")
            .frame(Frame{inner_margin: Margin::same(10.0), fill: bg_color, ..Default::default()})
            .resizable(false)
            .show_separator_line(false)
            .show(ctx, |ui|{
                ui.with_layout(Layout::left_to_right(Align::LEFT), |ui|{
                    ui.spacing_mut().button_padding = Vec2::splat(10.0);
                    
                    if Button::image_and_text(draw_icon.texture_id(ctx), icon_size, "Libero")
                        .stroke(Stroke::new(1.0,
                        match drawing_mode{
                            DrawingMode::Free => green,
                            _ => border_color
                        }))
                        .ui(ui).clicked(){
                            ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("drawing_mode"), DrawingMode::Free));
                        };
                    if Button::image_and_text(line_icon.texture_id(ctx), icon_size, "Linea")
                        .stroke(Stroke::new(1.0,
                        match drawing_mode{
                            DrawingMode::Line => green,
                            _ => border_color
                        }))
                        .ui(ui).clicked(){
                            ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("drawing_mode"), DrawingMode::Line));
                        };
                    
                    if Button::image_and_text(rectangle_icon.texture_id(ctx), icon_size, "Rettangolo")
                        .stroke(Stroke::new(1.0,
                        match drawing_mode{
                            DrawingMode::Rectangle => green,
                            _ => border_color
                        }))
                        .ui(ui).clicked(){
                            ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("drawing_mode"), DrawingMode::Rectangle));
                        };
                    if Button::image_and_text(circle_icon.texture_id(ctx), icon_size, "Cerchio")
                        .stroke(Stroke::new(1.0,
                        match drawing_mode{
                            DrawingMode::Circle => green,
                            _ => border_color
                        }))
                        .ui(ui).clicked(){
                            ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("drawing_mode"), DrawingMode::Circle));
                        };
                    if Button::image_and_text(arrow_icon.texture_id(ctx), icon_size, "Freccia")
                        .stroke(Stroke::new(1.0,
                        match drawing_mode{
                            DrawingMode::Arrow => green,
                            _ => border_color
                        }))
                        .ui(ui).clicked(){
                            ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("drawing_mode"), DrawingMode::Arrow));
                        };
                    if Button::image_and_text(counter_icon.texture_id(ctx), icon_size, "Numeri")
                        .stroke(Stroke::new(1.0,
                        match drawing_mode{
                            DrawingMode::Numbers => green,
                            _ => border_color
                        }))
                        .ui(ui).clicked(){
                            ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("drawing_mode"), DrawingMode::Numbers));
                        };
                    ui.add_space(20.0);
                    if fill {
                        if Button::image_and_text(rectangle_fill_icon.texture_id(ctx), icon_size, "Pieno").ui(ui).clicked(){
                            ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("fill"), !fill));
                        };
                    }
                    else {
                        if Button::image_and_text(rectangle_icon.texture_id(ctx), icon_size, "Vuoto").ui(ui).clicked(){
                            ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("fill"), !fill));
                        };
                    }
                    
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui|{
                        ui.label("Colore");
                        let color_picker = color_edit_button_rgba(ui, &mut color, Alpha::BlendOrAdditive);
                        
                        if ctx.memory(|mem| mem.any_popup_open()) {
                            ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("color_picker_open"), true));
                        }
                        else {
                            ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("color_picker_open"), false));
                        }
                        if color_picker.changed() {
                            ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("color"), color));
                        }
                    });

                    ui.with_layout(Layout::left_to_right(Align::Center), |ui|{
                        ui.label("Spessore");
                        if DragValue::new(&mut thickness)
                            .speed(0.1)
                            .clamp_range(1.0..=10.0)
                            .ui(ui)
                            .changed()
                        {
                            ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("thickness"), thickness));
                        }
                    });
                    
                    ui.add_space(20.0);
                    
                    let undo_button = ui.add_enabled(drawings.len() > 0, Button::image_and_text(undo_icon.texture_id(ctx), icon_size, "Annulla"));
                    if undo_button.clicked(){
                        undo(ctx, drawings.clone(), drawings_redo.clone(), circle_number);
                    };

                    let redo_button = ui.add_enabled(drawings_redo.len() > 0, Button::image_and_text(redo_icon.texture_id(ctx), icon_size, "Rifai"));
                    if redo_button.clicked(){
                        redo(ctx, drawings.clone(), drawings_redo.clone(), circle_number);
                    };
                    
                    if Button::image_and_text(delete_all_icon.texture_id(ctx), icon_size, "Cancella").ui(ui).clicked(){
                        drawings.clear();
                        ctx.memory_mut(|mem| {
                            mem.data.insert_temp(Id::from("circle_number"), 1u32);
                            mem.data.insert_temp(Id::from("drawings"), drawings.clone());
                        });
                    }
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui|{
                        if Button::image_and_text(save_icon.texture_id(ctx), icon_size, "Salva").ui(ui).clicked(){
                            let rect = match ctx.memory(|mem| mem.data.get_temp::<Rect>(Id::from("rect"))){
                                Some(rect) => rect,
                                None => Rect::from_min_max(pos2(0.0, 0.0), pos2(0.0, 0.0))
                            };
                            
                            ctx.memory_mut(|mem|{
                                mem.data.remove::<DrawingMode>(Id::from("drawing_mode"));
                                mem.data.remove::<Rgba>(Id::from("color"));
                                mem.data.remove::<f32>(Id::from("thickness"));
                                mem.data.remove::<bool>(Id::from("fill"));
                            });
                            self.set_region(rect);
                            _frame.request_screenshot();
                        };
                        if Button::image_and_text(close_icon.texture_id(ctx), icon_size, "Esci").ui(ui).clicked(){
                           self.set_win_type(Preview);
                        }
                        
                    });
                    
                });
            });
        CentralPanel::default()
            .frame(Frame{inner_margin: Margin::same(0.0), fill: bg_color, ..Default::default()})
            .show(ctx, |ui|{
                let mut painter = ctx.layer_painter(LayerId::new(Order::Background, Id::from("")));
        
                let aspect_ratio = r_image.width() as f32 / r_image.height() as f32;
                let mut width = ui.available_width();
                let mut height = width / aspect_ratio;
                if height > ui.available_height() {
                    height = ui.available_height();
                    width = height * aspect_ratio;
                }
        
                let mut rect = ui.available_rect_before_wrap();
                if rect.width() > width {
                    rect.min.x += (rect.width() - width) / 2.0;
                    rect.max.x = rect.min.x + width;
                }
                if rect.height() > height {
                    rect.min.y += (rect.height() - height) / 2.0;
                    rect.max.y = rect.min.y + height;
                }
                rect.set_width(width);
                rect.set_height(height);
                
                ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("rect"), rect));
        
                painter.set_clip_rect(rect);
                painter.image(r_image.texture_id(ctx), rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
                
                for d in drawings.iter() {
                    match d.clone(){
                        Line {p1, p2, s} => {
                            painter.line_segment([p1, p2], s);
                        },
                        Rectangle {r, s, f} => {
                            if f {
                                painter.rect_filled(r, 0.0, s.color);
                            } else {
                                painter.rect_stroke(r, 0.0, s);
                            }
                        },
                        Circle {c, r, s, f} => {
                            if f {
                                painter.circle_filled(c, r, s.color);
                            } else {
                                painter.circle_stroke(c, r, s);
                            }
                        },
                        Arrow {p, v, s} => {
                            painter.arrow(p, v, s);
                        },
                        Free {points, s, ..} => {
                            for i in 1..points.len() {
                                painter.line_segment([points[i-1], points[i]], s);
                            }
                        },
                        Numbers{ p, n, c} => {
                            painter.circle_filled(p, 20.0, c);
                            
                            let diff = 3.0 - c.r() - c.g() - c.b();
                            let text_color = match diff {
                                d if d >= 2.0 => Color32::WHITE,
                                _ => Color32::BLACK
                            };
                            
                            painter.text(p, Align2::CENTER_CENTER, format!("{}", n), FontId::new(20.0, FontFamily::default()), text_color);
                        }
                    }
                }
        
                match ctx.input(|i| i.pointer.hover_pos()){
                    Some(mut mouse_pos) => {
                        let hover_rect = match ctx.memory(|mem| mem.data.get_temp::<Rect>(Id::from("hover_rect"))){
                            Some(r) => r,
                            None => rect
                        };
                        if hover_rect.contains(mouse_pos) && !color_picker_open{
                            
                            if ctx.input(|i| i.pointer.primary_clicked()){
                                match drawing_mode {
                                    DrawingMode::Numbers => {
                                        drawings.push(Numbers {p: mouse_pos, n: circle_number, c: color});
                                        ctx.memory_mut(|mem| {
                                            drawings_redo.clear();
                                            mem.data.insert_temp(Id::from("circle_number"), circle_number + 1);
                                            mem.data.insert_temp(Id::from("drawings"), drawings.clone());
                                            mem.data.insert_temp(Id::from("drawings_redo"), drawings_redo.clone());
                                        });
                                    },
                                    _ => {}
                                }
                            }
                            
                            if ctx.input(|i| i.pointer.primary_down()){
                                let mut init_pos = match ctx.memory(|mem| mem.data.get_temp(Id::from("init_pos"))){
                                    Some(p) => p,
                                    None => {
                                        let p = ctx.input(|i| i.pointer.press_origin()).unwrap();
                                        ctx.memory_mut(|mem| mem.data.insert_temp(Id::from("init_pos"), p));
                                        p
                                    }
                                };
                                
                                match drawing_mode {
                                    DrawingMode::Line => { painter.line_segment([init_pos, mouse_pos], stroke); }
                                    DrawingMode::Rectangle => {
                                        if mouse_pos.x < init_pos.x {
                                            let tmp = mouse_pos.x;
                                            mouse_pos.x = init_pos.x;
                                            init_pos.x = tmp;
                                        }
                                        if mouse_pos.y < init_pos.y {
                                            let tmp = mouse_pos.y;
                                            mouse_pos.y = init_pos.y;
                                            init_pos.y = tmp;
                                        }
                                        match fill {
                                            true => { painter.rect_filled(Rect::from_min_max(init_pos, mouse_pos), 0.0, color); }
                                            false => { painter.rect_stroke(Rect::from_min_max(init_pos, mouse_pos), 0.0, stroke); }
                                        }
                                    }
                                    DrawingMode::Circle => {
                                        if mouse_pos.x < init_pos.x {
                                            let tmp = mouse_pos.x;
                                            mouse_pos.x = init_pos.x;
                                            init_pos.x = tmp;
                                        }
                                        if mouse_pos.y < init_pos.y {
                                            let tmp = mouse_pos.y;
                                            mouse_pos.y = init_pos.y;
                                            init_pos.y = tmp;
                                        }
                                        let center = pos2(init_pos.x + (mouse_pos.x - init_pos.x) / 2.0, init_pos.y + (mouse_pos.y - init_pos.y) / 2.0);
                                        let radius = (mouse_pos.x - init_pos.x) / 2.0;
                                        match fill {
                                            true => { painter.circle_filled(center, radius, color); }
                                            false => { painter.circle_stroke(center, radius, stroke); }
                                        }
                                    }
                                    DrawingMode::Arrow => {
                                        painter.arrow(init_pos, Vec2::new(mouse_pos.x - init_pos.x, mouse_pos.y - init_pos.y), stroke);
                                    }
                                    DrawingMode::Free => {
                                        let prev_pos = match ctx.memory(|mem| mem.data.get_temp::<Pos2>(Id::from("prev_pos"))){
                                            Some(p) => p,
                                            None => init_pos
                                        };
                                        match drawings.last() {
                                            Some(d) => {
                                                match d.clone() {
                                                    Free {points,complete, .. } => {
                                                        if !complete {
                                                            let mut points = points.clone();
                                                            drawings.pop();
                                                            points.push(mouse_pos);
                                                            drawings.push(Free {points, s: stroke, complete: false});
                                                        }
                                                        else {
                                                            let mut points = Vec::new();
                                                            points.push(prev_pos);
                                                            points.push(mouse_pos);
                                                            drawings.push(Free {points, s: stroke, complete: false});
                                                        }
                                                    },
                                                    _ => {
                                                        let mut points = Vec::new();
                                                        points.push(prev_pos);
                                                        points.push(mouse_pos);
                                                        drawings.push(Free {points, s: stroke, complete: false});
                                                    }
                                                };
                                            },
                                            None => {
                                                let mut points = Vec::new();
                                                points.push(prev_pos);
                                                points.push(mouse_pos);
                                                drawings.push(Free {points, s: stroke, complete: false});
                                            }
                                        };
                                        ctx.memory_mut(|mem| {
                                            drawings_redo.clear();
                                            mem.data.insert_temp(Id::from("prev_pos"), mouse_pos);
                                            mem.data.insert_temp(Id::from("drawings"), drawings.clone());
                                            mem.data.insert_temp(Id::from("drawings_redo"), drawings_redo.clone());
                                        });
                                    }
                                    _ => {}
                                }
                                ctx.memory_mut(|mem| {
                                    mem.data.insert_temp(Id::from("mouse_pos"), mouse_pos);
                                    let frame_size = _frame.info().window_info.size;
                                    mem.data.insert_temp(Id::from("hover_rect"), Rect::from_min_size(pos2(0.0,0.0), frame_size));
                                });
                            }
                            
                            if ctx.input(|i| i.pointer.primary_released()){
                                let mut init_pos = match ctx.memory(|mem| mem.data.get_temp::<Pos2>(Id::from("init_pos"))){
                                    Some(p) => p,
                                    None => pos2(0.0, 0.0)
                                };
                                
                                match drawing_mode {
                                    DrawingMode::Line => { drawings.push(Line {p1: init_pos, p2: mouse_pos, s: stroke}); }
                                    DrawingMode::Rectangle => {
                                        if mouse_pos.x < init_pos.x {
                                            let tmp = mouse_pos.x;
                                            mouse_pos.x = init_pos.x;
                                            init_pos.x = tmp;
                                        }
                                        if mouse_pos.y < init_pos.y {
                                            let tmp = mouse_pos.y;
                                            mouse_pos.y = init_pos.y;
                                            init_pos.y = tmp;
                                        }
                                        drawings.push(Rectangle {r: Rect::from_min_max(init_pos, mouse_pos), f: fill, s: stroke})
                                    }
                                    DrawingMode::Circle => {
                                        if mouse_pos.x < init_pos.x {
                                            let tmp = mouse_pos.x;
                                            mouse_pos.x = init_pos.x;
                                            init_pos.x = tmp;
                                        }
                                        if mouse_pos.y < init_pos.y {
                                            let tmp = mouse_pos.y;
                                            mouse_pos.y = init_pos.y;
                                            init_pos.y = tmp;
                                        }
                                        let center = pos2(init_pos.x + (mouse_pos.x - init_pos.x) / 2.0, init_pos.y + (mouse_pos.y - init_pos.y) / 2.0);
                                        let radius = (mouse_pos.x - init_pos.x) / 2.0;
                                        drawings.push(Circle {c: center, r: radius, f: fill, s: stroke})
                                    }
                                    DrawingMode::Arrow => { drawings.push(Arrow {p: init_pos, v: Vec2::new(mouse_pos.x - init_pos.x, mouse_pos.y - init_pos.y), s: stroke}); }
                                    DrawingMode::Free => {
                                        match drawings.last_mut().unwrap(){
                                            Free {points, s, complete} => {
                                                points.push(mouse_pos);
                                                *complete = true;
                                            },
                                            _ => {}
                                        }
                                        ctx.memory_mut(|mem| mem.data.remove::<Pos2>(Id::from("prev_pos")));
                                    }
                                    _ => {}
                                }
        
                                ctx.memory_mut(|mem| {
                                    drawings_redo.clear();
                                    mem.data.insert_temp(Id::from("drawings"), drawings.clone());
                                    mem.data.insert_temp(Id::from("drawings_redo"), drawings_redo.clone());
                                    mem.data.remove::<Pos2>(Id::from("init_pos"));
                                    mem.data.remove::<Rect>(Id::from("hover_rect"));
                                });
                            }
                        }
                    }
                    None => {}
                };
            });
    }
}
fn undo (ctx: &Context, mut drawings: Vec<Drawings>, mut redo: Vec<Drawings>, circle_number: u32){
    match drawings.last(){
        Some(d) => {
            match d {
                Numbers { .. } => {
                    redo.push(d.clone());
                    drawings.pop();
                    ctx.memory_mut(|mem| {
                        mem.data.insert_temp(Id::from("drawings"), drawings.clone());
                        mem.data.insert_temp(Id::from("drawings_redo"), redo.clone());
                        mem.data.insert_temp(Id::from("circle_number"), circle_number - 1);
                    });
            }
                _ => {
                    redo.push(d.clone());
                    drawings.pop();
                    ctx.memory_mut(|mem| {
                        mem.data.insert_temp(Id::from("drawings"), drawings.clone());
                        mem.data.insert_temp(Id::from("drawings_redo"), redo.clone())
                    });
                }
            }
        },
        None => ()
    }
}

fn redo (ctx: &Context, mut drawings: Vec<Drawings>, mut redo: Vec<Drawings>, circle_number: u32){
    match redo.last(){
        Some(d) => {
            match d {
                Numbers { .. } => {
                    drawings.push(d.clone());
                    redo.pop();
                    ctx.memory_mut(|mem| {
                        mem.data.insert_temp(Id::from("drawings"), drawings.clone());
                        mem.data.insert_temp(Id::from("drawings_redo"), redo.clone());
                        mem.data.insert_temp(Id::from("circle_number"), circle_number + 1);
                    });
            }
                _ => {
                    drawings.push(d.clone());
                    redo.pop();
                    ctx.memory_mut(|mem| {
                        mem.data.insert_temp(Id::from("drawings"), drawings.clone());
                        mem.data.insert_temp(Id::from("drawings_redo"), redo.clone())
                    });
                }
            }
        },
        None => ()
    }
}

