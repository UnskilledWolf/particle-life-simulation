const GRID_SIZE: usize = 800 / 10;
const INIT: Vec<Particle> = Vec::new();

use crate::particle::Particle;

pub struct ParticleGrid {
    points: [[usize; GRID_SIZE]; GRID_SIZE],
}

impl ParticleGrid {
    pub fn new() -> ParticleGrid {
        ParticleGrid {
            // ! Particle 0 will never get updated
            points: [[0; GRID_SIZE]; GRID_SIZE],
        }
    }

    pub fn insert(&mut self, particle: &Particle, particle_id: usize) {
        let x: usize = (particle.pos.x as usize) / GRID_SIZE;
        let y: usize = (particle.pos.y as usize) / GRID_SIZE;

        self.points[y][x] = particle_id;
    }
}
