use crate::environment::State;

#[derive(Copy, Clone, Debug)]
pub enum Action {
	Up,
	Right,
	Down, 
	Left,
}

pub struct Agent {}

impl Agent {
    pub fn act(&self, state: State, reward: f64) -> Action {
        Action::Up
    }
}