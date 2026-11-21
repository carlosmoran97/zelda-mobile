//! MIDP drawing semantics implemented on native RGBA surfaces.
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub const WIDTH: i32 = 240;
pub const HEIGHT: i32 = 320;
pub type Surface = Rc<RefCell<Bitmap>>;

#[derive(Clone)]
pub struct Bitmap {
    pub w: i32,
    pub h: i32,
    pub pixels: Vec<u32>,
}
impl Bitmap {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            w,
            h,
            pixels: vec![0xffffffff; (w * h) as usize],
        }
    }
    pub fn load(path: &std::path::Path) -> Result<Self, String> {
        let im = image::open(path)
            .map_err(|e| format!("{}: {e}", path.display()))?
            .into_rgba8();
        Ok(Self {
            w: im.width() as i32,
            h: im.height() as i32,
            pixels: im
                .as_raw()
                .chunks_exact(4)
                .map(|p| u32::from_le_bytes(p.try_into().unwrap()))
                .collect(),
        })
    }
    pub fn save(&self, path: &std::path::Path) -> Result<(), String> {
        let bytes: Vec<u8> = self.pixels.iter().flat_map(|p| p.to_le_bytes()).collect();
        image::save_buffer(
            path,
            &bytes,
            self.w as u32,
            self.h as u32,
            image::ColorType::Rgba8,
        )
        .map_err(|e| e.to_string())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
impl Rect {
    pub fn intersect(self, b: Rect) -> Rect {
        let x = self.x.max(b.x);
        let y = self.y.max(b.y);
        Rect {
            x,
            y,
            w: (self.x + self.w).min(b.x + b.w).saturating_sub(x).max(0),
            h: (self.y + self.h).min(b.y + b.h).saturating_sub(y).max(0),
        }
    }
}

#[derive(Clone)]
pub struct Graphics {
    pub target: Surface,
    pub tx: i32,
    pub ty: i32,
    pub color: u32,
    pub clip: Rect,
    pub font: u32,
}
impl Graphics {
    pub fn new(target: Surface) -> Self {
        let b = target.borrow();
        let clip = Rect {
            x: 0,
            y: 0,
            w: b.w,
            h: b.h,
        };
        drop(b);
        Self {
            target,
            tx: 0,
            ty: 0,
            color: 0xff000000,
            clip,
            font: 0,
        }
    }
    pub fn reset(&mut self) {
        self.tx = 0;
        self.ty = 0;
        let b = self.target.borrow();
        self.clip = Rect {
            x: 0,
            y: 0,
            w: b.w,
            h: b.h,
        };
    }
    pub fn set_clip(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let b = self.target.borrow();
        self.clip = Rect {
            x: x + self.tx,
            y: y + self.ty,
            w: w.max(0),
            h: h.max(0),
        }
        .intersect(Rect {
            x: 0,
            y: 0,
            w: b.w,
            h: b.h,
        });
    }
    pub fn clip_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.clip = self.clip.intersect(Rect {
            x: x + self.tx,
            y: y + self.ty,
            w: w.max(0),
            h: h.max(0),
        });
    }
    pub fn color(&mut self, rgb: i32) {
        self.color = u32::from_le_bytes([
            ((rgb >> 16) & 255) as u8,
            ((rgb >> 8) & 255) as u8,
            (rgb & 255) as u8,
            255,
        ]);
    }
    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let q = self.clip.intersect(Rect {
            x: x + self.tx,
            y: y + self.ty,
            w: w.max(0),
            h: h.max(0),
        });
        if q.w <= 0 || q.h <= 0 {
            return;
        }
        let mut b = self.target.borrow_mut();
        let bw = b.w as usize;
        for y in q.y..q.y + q.h {
            let start = y as usize * bw + q.x as usize;
            b.pixels[start..start + q.w as usize].fill(self.color);
        }
    }
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let (mut x, mut y, x1, y1) = (x0 + self.tx, y0 + self.ty, x1 + self.tx, y1 + self.ty);
        let dx = (x1 - x).abs();
        let sx = if x < x1 { 1 } else { -1 };
        let dy = -(y1 - y).abs();
        let sy = if y < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut b = self.target.borrow_mut();
        loop {
            self.pixel(&mut b, x, y, self.color);
            if x == x1 && y == y1 {
                break;
            }
            let e = 2 * err;
            if e >= dy {
                err += dy;
                x += sx;
            }
            if e <= dx {
                err += dx;
                y += sy;
            }
        }
    }
    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if w < 0 || h < 0 {
            return;
        }
        self.draw_line(x, y, x + w, y);
        self.draw_line(x, y + h, x + w, y + h);
        self.draw_line(x, y, x, y + h);
        self.draw_line(x + w, y, x + w, y + h);
    }
    fn pixel(&self, b: &mut Bitmap, x: i32, y: i32, p: u32) {
        if x < self.clip.x
            || y < self.clip.y
            || x >= self.clip.x + self.clip.w
            || y >= self.clip.y + self.clip.h
            || x < 0
            || y < 0
            || x >= b.w
            || y >= b.h
        {
            return;
        }
        let idx = (y * b.w + x) as usize;
        let a = p >> 24;
        if a == 255 {
            b.pixels[idx] = p;
        } else if a > 0 {
            let src = p.to_le_bytes();
            let dst = b.pixels[idx].to_le_bytes();
            let c = |i: usize| ((src[i] as u32 * a + dst[i] as u32 * (255 - a) + 127) / 255) as u8;
            b.pixels[idx] = u32::from_le_bytes([c(0), c(1), c(2), 255]);
        }
    }
    pub fn blit(
        &mut self,
        src: &Surface,
        region: Rect,
        transform: i32,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<(), String> {
        let Rect { x: sx, y: sy, w, h } = region;
        if w < 0 || h < 0 {
            return Err("negative source region".into());
        }
        let cloned;
        let src = if Rc::ptr_eq(src, &self.target) {
            cloned = Rc::new(RefCell::new(src.borrow().clone()));
            &cloned
        } else {
            src
        };
        let source = src.borrow();
        if sx < 0 || sy < 0 || sx + w > source.w || sy + h > source.h {
            return Err(format!(
                "invalid region {region:?} in {}x{}",
                source.w, source.h
            ));
        }
        let (dw, dh) = if transform >= 4 { (h, w) } else { (w, h) };
        let (x, y) = anchor_position(x + self.tx, y + self.ty, dw, dh, anchor);
        let visible = self.clip.intersect(Rect { x, y, w: dw, h: dh });
        let mut dest = self.target.borrow_mut();
        for py in visible.y..visible.y + visible.h {
            for px in visible.x..visible.x + visible.w {
                let (u, v) = (px - x, py - y);
                let (u, v) = match transform {
                    0 => (u, v),
                    1 => (u, h - 1 - v),
                    2 => (w - 1 - u, v),
                    3 => (w - 1 - u, h - 1 - v),
                    4 => (v, u),
                    5 => (v, h - 1 - u),
                    6 => (w - 1 - v, u),
                    7 => (w - 1 - v, h - 1 - u),
                    _ => return Err("invalid MIDP transform".into()),
                };
                self.pixel(
                    &mut dest,
                    px,
                    py,
                    source.pixels[((sy + v) * source.w + sx + u) as usize],
                );
            }
        }
        Ok(())
    }
    pub fn triangle(&mut self, v: &[i32]) {
        let pts = [
            (v[0] + self.tx, v[1] + self.ty),
            (v[2] + self.tx, v[3] + self.ty),
            (v[4] + self.tx, v[5] + self.ty),
        ];
        let edge = |a: (i32, i32), b: (i32, i32), x: i32, y: i32| {
            (x - a.0) as i64 * (b.1 - a.1) as i64 - (y - a.1) as i64 * (b.0 - a.0) as i64
        };
        let x0 = pts.iter().map(|p| p.0).min().unwrap().max(self.clip.x);
        let x1 = pts
            .iter()
            .map(|p| p.0)
            .max()
            .unwrap()
            .min(self.clip.x + self.clip.w - 1);
        let y0 = pts.iter().map(|p| p.1).min().unwrap().max(self.clip.y);
        let y1 = pts
            .iter()
            .map(|p| p.1)
            .max()
            .unwrap()
            .min(self.clip.y + self.clip.h - 1);
        let mut b = self.target.borrow_mut();
        for y in y0..=y1 {
            for x in x0..=x1 {
                let es = [
                    edge(pts[0], pts[1], x, y),
                    edge(pts[1], pts[2], x, y),
                    edge(pts[2], pts[0], x, y),
                ];
                if es.iter().all(|e| *e >= 0) || es.iter().all(|e| *e <= 0) {
                    self.pixel(&mut b, x, y, self.color);
                }
            }
        }
    }
    pub fn round_rect(&mut self, x: i32, y: i32, w: i32, h: i32, aw: i32, ah: i32) {
        let rx = (aw / 2).min(w / 2).max(0);
        let ry = (ah / 2).min(h / 2).max(0);
        if rx == 0 || ry == 0 {
            self.fill_rect(x, y, w, h);
            return;
        }
        let mut b = self.target.borrow_mut();
        for yy in 0..h {
            for xx in 0..w {
                let dx = if xx < rx {
                    rx - xx
                } else if xx >= w - rx {
                    xx - (w - rx - 1)
                } else {
                    0
                };
                let dy = if yy < ry {
                    ry - yy
                } else if yy >= h - ry {
                    yy - (h - ry - 1)
                } else {
                    0
                };
                if (dx * dx * ry * ry + dy * dy * rx * rx) as i64 <= (rx * rx * ry * ry) as i64 {
                    self.pixel(&mut b, x + xx + self.tx, y + yy + self.ty, self.color);
                }
            }
        }
    }
}

pub fn anchor_position(mut x: i32, mut y: i32, w: i32, h: i32, anchor: i32) -> (i32, i32) {
    if anchor & 8 != 0 {
        x -= w;
    } else if anchor & 1 != 0 {
        x -= w / 2;
    }
    if anchor & 32 != 0 {
        y -= h;
    } else if anchor & 2 != 0 {
        y -= h / 2;
    }
    (x, y)
}

#[derive(serde::Deserialize)]
struct AtlasMetrics {
    point_size: u8,
    style: i32,
    height: i32,
    baseline: i32,
    widths: Vec<i32>,
    image: String,
}
struct FontAtlas {
    metrics: AtlasMetrics,
    bitmap: Surface,
}
pub struct Fonts {
    regular: fontdue::Font,
    bold: fontdue::Font,
    cache: HashMap<(char, u8, bool), (fontdue::Metrics, Vec<u8>)>,
    atlases: HashMap<(u8, bool), FontAtlas>,
}
impl Fonts {
    pub fn new(assets: &std::path::Path) -> Result<Self, String> {
        let load = |p: &str| -> Result<fontdue::Font, String> {
            fontdue::Font::from_bytes(
                std::fs::read(p).map_err(|e| e.to_string())?,
                fontdue::FontSettings::default(),
            )
            .map_err(str::to_string)
        };
        let entries: Vec<AtlasMetrics> = serde_json::from_slice(
            &std::fs::read(assets.join("fonts/metrics.json")).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let mut atlases = HashMap::new();
        for metrics in entries {
            if metrics.widths.len() != 256 {
                return Err("Invalid font metrics".into());
            }
            let bitmap = Rc::new(RefCell::new(Bitmap::load(
                &assets.join("fonts").join(&metrics.image),
            )?));
            atlases.insert(
                (metrics.point_size, metrics.style & 1 != 0),
                FontAtlas { metrics, bitmap },
            );
        }
        Ok(Self {
            regular: load("/System/Library/Fonts/Supplemental/Arial.ttf")?,
            bold: load("/System/Library/Fonts/Supplemental/Arial Bold.ttf")?,
            cache: HashMap::new(),
            atlases,
        })
    }
    pub fn size(size: i32) -> u8 {
        match size {
            8 => 13,
            16 => 17,
            _ => 15,
        }
    }
    pub fn height(&self, size: u8) -> i32 {
        if let Some(a) = self.atlases.get(&(size, false)) {
            a.metrics.height
        } else {
            self.regular
                .horizontal_line_metrics(size as f32)
                .unwrap()
                .new_line_size
                .ceil() as i32
        }
    }
    pub fn width(&self, text: &str, size: u8, bold: bool) -> i32 {
        if let Some(a) = self.atlases.get(&(size, bold)) {
            return text
                .chars()
                .map(|c| a.metrics.widths[if (c as usize) < 256 { c as usize } else { 63 }])
                .sum();
        }
        let f = if bold { &self.bold } else { &self.regular };
        text.chars()
            .map(|c| f.metrics(c, size as f32).advance_width.round() as i32)
            .sum()
    }
    pub fn draw(
        &mut self,
        g: &mut Graphics,
        text: &str,
        x: i32,
        y: i32,
        anchor: i32,
        size: u8,
        bold: bool,
    ) {
        let width = self.width(text, size, bold);
        let height = self.height(size);
        if let Some(a) = self.atlases.get(&(size, bold)) {
            let (mut x, mut y) = anchor_position(x, y, width, height, anchor);
            if anchor & 64 != 0 {
                y -= a.metrics.baseline;
            }
            let source = a.bitmap.borrow();
            let mut dest = g.target.borrow_mut();
            for c in text.chars() {
                let c = if (c as usize) < 256 { c as usize } else { 63 };
                let w = a.metrics.widths[c];
                for yy in 0..height {
                    for xx in 0..w.min(32) {
                        let p = source.pixels
                            [((c / 16 * 32 + yy as usize) * 512) + (c % 16 * 32) + xx as usize];
                        let alpha = p & 0xff000000;
                        if alpha != 0 {
                            g.pixel(
                                &mut dest,
                                x + xx + g.tx,
                                y + yy + g.ty,
                                (g.color & 0xffffff) | alpha,
                            );
                        }
                    }
                }
                x += w;
            }
            return;
        }
        let (mut x, mut y) = anchor_position(x + g.tx, y + g.ty, width, height, anchor);
        let font = if bold { &self.bold } else { &self.regular };
        let ascent = font
            .horizontal_line_metrics(size as f32)
            .unwrap()
            .ascent
            .ceil() as i32;
        if anchor & 64 != 0 {
            y -= ascent;
        }
        let baseline = y + ascent;
        let mut b = g.target.borrow_mut();
        for c in text.chars() {
            let (m, mask) = self
                .cache
                .entry((c, size, bold))
                .or_insert_with(|| font.rasterize(c, size as f32));
            for yy in 0..m.height {
                for xx in 0..m.width {
                    let a = mask[yy * m.width + xx] as u32;
                    if a > 0 {
                        g.pixel(
                            &mut b,
                            x + m.xmin + xx as i32,
                            baseline - m.ymin - m.height as i32 + yy as i32,
                            (g.color & 0x00ffffff) | (a << 24),
                        );
                    }
                }
            }
            x += m.advance_width.round() as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn translation_clip_and_alpha() {
        let s = Rc::new(RefCell::new(Bitmap::new(8, 8)));
        let mut g = Graphics::new(s.clone());
        g.tx = 2;
        g.ty = 1;
        g.set_clip(0, 0, 2, 2);
        g.color(0xff0000);
        g.fill_rect(-4, -4, 20, 20);
        assert_eq!(
            s.borrow()
                .pixels
                .iter()
                .filter(|p| **p == 0xff0000ff)
                .count(),
            4
        );
        assert_eq!((g.clip.x - g.tx, g.clip.y - g.ty), (0, 0));
    }
    #[test]
    fn all_midp_transforms() {
        let source = Rc::new(RefCell::new(Bitmap {
            w: 2,
            h: 3,
            pixels: vec![1, 2, 3, 4, 5, 6]
                .into_iter()
                .map(|x| 0xff000000 | x)
                .collect(),
        }));
        let expected = [
            vec![1, 2, 3, 4, 5, 6],
            vec![5, 6, 3, 4, 1, 2],
            vec![2, 1, 4, 3, 6, 5],
            vec![6, 5, 4, 3, 2, 1],
            vec![1, 3, 5, 2, 4, 6],
            vec![5, 3, 1, 6, 4, 2],
            vec![2, 4, 6, 1, 3, 5],
            vec![6, 4, 2, 5, 3, 1],
        ];
        for t in 0..8 {
            let (w, h) = if t >= 4 { (3, 2) } else { (2, 3) };
            let dest = Rc::new(RefCell::new(Bitmap::new(w, h)));
            let mut g = Graphics::new(dest.clone());
            g.blit(
                &source,
                Rect {
                    x: 0,
                    y: 0,
                    w: 2,
                    h: 3,
                },
                t,
                0,
                0,
                0,
            )
            .unwrap();
            assert_eq!(
                dest.borrow()
                    .pixels
                    .iter()
                    .map(|p| p & 255)
                    .collect::<Vec<_>>(),
                expected[t as usize]
            );
        }
    }
}
