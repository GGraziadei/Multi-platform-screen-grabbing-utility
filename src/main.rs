use std::ptr::null;
use eframe::{egui, Frame};
use eframe::epaint::Rounding;
use egui::{Area, CentralPanel, Color32, ColorImage, Context, hex_color, Id, ImageData, LayerId, Margin, Order, Pos2, pos2, RawInput, Rect, Sense, SidePanel, Stroke, Style, TextStyle, Ui, Vec2, Visuals, Widget, Window};
use egui::epaint::Shadow;
use egui_extras::RetainedImage;


fn main() {
  let native_options = eframe::NativeOptions::default();
  eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc)))).unwrap();
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
fn new(cc: &eframe::CreationContext<'_>) -> Self {
    // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
    // Restore app state using cc.storage (requires the "persistence" feature).
    // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
    // for e.g. egui::PaintCallback.
    Self::default()
  }
}

impl eframe::App for MyEguiApp {
  fn update(&mut self, ctx: &Context, frame: &mut Frame) {

    let p_frame = egui::Frame{
      inner_margin: Default::default(),
      outer_margin: Default::default(),
      rounding: Default::default(),
      shadow: Default::default(),
      fill: Color32::GOLD,
      stroke: Default::default(),
    };


    CentralPanel::default().frame(p_frame).show(ctx, |ui| {
      let image = RetainedImage::from_image_bytes("screenshot.png", include_bytes!("screenshot.png")).unwrap();
      let win_max_x = frame.info().window_info.size.x;
      let win_max_y = frame.info().window_info.size.y;

      frame.set_decorations(false);
      frame.set_fullscreen(true);
      frame.set_maximized(true);

      let mut w_frame = egui::Frame {
        inner_margin: Default::default(),
        outer_margin: Default::default(),
        rounding: Rounding::same(4.0),
        shadow: Shadow::NONE,
        fill: hex_color!("#ababab"),
        stroke: Stroke::NONE
      };

      let w = Window::new("window")
        .frame(w_frame)
        .title_bar(false)
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_TOP, [0.0,15.0])
        .show(ctx, |ui| {
          ui.label("Hello World!");
          ui.set_min_width(0.9*win_max_x);
          // ui.painter_at()
        });


      let mut painter = ctx.layer_painter(LayerId::new(Order::Background, Id::new("image")));
      painter.set_clip_rect(Rect {min: pos2(0.1*win_max_x,0.1*win_max_y), max: pos2(0.9*win_max_x,0.9*win_max_y)});
      // painter.image(image.texture_id(ctx), Rect { min: pos2(0.0,0.0), max: pos2(frame.info().window_info.size.x,frame.info().window_info.size.y) }, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
      painter.image(image.texture_id(ctx), Rect { min: pos2(0.1*win_max_x,0.1*win_max_y), max: pos2(0.9*win_max_x,0.9*win_max_y) }, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);



      let mut init_pos = pos2(0.0,0.0);
      let mut actual_pos = ctx.input(|i| {
        if i.pointer.primary_down(){
          if i.pointer.press_origin().is_some(){
            init_pos = i.pointer.press_origin().unwrap();
          }
          if i.pointer.hover_pos().is_some(){
            i.pointer.hover_pos().unwrap()
          }
          else { pos2(0.0,0.0) }
        }
        else { pos2(0.0,0.0) }
      });
      if actual_pos != pos2(0.0,0.0) {
        if actual_pos.x < init_pos.x{
          let tmp = init_pos.x;
          init_pos.x = actual_pos.x;
          actual_pos.x = tmp;
        }
        if actual_pos.y < init_pos.y {
          let tmp = init_pos.y;
          init_pos.y = actual_pos.y;
          actual_pos.y = tmp;
        }
        painter.rect_filled(Rect { min: pos2(0.0,0.0), max: pos2(init_pos.x, win_max_y) }, 0.0,  hex_color!("#00000064"));
        painter.rect_filled(Rect { min: pos2(init_pos.x, 0.0), max: pos2(actual_pos.x, init_pos.y) }, 0.0, hex_color!("#00000064"));
        painter.rect_filled(Rect { min: pos2(init_pos.x, actual_pos.y), max: pos2(actual_pos.x, win_max_y) }, 0.0, hex_color!("#00000064"));
        painter.rect_filled(Rect { min: pos2(actual_pos.x, 0.0), max: pos2(win_max_x, win_max_y) }, 0.0, hex_color!("#00000064"));
        painter.rect_filled(Rect { min: init_pos, max: actual_pos }, 0.0, Color32::TRANSPARENT);
        println!("{:?}", init_pos);
        println!("{:?}", actual_pos);
      }
      else {
        painter.rect_filled(Rect::from_min_max(pos2(0.0,0.0), pos2(frame.info().window_info.size.x,frame.info().window_info.size.y)), 0.0, hex_color!("#00000064"));
      }
    });
  }
}
