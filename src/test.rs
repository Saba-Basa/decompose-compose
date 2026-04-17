use rusttype::{Font, Scale, point};
use image::{ImageBuffer, Luma};
use std::{io::Write, thread, time::Duration};

fn density(c: char, font: &Font, size: u32) -> f64 {
    let scale: Scale  = Scale::uniform(size as f32);
    let offset: rusttype::Point<f32> = point(0.0, font.v_metrics(scale).ascent);
    let mut img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::<Luma<u8>, Vec<u8>>::new(size, size);
    for glyph in font.layout(&c.to_string(), scale, offset) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let px = x as i32 + bb.min.x;
                let py = y as i32 + bb.min.y;
                if px >= 0 && py >= 0 && px < size as i32 && py < size as i32 {
                    img.put_pixel(px as u32, py as u32, Luma([(v * 255.0) as u8]));
                }
            });
        }
    }
    img.pixels().map(|p: &Luma<u8>| p[0] as f64).sum::<f64>() / (size * size) as f64 / 255.0
}

fn lookup(v: f64, ramp: &[(char, f64)]) -> char {
    let n = ramp.len();
    let v = v.clamp(0.0, 1.0);
    let k = ((v * (n - 1) as f64).round() as usize).min(n - 1);
    ramp[k].0
}

fn main() {
    let font_path   = "/System/Library/Fonts/Supplemental/Andale Mono.ttf";
    let measure_size: u32  = 128;
    let input       = "Hello Ra Ra! This is a density ramp test string 123 #@W.";

    let font_data = std::fs::read(font_path).expect("font not found");
    let font = rusttype::Font::try_from_vec(font_data).expect("font parse error");

    let mut pairs: Vec<(char, f64)> = (32u8..127u8)
        .map(|i| i as char)
        .map(|c| (c, density(c, &font, measure_size)))
        .filter(|(_, d)| *d > 0.001)
        .collect();
    pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // ── print ramp ────────────────────────────────────────────────────────────
    println!("=== RAMP ===");
    println!("{}\n", pairs.iter().map(|(c, _)| c).collect::<String>());

    // ── show each char in input string with its density ───────────────────────
    println!("=== INPUT CHAR DENSITIES ===");
    for c in input.chars() {
        if c == ' ' {
            println!("  ' '  0.0000  (space)");
            continue;
        }
        if let Some(&(_, d)) = pairs.iter().find(|(rc, _)| *rc == c) {
            let bar   = "█".repeat((d * 300.0) as usize);
            let v     = d / pairs.last().unwrap().1; // normalize to [0,1]
            let ramp_char = lookup(v, &pairs);
            println!("  '{}'  {:.4}  {}  -> ramp maps to '{}'", c, d, bar, ramp_char);
        }
    }

    // ── animate the string as a brightness wave in terminal ───────────────────
    println!("\n=== LIVE PREVIEW (Ctrl+C to stop) ===\n");
    thread::sleep(Duration::from_millis(800));

    print!("\x1b[?25l"); // hide cursor
    let mut t: f64 = 0.0;

    loop {
        t += 0.06;
        print!("\x1b[H\x1b[3B"); // home + 3 lines down (skip header)

        for row in 0..12 {
            for (i, c) in input.chars().enumerate() {
                if c == ' ' { print!(" "); continue; }

                // wave: each char gets a brightness value from a sine wave
                let phase = i as f64 * 0.4 - t + row as f64 * 0.3;
                let v     = (phase.sin() + 1.0) / 2.0; // [0,1]

                // pick ramp char for this brightness
                let rc = lookup(v, &pairs);

                // color: blue-cyan-white
                let bright = (60.0 + v * 195.0) as u8;
                let r = (bright as f64 * 0.3) as u8;
                let g = (bright as f64 * 0.8) as u8;
                let b = bright;

                print!("\x1b[38;2;{};{};{}m{}", r, g, b, rc);
            }
            println!("\x1b[0m");
        }

        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(33));
    }
}