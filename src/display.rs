use once_cell::sync::OnceCell;

use skia_safe::{surfaces, Color, Paint, Font, FontMgr, FontStyle, Typeface, ImageInfo, ColorType, AlphaType, ColorSpace, Point, Canvas};
use crate::ui_state::{OpPage, Page, UIState};
use crate::synth_params::SynthParams;


pub fn default_typeface() -> Typeface {
  DEFAULT_TYPEFACE
    .get_or_init(|| {
      let font_mgr = FontMgr::new();
      font_mgr
        .legacy_make_typeface(None, FontStyle::default())
        .unwrap()
    })
    .clone()
}

static DEFAULT_TYPEFACE: OnceCell<Typeface> = OnceCell::new();

pub fn render_param(
  name: &str,
  value: String,
  point: impl Into<Point>,
  canvas: &Canvas
) {
  let Point {x, y} = point.into();
  let mut paint = Paint::default();
  paint.set_color(Color::WHITE);
  paint.set_anti_alias(true);
  paint.set_stroke_width(1.0);

  canvas.draw_str(
    name,
    (x, y),
    &Font::from_typeface(default_typeface(), 30.0),
    &paint
  );
  canvas.draw_str(
    value.as_str(),
    (x, y + 40.0),
    &Font::from_typeface(default_typeface(), 40.0),
    &paint
  );
}

fn fmt_float(f: f64) -> String { format!("{:.2}", f) }

pub fn render_image(params: &SynthParams, state: UIState, pixels: &mut [u8; 2048 * 160]) -> [u8; 2048 * 160] {
  let image_info = ImageInfo::new((960, 160), ColorType::RGB565, AlphaType::Opaque, ColorSpace::new_srgb());
  let mut surface = surfaces::raster(&image_info, 2048usize, None).expect("surface");
  let canvas = surface.canvas();

  let mut paint = Paint::default();
  paint.set_color(Color::GREEN);
  paint.set_anti_alias(true);
  paint.set_stroke_width(1.0);
  let mut paint2 = Paint::default();
  paint2.set_color(Color::BLUE);

  let page = state.page.lock().unwrap();
  let op_subpage = state.op_subpage.lock().unwrap();
  let dest = state.lfo_dest.lock().unwrap();
  let calc_param_pos = |ord: f32| (120. * ord - 120. / 2. - 40., 60.);
  match *page {
    Page::Op1 => {
      match *op_subpage {
        OpPage::Tone => {
          render_param("Volume", fmt_float(params.op1.volume.value()), calc_param_pos(1.), canvas);
          render_param("Ratio", fmt_float(params.op1.ratio.value()), calc_param_pos(2.), canvas);

        }
        OpPage::Amp => {
          render_param("Attack", fmt_float(params.op1.adsr_params.a.value()), calc_param_pos(1.), canvas);
          render_param("Decay", fmt_float(params.op1.adsr_params.d.value()), calc_param_pos(2.), canvas);
          render_param("Sustain", fmt_float(params.op1.adsr_params.s.value()), calc_param_pos(3.), canvas);
          render_param("Release", fmt_float(params.op1.adsr_params.r.value()), calc_param_pos(4.), canvas);
        }
      }
    }
    Page::Op2 => {
      match *op_subpage {
        OpPage::Tone => {
          render_param("Volume", fmt_float(params.op2.volume.value()), calc_param_pos(1.), canvas);
          render_param("Ratio", fmt_float(params.op2.ratio.value()), calc_param_pos(2.), canvas);

        }
        OpPage::Amp => {
          render_param("Attack", fmt_float(params.op2.adsr_params.a.value()), calc_param_pos(1.), canvas);
          render_param("Decay", fmt_float(params.op2.adsr_params.d.value()), calc_param_pos(2.), canvas);
          render_param("Sustain", fmt_float(params.op2.adsr_params.s.value()), calc_param_pos(3.), canvas);
          render_param("Release", fmt_float(params.op2.adsr_params.r.value()), calc_param_pos(4.), canvas);
        }
      }
    }
    Page::Modulation => {
      let dest = dest.to_owned().1;
      render_param("Op 2", dest.name, calc_param_pos(1.), canvas)
    }
    _ => {}
  }
  canvas.scale((1.0, 1.0));
  canvas.save();

  let dest_row = 2048usize;
  surface.read_pixels(&image_info, pixels, dest_row, (0, 0));
  *pixels
}

