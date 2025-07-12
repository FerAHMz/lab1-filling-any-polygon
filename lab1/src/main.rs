mod framebuffer;
mod line;

use framebuffer::{Framebuffer, fill_polygon};
use line::line;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let mut fb = Framebuffer::new(width, height, Color::RAYWHITE);
    fb.clear();

    // ------------------------
    // POLIGON-1 START
    // ------------------------
    let poly = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];
    let points: Vec<Vector2> = poly.iter()
        .map(|&(x, y)| Vector2::new(x as f32, (height - y) as f32))
        .collect();

    fb.set_current_color(Color::YELLOW);
    fill_polygon(&mut fb, &points, None);

    fb.set_current_color(Color::WHITE);
    for i in 0..points.len() {
        line(&mut fb, points[i], points[(i + 1) % points.len()]);
    }
    // ------------------------
    // POLIGON-1 END
    // ------------------------

    // ------------------------
    // POLIGON-2 START
    // ------------------------
    let poly = vec![
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];
    let points: Vec<Vector2> = poly.iter()
        .map(|&(x, y)| Vector2::new(x as f32, (height - y) as f32))
        .collect();

    fb.set_current_color(Color::BLUE);
    fill_polygon(&mut fb, &points, None);

    fb.set_current_color(Color::WHITE);
    for i in 0..points.len() {
        line(&mut fb, points[i], points[(i + 1) % points.len()]);
    }
    // ------------------------
    // POLIGON-2 END
    // ------------------------

    // ------------------------
    // POLIGON-3 START
    // ------------------------
    let poly = vec![
        (377, 249), (411, 197), (436, 249),
    ];
    let points: Vec<Vector2> = poly.iter()
        .map(|&(x, y)| Vector2::new(x as f32, (height - y) as f32))
        .collect();

    fb.set_current_color(Color::RED);
    fill_polygon(&mut fb, &points, None);

    fb.set_current_color(Color::WHITE);
    for i in 0..points.len() {
        line(&mut fb, points[i], points[(i + 1) % points.len()]);
    }
    // ------------------------
    // POLIGON-3 END
    // ------------------------

    // ------------------------
    // POLIGON-4 START
    // ------------------------
    // ------------------------
    // POLIGON-4 END

    fb.render_to_file("out.bmp");
}
