// Adapted from the file below:
// https://github.com/alexheretic/ab-glyph/blob/main/dev/examples/image.rs

use ab_glyph::{point, Font, FontVec, Glyph, Point, PxScale, ScaleFont};
use image::{DynamicImage, ImageBuffer, Rgba};

pub struct FontRenderer {
  font_path: String,
  font: Option<FontVec>,
}

impl FontRenderer {
  pub fn new(font_path: String) -> Self {
    let mut font_renderer = FontRenderer {
      font_path: "".to_string(),
      font: None,
    };
    font_renderer.load_font(font_path);

    font_renderer
  }

  pub fn load_font(&mut self, font_path: String) {
    println!("load_font: {}", font_path);
    let font_path = std::env::current_dir().unwrap().join(font_path);
    let data = std::fs::read(&font_path).unwrap();

    match &mut self.font {
      Some(font) => {
        println!("font already loaded");
      }
      None => {
        println!("loading font...");
        self.font = Some(FontVec::try_from_vec(data).unwrap_or_else(|_| {
          panic!("error constructing a Font from data at {:?}", font_path);
        }));
        if let Some(name) = font_path.file_name().and_then(|n| n.to_str()) {
          eprintln!("Using font: {name}");
        }

        self.font_path = font_path.to_str().unwrap().to_string();
      }
    }
  }

  pub fn render(&mut self, text: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // The font size to use
    let scale = PxScale::from(45.0);

    let font = self.font.as_ref().unwrap();
    let scaled_font = font.as_scaled(scale);

    let mut glyphs = Vec::new();
    self.layout_paragraph(scaled_font, point(20.0, 20.0), 9999.0, text, &mut glyphs);

    // Use a dark red colour
    let colour = (255, 0, 0);

    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
      let min_x = glyphs.first().unwrap().position.x;
      let last_glyph = glyphs.last().unwrap();
      let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
      (max_x - min_x).ceil() as u32
    };

    println!(
      "glyphs_width: {}, glyphs_height: {}",
      glyphs_width, glyphs_height
    );

    // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
      if let Some(outlined) = scaled_font.outline_glyph(glyph) {
        let bounds = outlined.px_bounds();
        // Draw the glyph into the image per-pixel by using the draw closure
        outlined.draw(|x, y, v| {
          // Offset the position by the glyph bounding box
          let px = image.get_pixel_mut(x + bounds.min.x as u32, y + bounds.min.y as u32);
          // Turn the coverage into an alpha value (blended with any previous)
          *px = Rgba([
            colour.0,
            colour.1,
            colour.2,
            px.0[3].saturating_add((v * 255.0) as u8),
          ]);
        });
      }
    }

    // Return the image buffer
    image
  }

  pub fn layout_paragraph<F, SF>(
    &self,
    font: SF,
    position: Point,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
  ) where
    F: Font,
    SF: ScaleFont<F>,
  {
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
      if c.is_control() {
        if c == '\n' {
          caret = point(position.x, caret.y + v_advance);
          last_glyph = None;
        }
        continue;
      }
      let mut glyph = font.scaled_glyph(c);
      if let Some(previous) = last_glyph.take() {
        caret.x += font.kern(previous.id, glyph.id);
      }
      glyph.position = caret;

      last_glyph = Some(glyph.clone());
      caret.x += font.h_advance(glyph.id);

      if !c.is_whitespace() && caret.x > position.x + max_width {
        caret = Point {
          x: position.x,
          y: caret.y + v_advance,
        };
        glyph.position = caret;
        last_glyph = None;
      }

      target.push(glyph);
    }
  }
}
