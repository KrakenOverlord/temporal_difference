use draw::Draw;
use rand::prelude::SliceRandom;
use speedy2d::{Window, window::{WindowHelper, WindowHandler, VirtualKeyCode}, Graphics2D, color::Color};

mod agent;
use agent::Agent;

mod environment;
use environment::{Environment, State};

mod draw;

const AUTO_STEP: bool = false;
const MAX_EPISODES: u32 = 10_000;
const MAX_STEPS: u32 = 100_000;
const NUM_GOALS: u32 = 1;
const GOALS: [State; NUM_GOALS as usize] = [State { row: 0, col: 0, goal: true }];

const NUM_ROWS: u32 = 4;
const NUM_COLS: u32 = 4;
const X_OFFSET: u32 = 50;
const Y_OFFSET: u32 = 50;
const CELL_SIZE: u32 = 200;

#[derive(Copy, Clone, Debug)]
pub enum Action {
	Up,
	Right,
	Down, 
	Left,
}

fn main() {
	let mut main = Main::new();
    if AUTO_STEP {
		loop {
			main.step();
			if main.episodes >= MAX_EPISODES || main.steps >= MAX_STEPS {
				break;
			}
		}
	}

	window().run_loop(main);
}

fn window() -> Window {
	let x_dimension = NUM_COLS * CELL_SIZE + 2 * X_OFFSET;
	let y_dimension = NUM_ROWS * CELL_SIZE + 2 * Y_OFFSET;
    Window::<()>::new_centered("Simulation", (x_dimension, y_dimension)).unwrap()
}
pub struct Main {
	auto_step:		bool,
	steps: 			u32,
	episodes:		u32,
	state: 			State,
	reward: 		f32,
	agent: 			Agent,
    environment: 	Environment,
	draw:			Draw,
}

impl Main {
    pub fn new() -> Self {
		let environment = Environment::new(NUM_ROWS, NUM_COLS, GOALS);
		let starting_state = Main::select_starting_state(&environment);
		let agent = Agent::new(NUM_ROWS, NUM_COLS);

		println!("Starting state {:#?}", starting_state);

        Self { 
			auto_step: false,
			steps: 0,
			episodes: 0,
			state: starting_state,
			reward: 0.0,
			agent,
        	environment,
			draw: Draw::new(),
    	}
	}

	fn reset(&mut self) {
		let starting_state = Main::select_starting_state(&self.environment);
		self.agent.reset();
		self.state = Main::select_starting_state(&self.environment);
		self.reward = 0.0;
	}

	fn select_starting_state(environment: &Environment) -> State {
		let rows = &environment.states;
		loop {
			let row = rows.choose(&mut rand::thread_rng()).unwrap();
			let state = row.choose(&mut rand::thread_rng()).unwrap();

			let mut matched = false;
			for goal in GOALS {
				if goal.row == state.row && goal.col == state.col {
					matched = true;
				}
			}

			if matched == false {
				return state.clone();
			}
		}
	}

	fn step(&mut self) {
		let action = self.agent.iterate(self.state, self.reward);
		
		if self.state.goal {
			self.reset();
			self.episodes += 1;
		}

		let response = self.environment.respond(self.state, action);
		self.state = response.0;
		self.reward = response.1;

		self.steps += 1;
	}
}

impl WindowHandler for Main {
	fn on_draw(
		self: &mut Main,
		helper: &mut WindowHelper, 
		graphics: &mut Graphics2D
	) {		
		if self.auto_step == true {
			self.step();
		}
		
		let title = format!("Episodes: {} Steps: {}", self.episodes, self.steps);
		helper.set_title(title);
		graphics.clear_screen(Color::BLACK);
		self.draw.draw(graphics, &self.agent);
		helper.request_redraw();
	}

	fn on_key_down(
		&mut self,
		helper: &mut WindowHelper<()>,
		virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
		_scancode: speedy2d::window::KeyScancode
	) {
		match virtual_key_code.unwrap() {
			VirtualKeyCode::A => {
				self.auto_step = !self.auto_step;
			},
			VirtualKeyCode::Space => {
				self.step();
			},
			VirtualKeyCode::Q => helper.terminate_loop(),
			_ => {},
		}
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_modulo() {
        println!("{}", 3 % 4);
		println!("{}", 17 % 4);
		println!("{}", 4 % 4);
        println!("{}", 16 % 4);
    }
}