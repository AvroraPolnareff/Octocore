use once_cell::sync::OnceCell;
use rgb565::Rgb565;

use skia_safe::{surfaces, Color, Paint, PaintStyle, Path, Font, FontMgr, FontStyle, Typeface, ImageInfo, ColorType, AlphaType, ColorSpace};


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
static XOR_ENCODE_VALUES: [u8; 4] = [0xE7, 0xF3, 0xE7, 0xFF];

pub fn render_file() -> [u8; 2048 * 160] {
  let image_info = ImageInfo::new((960, 160), ColorType::RGB565, AlphaType::Opaque, ColorSpace::new_srgb());
  let mut surface = surfaces::raster(&image_info, 2048usize, None).expect("surface");
  let canvas = surface.canvas();

  let mut paint = Paint::default();
  paint.set_color(Color::GREEN);
  paint.set_anti_alias(true);
  paint.set_stroke_width(1.0);
  let mut paint2 = Paint::default();
  paint2.set_color(Color::BLUE);


  canvas.draw_str("Абоба", (500, 100), &Font::from_typeface(default_typeface(), 80.0), &paint2);

  canvas.scale((1.0, 1.0));
  let mut path1 = Path::new();
  path1.move_to((-50.0, -20.0));
  path1.quad_to((50.00, 50.0), (800.0, 120.0));
  canvas.translate((10.0, 10.0));
  paint.set_stroke_width(20.0);
  paint.set_style(PaintStyle::Stroke);
  canvas.draw_path(&path1, &paint);

  canvas.save();
  let mut pixels: [u8; 2048 * 160] = [0; 2048 * 160];
  let dest_row = 2048usize;
  surface.read_pixels(&image_info, &mut pixels, dest_row, (0, 0));
  for line in pixels.chunks_mut(960) {
    for frame in line.chunks_exact_mut(2) {
      let pixel = Rgb565::from_rgb565_le([frame[0], frame[1]]).to_bgr565_le();
      frame[0] = pixel[0];
      frame[1] = pixel[1]

    }
    for frame in line.chunks_exact_mut(4) {
      frame[0] = XOR_ENCODE_VALUES[0] ^ frame[0];
      frame[1] = XOR_ENCODE_VALUES[1] ^ frame[1];
      frame[2] = XOR_ENCODE_VALUES[2] ^ frame[2];
      frame[3] = XOR_ENCODE_VALUES[3] ^ frame[3];
    }
  }
  pixels
}

