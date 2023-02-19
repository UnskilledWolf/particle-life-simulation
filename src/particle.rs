use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

#[derive(Clone, Copy, PartialEq)]
pub enum ParticleColor {
    Yellow,
    Red,
    Green,
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub x: i16,
    pub y: i16,
    pub vx: f64,
    pub vy: f64,
    pub draw_color: Color,
    pub color: ParticleColor,
}

impl Particle {
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        DrawRenderer::filled_circle(canvas, self.x, self.y, 2, self.draw_color).unwrap();
    }
}

pub struct ParticleRule {
    pub from: ParticleColor,
    pub to: ParticleColor,
    pub g: f64,
}

impl ParticleRule {
    pub fn run(&self, p1: &mut Particle, particles: &Vec<Particle>) {
        if p1.color != self.from {
            return;
        }

        let mut fx: f64 = 0.0;
        let mut fy: f64 = 0.0;

        for p2 in particles {
            let dx: f64 = (p1.x - p2.x) as f64;
            let dy: f64 = (p1.y - p2.y) as f64;

            let d: f64 = f64::sqrt((dx * dx + dy * dy) as f64) + 1.0;
            if d > 0.0 && d < 175.0 {
                let f = self.g * 1.0 / d;
                fx += f * dx;
                fy += f * dy;
            }
        }

        p1.vx = (p1.vx + fx) * 0.5;
        p1.vy = (p1.vy + fy) * 0.5;

        if p1.x <= 20 || p1.x >= 800 - 20 {
            p1.vx *= -1.0;
        }
        if p1.y <= 20 || p1.y >= 800 - 20 {
            p1.vx *= -1.0;
        }

        p1.x += p1.vx as i16;
        p1.y += p1.vy as i16;
    }
}
