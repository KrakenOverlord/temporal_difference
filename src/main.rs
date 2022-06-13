mod agent;
use agent::Agent;

mod environment;
use environment::{Environment, State};

mod draw;
use draw::draw;

use rand::prelude::SliceRandom;
use speedy2d::{Window, window::{WindowHelper, WindowHandler, VirtualKeyCode}, Graphics2D, color::Color, font::{Font, TextOptions, TextLayout}, shape::Rectangle, dimen::Vector2};

const MAX_EPISODES: u32 = 10_000;
const MAX_STEPS: u32 = 50_000;
const NUM_GOALS: u32 = 1;
const GOALS: [State; NUM_GOALS as usize] = [State { row: 0, col: 0, goal: true }];//, State { row: 3, col: 3, goal: true }];

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
	let x_dimension = NUM_COLS * CELL_SIZE + 2 * X_OFFSET;
	let y_dimension = NUM_ROWS * CELL_SIZE + 2 * Y_OFFSET;

    let window = Window::<()>::new_centered("Simulation", (x_dimension, y_dimension)).unwrap();
    window.run_loop(Main::new());
}
pub struct Main {
	auto_step:		bool,
	steps: 			u32,
	episodes:		u32,
	state: 			State,
	reward: 		f32,
	agent: 			Agent,
    environment: 	Environment,
	font: 			Font,
}

impl Main {
    pub fn new() -> Self {
		let bytes = include_bytes!("../assets/fonts/ariel.ttf");
		let font = Font::new(bytes).unwrap();

		let environment = Environment::new(NUM_ROWS, NUM_COLS, GOALS);
		let starting_state = Main::select_starting_state(&environment);
		let agent = Agent::new(starting_state, NUM_ROWS, NUM_COLS);
		let response = environment.respond(starting_state, agent.action.convert());

		println!("Agent initialized to {:#?}", agent);
		println!("Main initialized to {:#?}", response.0);
		println!("Main initialized to reward = {:#?}", response.1);

        Self { 
			auto_step: false,
			steps: 0,
			episodes: 0,
			state: response.0,
			reward: response.1,
			agent,
        	environment,
			font,
    	}
	}

	fn reset(&mut self) {
		let starting_state = Main::select_starting_state(&self.environment);
		self.agent.reset(starting_state);
		let response = self.environment.respond(starting_state, self.agent.action.convert());
		self.state = response.0;
		self.reward = response.1;
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

	fn on_step(&mut self) {
		if self.episodes >= MAX_EPISODES || self.steps >= MAX_STEPS {
			return;
		}

		let action = self.agent.iterate(self.state, self.reward);
		
		if self.state.goal {
			println!("Step: {}, Episodes: {}", self.steps, self.episodes);

			self.reset();
			self.episodes += 1;
			return;
		}

		let response = self.environment.respond(self.state, action);
		self.state = response.0;
		self.reward = response.1;

		self.steps += 1;
	}

	fn convert_state(&self, state: &agent::State) -> State {
		self.environment.states[state.row as usize][state.col as usize]
	}
}

impl WindowHandler for Main {
	fn on_draw(
		self: &mut Main,
		helper: &mut WindowHelper, 
		graphics: &mut Graphics2D
	) {		
		if self.auto_step == true {
			self.on_step();
		}
		
		graphics.clear_screen(Color::BLACK);
		draw(graphics, &self.font, &self.agent);
		helper.request_redraw();
	}

	fn on_key_down(
		&mut self,
		helper: &mut WindowHelper<()>,
		virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
		_scancode: speedy2d::window::KeyScancode
	) {
		println!("{:#?}", virtual_key_code);
		match virtual_key_code.unwrap() {
			VirtualKeyCode::A => {
				if self.auto_step == false {
					self.auto_step = true;
				} else {
					self.auto_step = false;
				}
			},
			VirtualKeyCode::Space => self.on_step(),
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