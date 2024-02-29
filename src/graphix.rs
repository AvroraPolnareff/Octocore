use once_cell::sync::OnceCell;

use skia_safe::{surfaces, Color, Paint, PaintStyle, Path, Font, FontMgr, FontStyle, Typeface, ImageInfo, ColorType, AlphaType, ColorSpace};
use crate::voice_params::VoiceParams;


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

pub fn render_image(params: &VoiceParams, pixels: &mut [u8; 2048 * 160]) -> [u8; 2048 * 160] {
  let image_info = ImageInfo::new((960, 160), ColorType::RGB565, AlphaType::Opaque, ColorSpace::new_srgb());
  let mut surface = surfaces::raster(&image_info, 2048usize, None).expect("surface");
  let canvas = surface.canvas();

  let mut paint = Paint::default();
  paint.set_color(Color::GREEN);
  paint.set_anti_alias(true);
  paint.set_stroke_width(1.0);
  let mut paint2 = Paint::default();
  paint2.set_color(Color::BLUE);


  canvas.draw_str(format!("{:.2}", params.op2.volume.value()), (300, 100), &Font::from_typeface(default_typeface(), 80.0), &paint2);
  canvas.draw_str(format!("{:.2}", params.op2.ratio.value()), (500, 100), &Font::from_typeface(default_typeface(), 80.0), &paint2);

  canvas.scale((1.0, 1.0));
  let mut path1 = Path::new();
  path1.move_to((-50.0, -20.0));
  path1.quad_to((50.00, 50.0), (800.0, 120.0));
  canvas.translate((10.0, 10.0));
  paint.set_stroke_width(20.0);
  paint.set_style(PaintStyle::Stroke);
  canvas.draw_path(&path1, &paint);

  canvas.save();

  let dest_row = 2048usize;
  surface.read_pixels(&image_info, pixels, dest_row, (0, 0));
  *pixels
}

