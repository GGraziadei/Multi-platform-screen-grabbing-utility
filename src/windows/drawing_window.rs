use eframe::emath::Rect;
use eframe::Theme;
use egui::{Align, CentralPanel, Color32, Context, Frame, Id, LayerId, Layout, Margin, Order, pos2, TopBottomPanel, Vec2, Stroke, Pos2, Button, Widget, hex_color};
use egui_extras::RetainedImage;
use crate::window::Content;
use crate::windows::drawing_window::Drawing::{Arrow, Circle, Free, Line, Rectangle};

#[derive(Clone, Debug, PartialEq)]
enum Drawing {
    Rectangle { r: Rect, s: Stroke, f: bool },
    Circle { c: Pos2, r: f32, s: Stroke, f: bool },
    Line { p1: Pos2, p2: Pos2, s: Stroke },
    Arrow { p: Pos2, v: Vec2, s: Stroke },
    Free { points: Vec<Pos2>, s: Stroke, complete: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
enum DrawingMode {
    Line,
    Rectangle,
    Circle,
    Arrow,
    Free
}

impl Content{
	pub fn drawing_window(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
        //let monitor_size = _frame.info().window_info.monitor_size.unwrap();
        let bg_color = ctx.style().visuals.panel_fill;
        let green = hex_color!("#16A085");
        let border_color = ctx.style().visuals.widgets.inactive.bg_stroke.color;
        let mut drawings = match ctx.memory(|mem| mem.data.get_temp::<Vec<Drawing>>(Id::from("drawings"))){
            Some(d) => d.clone(),
            None => Vec::<Drawing>::new(),
        };
        
        let drawing_mode = match ctx.memory(|mem| mem.data.get_temp::<DrawingMode>(Id::from("drawing_mode"))){
            Some(d) => d,
            None => DrawingMode::Free,
        };

        let fill = match ctx.memory(|mem| mem.data.get_temp::<bool>(Id::from("fill"))){
            Some(d) => d,
            None => false,
        };
        
        //TODO: REMOVE THIS
        let color = Color32::RED;
        let stroke = Stroke::new(1.0, color);
        
        
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
        
        let icon_size = Vec2::new(16.0,16.0);
        
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
        }

        // _frame.set_maximized(true);
        
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
                        }
                    }
                }
        
                match ctx.input(|i| i.pointer.hover_pos()){
                    Some(mut mouse_pos) => {
                        let hover_rect = match ctx.memory(|mem| mem.data.get_temp::<Rect>(Id::from("hover_rect"))){
                            Some(r) => r,
                            None => rect
                        };
                        if hover_rect.contains(mouse_pos){
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
                                            mem.data.insert_temp(Id::from("prev_pos"), mouse_pos);
                                            mem.data.insert_temp(Id::from("drawings"), drawings.clone());
                                            
                                        });
                                    }
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
                                    DrawingMode::Line => { drawings.push(Line {p1: init_pos, p2: mouse_pos, s: Stroke::new(1.0, Color32::RED)}); }
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
                                    DrawingMode::Arrow => { drawings.push(Arrow {p: init_pos, v: Vec2::new(mouse_pos.x - init_pos.x, mouse_pos.y - init_pos.y), s: Stroke::new(1.0, Color32::RED)}); }
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
                                }
        
                                ctx.memory_mut(|mem| {
                                    mem.data.insert_temp(Id::from("drawings"), drawings.clone());
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
