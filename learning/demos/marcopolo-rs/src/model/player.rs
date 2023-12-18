use std::hash::{Hash, Hasher};

use crate::model::arena::Arena;
use crate::{DISCRETIZATION, PI, SCALE, TOROIDAL, TWOPI};

use core::fmt;
use krabmaga::{
    engine::{
        agent::Agent,
        fields::field_2d::{toroidal_transform, Location2D},
        location::Real2D,
        state::State,
    },
    log,
    rand::{self, Rng},
};
use nalgebra::{Rotation2, Vector2};
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq)]
pub enum PlayState {
    Waiting,
    Running,
    Seeking,
}

impl fmt::Display for PlayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayState::Running => write!(f, "running"),
            PlayState::Seeking => write!(f, "seeking"),
            PlayState::Waiting => write!(f, "waiting"),
        }
    }
}

/// The most basic agent should implement Clone, Copy and Agent to be able to be inserted in a Schedule.
#[derive(Clone, Copy)]
pub struct Player {
    pub id: Uuid,
    pub state: PlayState,
    pub loc: Real2D,
    pub heading: Vector2<f32>,
    pub speed: f32,
    pub detection_signal: Vector2<f32>,
    inhibit_counter: u32,
}

impl Player {
    pub fn new(x: f32, y: f32, heading_rad: f32, speed: f32) -> Player {
        let id = Uuid::new_v4();
        Player {
            id,
            state: PlayState::Running,
            loc: Real2D { x, y },
            heading: Vector2::new(heading_rad.sin(), heading_rad.cos()),
            speed: speed.max(0.1),
            inhibit_counter: 0,
            detection_signal: Vector2::zeros(),
        }
    }

    pub fn short_id(&self) -> String {
        format!("{id:.*}", 6, id = self.id.to_string())
    }

    fn get_current_speed(&self) -> f32 {
        match self.state {
            PlayState::Running => self.speed,
            PlayState::Seeking => self.speed / 2.0,
            PlayState::Waiting => 0.0,
        }
    }

    fn get_detection_fov(&self) -> f32 {
        match self.state {
            PlayState::Running => PI / 2.,
            PlayState::Seeking => TWOPI,
            PlayState::Waiting => 0.0,
        }
    }

    fn get_detection_range(&self) -> f32 {
        let base_range = SCALE * 10.0;
        match self.state {
            PlayState::Running => base_range,
            PlayState::Seeking => base_range * 10.0,
            PlayState::Waiting => 0.0,
        }
    }

    fn get_neighbors(&self, range: f32, world: &Arena) -> Vec<Player> {
        world.field.get_neighbors_within_distance(self.loc, range)
    }

    pub fn detect(&mut self, world: &Arena) -> Vec<Vector2<f32>> {
        let neighbors = self.get_neighbors(self.get_detection_range(), world);
        let mut detections: Vec<Vector2<_>> = neighbors
            .iter()
            .filter_map(|&neighbor| {
                let x = neighbor.loc.x - self.loc.x;
                let y = neighbor.loc.y - self.loc.y;
                let vector_to_other: Vector2<f32> = Vector2::new(x, y);
                if is_in_fov(self.get_detection_fov(), &self.heading, &vector_to_other)
                    && self.state != neighbor.state
                {
                    Some(inverse_square_law(vector_to_other))
                } else {
                    None
                }
            })
            .collect();
        detections.dedup();
        self.detection_signal = detections.iter().sum::<Vector2<f32>>();
        detections
    }

    pub fn notit(&mut self) {
        self.state = PlayState::Running;
    }

    pub fn tagged(&mut self) {
        self.inhibit_counter = 5;
        self.state = PlayState::Waiting;
        log!(
            LogType::Warning,
            format!(
                "{:} got tagged! Frozen for {:} ticks.",
                self.short_id(),
                self.inhibit_counter
            )
        );
    }

    fn maybe_tagged(&mut self, world: &Arena) {
        let neighbors = self.get_neighbors(DISCRETIZATION, world);
        let other_state = match self.state {
            PlayState::Running => PlayState::Seeking,
            _ => PlayState::Running,
        };
        let tagging_happened = neighbors.iter().any(|&n| n.state == other_state);

        if tagging_happened {
            match self.state {
                PlayState::Running => self.tagged(),
                PlayState::Seeking => self.notit(),
                PlayState::Waiting => {},
            }
        }
    }

    fn maybe_seek(&mut self) {
        if self.inhibit_counter > 0 {
            self.inhibit_counter -= 1;
            self.state = PlayState::Waiting;
            log!(
                LogType::Info,
                format!(
                    "{:} is still waiting. {:} ticks left.",
                    self.short_id(),
                    self.inhibit_counter
                )
            )
        } else if self.state == PlayState::Waiting {
            // done waiting, go seek!
            self.state = PlayState::Seeking;
        }
    }

    fn turn(&mut self, detections: &Vec<Vector2<f32>>) {
        let mut rng = rand::thread_rng();
        let angle_to_turn: Rotation2<f32>;
        if detections.is_empty() {
            angle_to_turn = Rotation2::new(PI * rng.gen_range(-1.0..=1.0))
        } else {
            let total_signal: Vector2<f32> = self.detection_signal;
            let negated_signal: Vector2<f32> = -total_signal;
            angle_to_turn = match self.state {
                PlayState::Seeking => Rotation2::rotation_between(&self.heading, &total_signal),
                PlayState::Running => Rotation2::rotation_between(&self.heading, &negated_signal),
                _ => Rotation2::new(0.0),
            }
        }
        self.heading = angle_to_turn.transform_vector(&self.heading);
    }

    fn velocity(&self) -> Real2D {
        let adjusted_speed = self.get_current_speed();
        Real2D {
            x: adjusted_speed * self.heading.x,
            y: adjusted_speed * self.heading.y,
        }
    }

    fn next_loc(&self) -> Real2D {
        // Get the next location the Agent will be at based on current heading and speed
        let velocity: Real2D = self.velocity();
        Real2D {
            x: self.loc.x + velocity.x,
            y: self.loc.y + velocity.y,
        }
    }

    fn translate(&mut self, world: &Arena) {
        let maybe_next_loc: Real2D = self.next_loc();
        let next_loc: Real2D;
        if TOROIDAL {
            next_loc = toroidal_transform_2d(world, maybe_next_loc);
        } else {
            let (oobx, ooby) = out_of_bounds(world, maybe_next_loc);
            if oobx {
                self.heading[0] = -self.heading[0];
            }
            if ooby {
                self.heading[1] = -self.heading[1];
            }
            next_loc = self.next_loc();
        }
        self.loc = next_loc;
    }
}

impl Agent for Player {
    /// Put the code that should happen for each step, for each agent here.
    fn step(&mut self, arena: &mut dyn State) {
        let world = arena.as_any_mut().downcast_mut::<Arena>().unwrap();
        self.maybe_seek();
        self.maybe_tagged(world);
        let detections: Vec<Vector2<f32>> = self.detect(world);
        self.turn(&detections);
        self.translate(world);
        world.field.set_object_location(*self, self.loc);
        log!(LogType::Info, format!("{:}", self));
    }

    /// Put the code that decides if an agent should be removed or not
    /// for example in simulation where agents can die
    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }
}

impl Hash for Player {
    fn hash<H>(&self, world: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(world);
    }
}

impl Location2D<Real2D> for Player {
    fn get_location(self) -> Real2D {
        self.loc
    }

    fn set_location(&mut self, loc: Real2D) {
        self.loc = loc;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}) x:{:>6.2} y:{:>6.2} hdg:{:>5.1}° det: {:>6.2} @ {:>5.1}°",
            self.short_id(),
            self.state,
            self.loc.x,
            self.loc.y,
            vec_to_heading(self.heading),
            self.detection_signal.magnitude(),
            vec_to_heading(self.detection_signal),
        )
    }
}

impl Eq for Player {}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.id == other.id
    }
}

fn toroidal_transform_2d(world: &Arena, loc: Real2D) -> Real2D {
    let x = toroidal_transform(loc.x, world.field.width);
    let y = toroidal_transform(loc.y, world.field.height);
    Real2D { x, y }
}

fn out_of_bounds(world: &Arena, loc: Real2D) -> (bool, bool) {
    let oob_x: bool = loc.x > world.field.width || loc.x < 0.;
    let oob_y: bool = loc.y > world.field.height || loc.y < 0.;
    (oob_x, oob_y)
}

pub fn is_in_fov(fov: f32, heading: &Vector2<f32>, to_other: &Vector2<f32>) -> bool {
    let angle_between = heading.angle(to_other);
    if angle_between <= fov {
        true
    } else {
        false
    }
}

pub fn inverse_square_law(v: Vector2<f32>) -> Vector2<f32> {
    v.scale(1. / v.magnitude().powf(2.0))
}

pub fn vec_to_heading(v: Vector2<f32>) -> f32 {
    v.angle(&Vector2::y_axis()) * 180.0 / PI
}
