use crate::{
    particle::{self, Particle},
    quadtree::{QuadTree, AABB},
};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

const BOX_COLOR: Color = Color::RGB(64, 64, 64);
const QUERY_COLOR: Color = Color::RGB(134, 189, 149);

pub fn draw_quadtree(canvas: &mut Canvas<Window>, qt: &QuadTree) {
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

pub fn draw_query(
    canvas: &mut Canvas<Window>,
    qt: &QuadTree,
    range: &AABB,
    points: &Vec<Particle>,
) {
    let main_rect = aabb_to_rect(range);

    canvas.set_draw_color(QUERY_COLOR);
    canvas.draw_rect(main_rect).unwrap();

    let pts = qt.query_range(range);
    for p in pts {
        canvas
            .draw_rect(Rect::new(
                (points[p].pos.x - 5.0) as i32,
                (points[p].pos.y - 5.0) as i32,
                10,
                10,
            ))
            .unwrap();
    }

    let mut right = main_rect.center();
    right.x += particle::MAX_DISTANCE as i32;
    canvas.draw_line(main_rect.center(), right).unwrap();
}

fn draw_section(canvas: &mut Canvas<Window>, qt: &QuadTree) {
    canvas.set_draw_color(BOX_COLOR);
    canvas.draw_rect(aabb_to_rect(&qt.boundary)).unwrap();
}

fn aabb_to_rect(range: &AABB) -> Rect {
    Rect::new(
        (range.center.x - range.half_dimension) as i32,
        (range.center.y - range.half_dimension) as i32,
        range.half_dimension as u32 * 2,
        range.half_dimension as u32 * 2,
    )
}
