use once_cell::sync::OnceCell;
use std::fs::File;
use std::io::Write;

use skia_safe::{surfaces, Color, EncodedImageFormat, Paint, PaintStyle, Path, Font, FontMgr, FontStyle, Typeface, ImageInfo, ColorType, AlphaType};


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

pub fn render_file() -> [u8; 2048 * 160] {
    let mut surface = surfaces::raster_n32_premul((960, 160)).expect("surface");
    let mut canvas = surface.canvas();

    let mut paint = Paint::default();
    paint.set_color(Color::YELLOW);
    paint.set_anti_alias(true);
    paint.set_stroke_width(1.0);


    canvas.draw_str("Абоба", (500, 100), &Font::from_typeface(default_typeface(), 80.0), &paint);

    canvas.scale((1.2, 1.2));
    let mut path1 = Path::new();
    //canvas.draw_path(&path1, &paint);
    path1.move_to((36.0, 5.0));
    path1.quad_to((50.00, 50.0), (800.0, 120.0));
    canvas.translate((10.0, 10.0));

    paint.set_stroke_width(20.0);
    paint.set_style(PaintStyle::Stroke);
    canvas.draw_path(&path1, &paint);
    canvas.save();
    let image = surface.image_snapshot();
    let mut pixels: [u8; 2048 * 160] = [0; 2048 * 160];
    let dest_row = 2048usize;
    surface.read_pixels(&ImageInfo::new((960, 160), ColorType::RGB565, AlphaType::Opaque, None), &mut pixels, dest_row, (0, 0));
    pixels
}

