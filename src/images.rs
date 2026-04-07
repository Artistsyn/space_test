use image::{Rgba, RgbaImage};
use crate::constants::*;

// ─────────────────────────────────────────────────────────────────────────────
// Primitives
// ─────────────────────────────────────────────────────────────────────────────
pub fn solid(r: u8, g: u8, b: u8, a: u8) -> RgbaImage {
    let mut img = RgbaImage::new(1, 1);
    img.put_pixel(0, 0, Rgba([r, g, b, a]));
    img
}

pub fn draw_rect(img: &mut RgbaImage, x: i32, y: i32, w: u32, h: u32, c: [u8; 4]) {
    let max_x = ((x + w as i32) as u32).min(img.width());
    let max_y = ((y + h as i32) as u32).min(img.height());
    let sx = x.max(0) as u32;
    let sy = y.max(0) as u32;
    for py in sy..max_y {
        for px in sx..max_x {
            img.put_pixel(px, py, Rgba(c));
        }
    }
}

pub fn draw_tri_up(img: &mut RgbaImage, cx: i32, top_y: i32, half_w: i32, h: i32, c: [u8; 4]) {
    for row in 0..h {
        let frac = row as f32 / h as f32;
        let half = (half_w as f32 * frac) as i32;
        let y = top_y + row;
        if y < 0 || y >= img.height() as i32 { continue; }
        for dx in -half..=half {
            let x = cx + dx;
            if x >= 0 && x < img.width() as i32 {
                img.put_pixel(x as u32, y as u32, Rgba(c));
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Player spaceship — a simple arrow-like ship pointing UP
//   • Obvious triangular nose (top)
//   • Wider body with wings
//   • Thruster nozzle (bottom)
// ─────────────────────────────────────────────────────────────────────────────
pub fn player_ship_img() -> RgbaImage {
    let w = PLAYER_W as u32;
    let h = PLAYER_H as u32;
    let mut img = RgbaImage::new(w, h);
    let cx = w as i32 / 2;

    // Nose — narrow triangle top 40%
    let nose_h = (h as f32 * 0.40) as i32;
    draw_tri_up(&mut img, cx, 0, (w as i32 / 2) - 6, nose_h, [180, 200, 220, 255]);

    // Cockpit window
    let cockpit_y = (h as f32 * 0.18) as i32;
    draw_rect(&mut img, cx - 4, cockpit_y, 9, 8, [80, 180, 255, 255]);

    // Body — central rectangle
    let body_top = nose_h;
    let body_h = (h as f32 * 0.35) as u32;
    let body_half = (w as i32 / 2) - 4;
    draw_rect(&mut img, cx - body_half, body_top, (body_half * 2) as u32, body_h, [140, 155, 170, 255]);

    // Wing fins — wider at body level
    let wing_y = body_top + (body_h as i32 / 3);
    let wing_h = (body_h as f32 * 0.55) as u32;
    draw_rect(&mut img, 0, wing_y, 10, wing_h, [100, 120, 140, 255]);
    draw_rect(&mut img, (w - 10) as i32, wing_y, 10, wing_h, [100, 120, 140, 255]);

    // Engine nozzle — bottom
    let nozzle_top = body_top + body_h as i32;
    let nozzle_h = h as i32 - nozzle_top;
    draw_rect(&mut img, cx - 10, nozzle_top, 20, nozzle_h as u32, [80, 80, 100, 255]);
    // Engine glow
    draw_rect(&mut img, cx - 6, nozzle_top + 4, 12, (nozzle_h - 4).max(1) as u32, [255, 140, 40, 200]);

    // Accent stripe
    let stripe_y = body_top + 3;
    draw_rect(&mut img, cx - body_half + 2, stripe_y, ((body_half - 2) * 2) as u32, 3, [40, 180, 255, 180]);

    img
}

// ─────────────────────────────────────────────────────────────────────────────
// Enemy ship — red/dark, diamond-ish shape pointing up
// ─────────────────────────────────────────────────────────────────────────────
pub fn enemy_ship_img() -> RgbaImage {
    let w = ENEMY_W as u32;
    let h = ENEMY_H as u32;
    let mut img = RgbaImage::new(w, h);
    let cx = w as i32 / 2;

    // Nose
    let nose_h = (h as f32 * 0.35) as i32;
    draw_tri_up(&mut img, cx, 0, (w as i32 / 2) - 2, nose_h, [200, 50, 50, 255]);

    // Body
    let body_top = nose_h;
    let body_h = (h as f32 * 0.40) as u32;
    let body_half = (w as i32 / 2) - 2;
    draw_rect(&mut img, cx - body_half, body_top, (body_half * 2) as u32, body_h, [160, 40, 40, 255]);

    // Wings
    let wing_y = body_top;
    draw_rect(&mut img, 0, wing_y, 8, body_h, [120, 30, 30, 255]);
    draw_rect(&mut img, (w - 8) as i32, wing_y, 8, body_h, [120, 30, 30, 255]);

    // Cockpit
    draw_rect(&mut img, cx - 3, (h as f32 * 0.20) as i32, 7, 6, [255, 80, 80, 255]);

    // Engine
    let eng_top = body_top + body_h as i32;
    let eng_h = h as i32 - eng_top;
    draw_rect(&mut img, cx - 8, eng_top, 16, eng_h.max(1) as u32, [100, 30, 30, 255]);
    draw_rect(&mut img, cx - 4, eng_top + 2, 8, (eng_h - 2).max(1) as u32, [255, 100, 20, 180]);

    img
}

// ─────────────────────────────────────────────────────────────────────────────
// Laser bolt image (small bright rectangle)
// ─────────────────────────────────────────────────────────────────────────────
pub fn laser_img(r: u8, g: u8, b: u8) -> RgbaImage {
    let w = LASER_W as u32;
    let h = LASER_H as u32;
    let mut img = RgbaImage::new(w, h);
    for py in 0..h {
        for px in 0..w {
            // Glow at center
            let cx = w as f32 / 2.0;
            let dx = (px as f32 - cx).abs() / cx;
            let brightness = (1.0 - dx * 0.5).clamp(0.0, 1.0);
            let rr = (r as f32 * brightness + 255.0 * (1.0 - brightness) * 0.3) as u8;
            let gg = (g as f32 * brightness + 255.0 * (1.0 - brightness) * 0.3) as u8;
            let bb = (b as f32 * brightness + 255.0 * (1.0 - brightness) * 0.3) as u8;
            img.put_pixel(px, py, Rgba([rr, gg, bb, 240]));
        }
    }
    img
}

// ─────────────────────────────────────────────────────────────────────────────
// HUD bar image
// ─────────────────────────────────────────────────────────────────────────────
pub fn bar_img(w: u32, h: u32, fill: f32, r: u8, g: u8, b: u8) -> RgbaImage {
    let mut img = RgbaImage::new(w, h);
    let filled = (w as f32 * fill.clamp(0.0, 1.0)) as u32;
    for py in 0..h {
        for px in 0..w {
            let c = if px == 0 || px == w - 1 || py == 0 || py == h - 1 {
                Rgba([200, 200, 200, 200])
            } else if px < filled {
                Rgba([r, g, b, 220])
            } else {
                Rgba([20, 20, 30, 160])
            };
            img.put_pixel(px, py, c);
        }
    }
    img
}

// ─────────────────────────────────────────────────────────────────────────────
// Pause overlay — semi-transparent dark
// ─────────────────────────────────────────────────────────────────────────────
pub fn pause_overlay_img() -> RgbaImage {
    RgbaImage::from_pixel(4, 4, Rgba([10, 10, 20, 180]))
}

// ─────────────────────────────────────────────────────────────────────────────
// Minimap background
// ─────────────────────────────────────────────────────────────────────────────
pub fn minimap_bg() -> RgbaImage {
    let w = MINIMAP_W as u32;
    let h = MINIMAP_H as u32;
    let mut img = RgbaImage::new(w, h);
    for py in 0..h {
        for px in 0..w {
            let border = px == 0 || px == w - 1 || py == 0 || py == h - 1;
            if border {
                img.put_pixel(px, py, Rgba([100, 120, 140, 200]));
            } else {
                img.put_pixel(px, py, Rgba([5, 5, 15, 160]));
            }
        }
    }
    img
}

// ─────────────────────────────────────────────────────────────────────────────
// Controls overlay image (for pause menu)
// ─────────────────────────────────────────────────────────────────────────────
pub fn controls_overlay_img() -> RgbaImage {
    // Just a dark box — text will be rendered via Quartz text system
    let w = 1200u32;
    let h = 900u32;
    let mut img = RgbaImage::new(w, h);
    for py in 0..h {
        for px in 0..w {
            let border = px < 3 || px >= w - 3 || py < 3 || py >= h - 3;
            if border {
                img.put_pixel(px, py, Rgba([80, 160, 255, 220]));
            } else {
                img.put_pixel(px, py, Rgba([15, 15, 30, 230]));
            }
        }
    }
    img
}

// ─────────────────────────────────────────────────────────────────────────────
// Space debris — jagged irregular rocks
// ─────────────────────────────────────────────────────────────────────────────
/// Generates an irregular rock sprite of the given size with a seed for variety.
/// `kind` 0 = rocky grey, 1 = metallic blue, 2 = icy white
pub fn debris_img(size: u32, seed: u64, kind: u8) -> RgbaImage {
    let (base_r, base_g, base_b) = match kind {
        1 => (100u8, 120u8, 160u8), // metallic
        2 => (180u8, 200u8, 220u8), // icy
        _ => (120u8, 110u8, 100u8), // rocky
    };
    let half = size as f32 / 2.0;
    let mut img = RgbaImage::new(size, size);
    let mut s = seed;
    // Generate 8 radii for an irregular polygon
    let mut radii = [0.0f32; 8];
    for r in radii.iter_mut() {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let frac = ((s >> 32) as u32) as f32 / u32::MAX as f32;
        *r = half * (0.5 + frac * 0.5);
    }
    for py in 0..size {
        for px in 0..size {
            let dx = px as f32 - half;
            let dy = py as f32 - half;
            let dist = (dx * dx + dy * dy).sqrt();
            let angle = dy.atan2(dx);
            // Interpolate between radii based on angle
            let sector = ((angle + std::f32::consts::PI) / (2.0 * std::f32::consts::PI) * 8.0) % 8.0;
            let i0 = sector as usize % 8;
            let i1 = (i0 + 1) % 8;
            let t = sector - sector.floor();
            let r_at = radii[i0] * (1.0 - t) + radii[i1] * t;
            if dist < r_at {
                // Shade variation
                let shade = (0.8 + 0.4 * (dist / r_at)) as f32;
                let cr = (base_r as f32 * shade).min(255.0) as u8;
                let cg = (base_g as f32 * shade).min(255.0) as u8;
                let cb = (base_b as f32 * shade).min(255.0) as u8;
                img.put_pixel(px, py, Rgba([cr, cg, cb, 255]));
            }
        }
    }
    img
}
