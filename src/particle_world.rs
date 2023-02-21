use rand::prelude::*;
use sdl2::{pixels::Color, render::Canvas, video::Window};

use crate::{
    particle::{Particle, ParticleColor, ParticleRule},
    quadtree::{QuadTree, AABB, XY},
    quadtree_visualizer,
};

pub struct ParticleWorld {
    particles: Vec<Particle>,
    range: AABB,
    tree: QuadTree<Particle>,
}

impl ParticleWorld {
    pub fn new(particles: Vec<Particle>, range_size: f32) -> ParticleWorld {
        ParticleWorld {
            particles,
            range: AABB {
                center: XY { x: 0.0, y: 0.0 },
                half_dimension: range_size,
            },
            tree: QuadTree::new(AABB::new(400.0, 400.0, 400.0)),
        }
    }

    pub fn update(&mut self, rules: &[ParticleRule]) {
        // // Update
        // let world: &Vec<Particle> = &particles.clone();
        // let mut tree: QuadTree<Particle> = QuadTree::new(AABB::new(400.0, 400.0, 400.0));
        // for wp in world {
        //     tree.insert(wp.pos, *wp);
        // }

        // Update Tree
        self.tree = QuadTree::new(AABB::new(400.0, 400.0, 400.0));
        for p in &self.particles {
            let pc = p.clone();
            self.tree.insert(pc.pos, pc);
        }

        // Update particles

        for p in &mut self.particles {
            self.range.center.x = p.pos.x;
            self.range.center.y = p.pos.y;
            for r in rules {
                let in_range = self.tree.query_range(&self.range);
                r.run(p, &in_range);
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for p in &self.particles {
            p.draw(canvas);
        }
    }

    pub fn draw_debug(&self, canvas: &mut Canvas<Window>, test_query: &AABB) {
        quadtree_visualizer::draw_quadtree(canvas, &self.tree);
        quadtree_visualizer::draw_query(canvas, &self.tree, test_query);
    }
}

pub fn create_particles(number: i32, color: ParticleColor, draw_color: Color) -> Vec<Particle> {
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
