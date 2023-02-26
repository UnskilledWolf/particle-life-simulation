use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

use crate::quadtree::XY;

#[derive(Clone, Copy, PartialEq)]
pub enum ParticleColor {
    Yellow,
    Red,
    Green,
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: XY,
    pub vx: f32,
    pub vy: f32,
    pub draw_color: Color,
    pub color: ParticleColor,
}

impl Particle {
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        DrawRenderer::filled_circle(
            canvas,
            self.pos.x as i16,
            self.pos.y as i16,
            2,
            self.draw_color,
        )
        .unwrap();
    }
}

pub struct ParticleRule {
    pub from: ParticleColor,
    pub to: ParticleColor,
    pub g: f32,
}

impl ParticleRule {
    pub fn get_distances(
        p1: &Particle,
        in_range: &Vec<usize>,
        particles: &Vec<Particle>,
    ) -> Vec<f32> {
        let mut output: Vec<f32> = Vec::new();

        for p2_index in in_range {
            let p2 = particles[*p2_index];
            let dx: f32 = (p1.pos.x - p2.pos.x) as f32;
            let dy: f32 = (p1.pos.y - p2.pos.y) as f32;

            let d: f32 = (dx.powi(2) + dy.powi(2)).sqrt();
            output.push(d);
        }

        return output;
    }

    pub fn run(
        &self,
        p1: &mut Particle,
        in_range: &Vec<usize>,
        particles: &Vec<Particle>,
        distances: &Vec<f32>,
    ) {
        if p1.color != self.from {
            return;
        }

        let mut fx: f32 = 0.0;
        let mut fy: f32 = 0.0;

        for (i, p2_index) in in_range.iter().enumerate() {
            let p2 = particles[*p2_index];
            let dx: f32 = (p1.pos.x - p2.pos.x) as f32;
            let dy: f32 = (p1.pos.y - p2.pos.y) as f32;

            let d: f32 = distances[i];
            if d > 0.0 && d < 175.0 {
                let f = self.g * 1.0 / d;
                fx += f * dx;
                fy += f * dy;
            }
        }

        p1.vx = (p1.vx + fx) * 0.5;
        p1.vy = (p1.vy + fy) * 0.5;

        if p1.pos.x <= 20.0 || p1.pos.x >= 780.0 {
            p1.vx *= -1.0;
        }
        if p1.pos.y <= 20.0 || p1.pos.y >= 780.0 {
            p1.vx *= -1.0;
        }

        p1.pos.x += p1.vx;
        p1.pos.y += p1.vy;
    }
}
