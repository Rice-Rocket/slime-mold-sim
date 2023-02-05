use macroquad::prelude::{Image, Color};
use rand::distributions::{Distribution, Uniform};
use std::{f32::consts::PI};

#[path = "agent.rs"] mod agent;
use self::agent::Agent;


pub struct SimulationParams {
    pub move_speed: f32,
    pub evaporation_speed: f32,
    pub diffuse_speed: f32,
    pub sense_angle_difference: f32,
    pub sense_distance: f32,
    pub sense_size: i32,
    pub turn_speed: f32,
    pub pheromone_color: Color,
    pub background_color: Color
}

impl Default for SimulationParams {
    fn default() -> Self {
        Self {
            move_speed: 50.0,
            evaporation_speed: 0.25,
            diffuse_speed: 8.0,
            sense_angle_difference: 1.0,
            sense_distance: 10.0,
            sense_size: 3,
            turn_speed: 30.0,
            pheromone_color: Color::new(1.0, 1.0, 1.0, 1.0),
            background_color: Color::new(0.0, 0.0, 0.0, 1.0)
        }
    }
}



pub struct Field {
    width: u16,
    height: u16,
    field: Vec<Vec<f64>>,
    agents: Vec<Agent>,
    can_run: bool,
    started_running: bool,
    settings: SimulationParams
}


impl Field {
    pub fn new(width: u16, height: u16, params: SimulationParams) -> Self {
        let mut field_vec = Vec::new();
        for _ in 0..width {
            let mut col = Vec::new();
            for _ in 0..height {
                col.push(0.0);
            }
            field_vec.push(col);
        }
        return Self {
            width: width,
            height: height,
            field: field_vec,
            agents: Vec::new(),
            can_run: false,
            started_running: false,
            settings: params,
        };
    }
    pub fn reset(&mut self) {
        self.field.clear();
        for _ in 0..self.width {
            let mut col = Vec::new();
            for _ in 0..self.height {
                col.push(0.0);
            }
            self.field.push(col);
        }
        self.agents.clear();
        self.can_run = false;
        self.started_running = false;
    }
    pub fn add_point(&mut self, n_agents: usize) {
        self.agents.clear();
        self.can_run = true;
        let rand_range: Uniform<f32> = Uniform::from(0.0..(2.0 * PI));
        let mut rng = rand::thread_rng();
        for _ in 0..n_agents {
            self.agents.push(Agent::new(self.width as f32 / 2.0, self.height as f32 / 2.0, rand_range.sample(&mut rng)));
        }
    }
    pub fn add_random(&mut self, n_agents: usize) {
        self.agents.clear();
        self.can_run = true;
        let rand_x = Uniform::from(0..self.width);
        let rand_y = Uniform::from(0..self.height);
        let rand_angle = Uniform::from(0.0..(2.0 * PI));
        let mut rng = rand::thread_rng();
        for _ in 0..n_agents {
            let (x, y) = (rand_x.sample(&mut rng), rand_y.sample(&mut rng));
            self.agents.push(Agent::new(x as f32, y as f32, rand_angle.sample(&mut rng)));
        }
    }
    pub fn add_circle(&mut self, n_agents: usize, radius: f32) {
        self.agents.clear();
        self.can_run = true;
        let rand_radius: Uniform<f32> = Uniform::from(0.0..1.0);
        let rand_angle = Uniform::from(0.0..(2.0 * PI));
        let mut rng = rand::thread_rng();
        for _ in 0..n_agents {
            let angle = rand_angle.sample(&mut rng);
            let r = radius * rand_radius.sample(&mut rng).sqrt();
            let x = r * angle.cos() + self.width as f32 / 2.0;
            let y = r * angle.sin() + self.height as f32 / 2.0;
            self.agents.push(Agent::new(x, y, angle));
        }
    }
    pub fn add_inward_circle(&mut self, n_agents: usize, radius: f32) {
        self.agents.clear();
        self.can_run = true;
        let rand_radius: Uniform<f32> = Uniform::from(0.0..1.0);
        let rand_angle = Uniform::from(0.0..(2.0 * PI));
        let mut rng = rand::thread_rng();
        for _ in 0..n_agents {
            let angle = rand_angle.sample(&mut rng);
            let r = radius * rand_radius.sample(&mut rng).sqrt();
            let x = r * angle.cos() + self.width as f32 / 2.0;
            let y = r * angle.sin() + self.height as f32 / 2.0;
            self.agents.push(Agent::new(x, y, (angle + PI) % (2.0 * PI)));
        }
    }
    fn sense(field: &Vec<Vec<f64>>, agent: &Agent, angle_offset: f32, sense_dist: f32, sense_size: i32, width: u16, height: u16) -> f64 {
        let angle = agent.angle + angle_offset;
        let sense_dir = (angle.cos(), angle.sin());

        let sense_x = (agent.x + sense_dir.0 * sense_dist) as i32;
        let sense_y = (agent.y + sense_dir.1 * sense_dist) as i32;
        let mut sum = 0f64;

        for x in -sense_size..=sense_size {
            for y in -sense_size..=sense_size {
                let ix = (width - 1).min(0u16.max((sense_x + x) as u16));
                let iy = (height - 1).min(0u16.max((sense_y + y) as u16));
                sum += field[ix as usize][iy as usize];
            };
        };
        return sum;
    }
    pub fn update(&mut self, dt: f32) {
        let rand_range = Uniform::from(0.0..1.0);
        let mut rng = rand::thread_rng();
        for agent in self.agents.iter_mut() {
            let weight_forward = Field::sense(&self.field, agent, 0.0, self.settings.sense_distance, self.settings.sense_size, self.width, self.height);
            let weight_left = Field::sense(&self.field, agent, self.settings.sense_angle_difference, self.settings.sense_distance, self.settings.sense_size, self.width, self.height);
            let weight_right = Field::sense(&self.field, agent, -self.settings.sense_angle_difference, self.settings.sense_distance, self.settings.sense_size, self.width, self.height);

            let steer_strength = rand_range.sample(&mut rng);
            
            if (weight_forward > weight_left) && (weight_forward > weight_right) {
            } else if (weight_forward < weight_left) && (weight_forward < weight_right) {
                agent.angle += (steer_strength - 0.5) * 2.0 * self.settings.turn_speed * dt;
            } else if weight_right > weight_left {
                agent.angle -= steer_strength * self.settings.turn_speed * dt;
            } else if weight_left < weight_right {
                agent.angle += steer_strength * self.settings.turn_speed * dt;
            }

            let direction = (agent.angle.cos(), agent.angle.sin());
            let mut new_x = agent.x + direction.0 * self.settings.move_speed * dt;
            let mut new_y = agent.y + direction.1 * self.settings.move_speed * dt;

            if (new_x < 0f32) || (new_x >= self.width as f32) || (new_y < 0f32) || (new_y >= self.height as f32) {
                new_x = (self.width as f32 - 1.01).min(0f32.max(new_x));
                new_y = (self.height as f32 - 1.01).min(0f32.max(new_y));
                agent.angle = rand_range.sample(&mut rng) * 2.0 * PI;
            }

            agent.x = new_x;
            agent.y = new_y;
        }

        let mut temp = self.field.clone();
        for i in 0..self.width {
            for j in 0..self.height {
                let mut sum = 0f64;
                for x in -1i32..=1 {
                    for y in -1i32..=1 {
                        let ix = i as i32 + x;
                        let iy = j as i32 + y;
                        if (ix >= 0) && (ix < self.width as i32) && (iy >= 0) && (iy < self.height as i32) {
                            sum += self.field[ix as usize][iy as usize];
                        }
                    }
                }
                let blur = sum / 9.0;
                let diffused = (1.0 - (self.settings.diffuse_speed * dt)) as f64 * self.field[i as usize][j as usize] + (self.settings.diffuse_speed * dt) as f64 * blur;
                temp[i as usize][j as usize] = 0f64.max(diffused - (self.settings.evaporation_speed * dt) as f64);
            }
        }
        self.field = temp.clone();

        for agent in self.agents.iter() {
            self.field[agent.x as usize][agent.y as usize] = 1.0;
        }
    }
    pub fn draw(&self, img: &mut Image) {
        for (x, col) in self.field.iter().enumerate() {
            for (y, pixel) in col.iter().enumerate() {
                img.set_pixel(x as u32, y as u32, 
                    // Color::new(*pixel as f32, *pixel as f32, *pixel as f32, 1.0));
                    Color::from_vec(self.settings.background_color.to_vec().lerp(self.settings.pheromone_color.to_vec(), *pixel as f32)))
            }
        }
    }
}