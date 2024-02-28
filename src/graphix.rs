use once_cell::sync::OnceCell;
use std::fs::File;
use std::io::Write;

use skia_safe::{surfaces, Color, EncodedImageFormat, Paint, PaintStyle, Path, Font, FontMgr, FontStyle, Typeface};


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

pub fn render_file() {
    let mut surface = surfaces::raster_n32_premul((500, 500)).expect("surface");
    let mut canvas = surface.canvas();

    let mut paint = Paint::default();
    paint.set_color(Color::YELLOW);
    paint.set_anti_alias(true);
    paint.set_stroke_width(1.0);
    

    canvas.draw_str("Абоба", (200, 200), &Font::from_typeface(default_typeface(), 80.0), &paint);

    canvas.scale((1.2, 1.2));
    let mut path1 = Path::new();
    canvas.draw_path(&path1, &paint);
    path1.move_to((36.0, 48.0));
    path1.quad_to((660.0, 880.0), (1200.0, 360.0));
    canvas.translate((10.0, 10.0));

    paint.set_stroke_width(20.0);
    paint.set_style(PaintStyle::Stroke);
    canvas.draw_path(&path1, &paint);
    canvas.save();
    // canvas.move_to(30.0, 90.0);
    // canvas.line_to(110.0, 20.0);
    // canvas.line_to(240.0, 130.0);
    // canvas.line_to(60.0, 130.0);
    // canvas.line_to(190.0, 20.0);
    // canvas.line_to(270.0, 90.0);
    // canvas.fill();
    let image = surface.image_snapshot();
    let mut context = surface.direct_context();
    let d = image
        .encode(context.as_mut(), EncodedImageFormat::PNG, None)
        .unwrap();
    let mut file = File::create("test.png").unwrap();
    let bytes = d.as_bytes();
    file.write_all(bytes).unwrap();
}

