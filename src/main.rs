use sdl2::event::Event;
use sdl2::pixels::Color;

mod particle;
use particle::{Particle, ParticleColor, ParticleRule};
use rand::prelude::*;

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

    // Main Loop
    let mut running = true;
    while running {
        // Events
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,

                // Event::MouseButtonDown { x, y, .. } => {}
                _ => {}
            }
        }

        // Update
        let world: &Vec<Particle> = &particles.clone();
        for p in &mut particles {
            for r in &rules {
                r.run(p, world)
            }
        }

        // Draw
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for p in &particles {
            p.draw(&mut canvas);
        }

        canvas.present();
    }

    Ok(())
}

fn create(number: i32, color: ParticleColor, draw_color: Color) -> Vec<Particle> {
    let mut group: Vec<Particle> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..number {
        group.push(Particle {
            x: rng.gen_range(0..800),
            y: rng.gen_range(0..800),
            vx: 0.0,
            vy: 0.0,
            draw_color: draw_color,
            color: color,
        })
    }

    return group;
}
