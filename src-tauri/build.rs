
use std::fs;
use tiny_skia::Transform;

fn main() {
    render_icons_from_svg();
    tauri_build::build();
}

fn render_icons_from_svg() {
    let svg_path = "icons/Mistral_AI_logo.svg";
    let sizes: [u32; 4] = [32, 64, 128, 256];

    match fs::read(svg_path) {
        Ok(svg_data) => {
            let mut opt = usvg::Options::default();
            opt.resources_dir = Some("icons".into());

            match usvg::Tree::from_data(&svg_data, &opt) {
                Ok(tree) => {
                    for size in sizes {
                        let out_path = format!("icons/icon-{}.png", size);
                        if !std::path::Path::new(&out_path).exists() {
                            if let Some(mut pixmap) = tiny_skia::Pixmap::new(size, size) {
                                let viewbox = tree.size();
                                let vb_width = viewbox.width();
                                let vb_height = viewbox.height();
                                
                                // Scale to fit within square, maintaining aspect ratio
                                let scale = (size as f32) / vb_width.max(vb_height);
                                let scaled_w = vb_width * scale;
                                let scaled_h = vb_height * scale;
                                
                                // Center within the square
                                let offset_x = ((size as f32) - scaled_w) / 2.0;
                                let offset_y = ((size as f32) - scaled_h) / 2.0;
                                
                                let transform = Transform::from_translate(offset_x, offset_y)
                                    .post_scale(scale, scale);
                                
                                let mut pm = pixmap.as_mut();
                                resvg::render(&tree, transform, &mut pm);
                                let _ = pixmap.save_png(&out_path);
                                println!("cargo:warning=Rendered {} (centered)", out_path);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse SVG: {:?}", e);
                    fallback_placeholder_icons(sizes);
                }
            }
        }
        Err(e) => {
            eprintln!("SVG file not found at {}: {:?}", svg_path, e);
            fallback_placeholder_icons(sizes);
        }
    }
}

fn fallback_placeholder_icons(sizes: [u32; 4]) {
    for size in sizes {
        let out_path = format!("icons/icon-{}.png", size);
        if !std::path::Path::new(&out_path).exists() {
            if let Some(mut pixmap) = tiny_skia::Pixmap::new(size, size) {
                let bg = tiny_skia::Color::from_rgba8(38, 79, 120, 255);
                pixmap.fill(bg);
                let _ = pixmap.save_png(&out_path);
                println!("cargo:warning=Generated placeholder {}", out_path);
            }
        }
    }
}
