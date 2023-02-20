use particle::{Particle, ParticleColor, ParticleRule};
use quadtree::{QuadTree, AABB, XY};
use rand::prelude::*;
use sdl2::event::Event;
use sdl2::pixels::Color;

mod particle;
mod quadtree;
mod quadtree_visualizer;

fn main() -> Result<(), String> {
    let width = 800;
    let height = 800;

    // System
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rust!", width, height)
        .position_centered()
        .build()
        .expect("Failed to build window!");
    let mut canvas = window.into_canvas().build().unwrap();

    // Events
    let mut event_queue = sdl_context.event_pump().unwrap();

    // Initialize Rules
    let rules = [
        ParticleRule {
            from: ParticleColor::Green,
            to: ParticleColor::Green,
            g: -0.32,
        },
        ParticleRule {
            from: ParticleColor::Green,
            to: ParticleColor::Red,
            g: -0.17,
        },
        ParticleRule {
            from: ParticleColor::Green,
            to: ParticleColor::Yellow,
            g: 0.34,
        },
        ParticleRule {
            from: ParticleColor::Red,
            to: ParticleColor::Red,
            g: -0.10,
        },
        ParticleRule {
            from: ParticleColor::Red,
            to: ParticleColor::Green,
            g: -0.34,
        },
        ParticleRule {
            from: ParticleColor::Yellow,
            to: ParticleColor::Yellow,
            g: 0.15,
        },
        ParticleRule {
            from: ParticleColor::Yellow,
            to: ParticleColor::Green,
            g: -0.20,
        },
    ];

    // Initialize Particles
    let mut particles: Vec<Particle> = Vec::new();
    particles.append(&mut create(500, ParticleColor::Yellow, Color::YELLOW));

    particles.append(&mut create(500, ParticleColor::Red, Color::RED));

    particles.append(&mut create(500, ParticleColor::Green, Color::GREEN));

    let mut test_query = AABB::new(0.0, 0.0, 50.0);

    // Main Loop
    let mut running = true;
    while running {
        // Events
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,

                Event::MouseButtonDown { x, y, .. } => {
                    test_query.center.x = x as f32;
                    test_query.center.y = y as f32;
                }
                _ => {}
            }
        }

        // Update
        let world: &Vec<Particle> = &particles.clone();
        let mut tree: QuadTree<Particle> = QuadTree::new(AABB::new(400.0, 400.0, 400.0));
        for wp in world {
            tree.insert(wp.pos, *wp);
        }

        for p in &mut particles {
            let range = AABB::new(p.pos.x, p.pos.y, 80.0);
            for r in &rules {
                let in_range = tree.query_range(&range);
                r.run(p, &in_range);
            }
        }

        // Draw
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        quadtree_visualizer::draw_quadtree(&mut canvas, &tree);
        for p in &particles {
            p.draw(&mut canvas);
        }
        quadtree_visualizer::draw_query(&mut canvas, &tree, &test_query);

        canvas.present();
    }

    Ok(())
}

fn create(number: i32, color: ParticleColor, draw_color: Color) -> Vec<Particle> {
    let mut group: Vec<Particle> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..number {
        group.push(Particle {
            pos: XY {
                x: rng.gen_range(0..800) as f32,
                y: rng.gen_range(0..800) as f32,
            },
            vx: 0.0,
            vy: 0.0,
            draw_color,
            color,
        })
    }

    return group;
}
