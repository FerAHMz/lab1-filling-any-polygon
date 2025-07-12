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
    let poly4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];
    let poly5 = vec![
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];
    let outer: Vec<Vector2> = poly4.iter()
        .map(|&(x, y)| Vector2::new(x as f32, (height - y) as f32))
        .collect();
    let hole: Vec<Vector2> = poly5.iter()
        .map(|&(x, y)| Vector2::new(x as f32, (height - y) as f32))
        .collect();

    fb.set_current_color(Color::GREEN);
    fill_polygon(&mut fb, &outer, Some(&hole));

    fb.set_current_color(Color::WHITE);
    for i in 0..outer.len() {
        line(&mut fb, outer[i], outer[(i + 1) % outer.len()]);
    }
    for i in 0..hole.len() {
        line(&mut fb, hole[i], hole[(i + 1) % hole.len()]);
    }
    // ------------------------
    // POLIGON-4 END
    // ------------------------
    
    fb.render_to_file("out.bmp");
}
