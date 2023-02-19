use crate::{particle::Particle, quadtree::QuadTree};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

const BOX_COLOR: Color = Color::RGB(64, 64, 64);

pub fn draw_quadtree(canvas: &mut Canvas<Window>, qt: &QuadTree<Particle>) {
    draw_section(canvas, qt);

    //TODO find a good way to put this in functions
    match &qt.north_west {
        Some(qt2) => draw_quadtree(canvas, qt2),
        None => {}
    }

    match &qt.north_east {
        Some(qt2) => draw_quadtree(canvas, qt2),
        None => {}
    }

    match &qt.south_west {
        Some(qt2) => draw_quadtree(canvas, qt2),
        None => {}
    }

    match &qt.south_east {
        Some(qt2) => draw_quadtree(canvas, qt2),
        None => {}
    }
}

fn draw_section(canvas: &mut Canvas<Window>, qt: &QuadTree<Particle>) {
    let r: Rect = Rect::new(
        (qt.boundary.center.x - qt.boundary.half_dimension) as i32,
        (qt.boundary.center.y - qt.boundary.half_dimension) as i32,
        qt.boundary.half_dimension as u32 * 2,
        qt.boundary.half_dimension as u32 * 2,
    );
    canvas.set_draw_color(BOX_COLOR);
    canvas.draw_rect(r).unwrap();
}
