use particle::{Particle, ParticleColor, ParticleRule};
use particle_world::ParticleWorld;
use quadtree::AABB;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Instant;

mod particle;
mod particle_world;
mod quadtree;

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

    // Initialize World
    let mut particles: Vec<Particle> = Vec::new();
    particles.append(&mut particle_world::create_particles(
        500,
        ParticleColor::Yellow,
        Color::YELLOW,
    ));
    particles.append(&mut particle_world::create_particles(
        500,
        ParticleColor::Red,
        Color::RED,
    ));
    particles.append(&mut particle_world::create_particles(
        500,
        ParticleColor::Green,
        Color::GREEN,
    ));

    let mut world = ParticleWorld::new(particles, 80.0);
    let mut test_query = AABB::new(0.0, 0.0, 80.0);
    let mut avg_time: u128 = 0;

    // Main Loop
    let mut running = true;
    while running {
        // Events
        let start = Instant::now();
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

        world.update(&rules);

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        world.draw(&mut canvas);
        world.draw_debug(&mut canvas, &test_query);

        canvas.present();

        let duration = start.elapsed();

        println!("Time elapsed in frame is: {:?}", duration);

        // Calculate average time
        if avg_time > 0 {
            avg_time = (avg_time + duration.as_millis()) / 2;
        } else {
            avg_time = duration.as_millis();
        }
    }

    println!("Average frame time is: {:?}ms", avg_time);

    Ok(())
}
