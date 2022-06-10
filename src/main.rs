mod agent;
use agent::Agent;

mod environment;
use environment::{Environment, State};

use speedy2d::{Window, window::{WindowHelper, WindowHandler}, Graphics2D, color::Color, font::Font};

const NUM_COLS: u32 = 4;
const NUM_ROWS: u32 = 2;
const X_OFFSET: u32 = 50;
const Y_OFFSET: u32 = 50;
const CELL_SIZE: u32 = 100;
const MANUAL_STEP: bool = false;
const STARTING_STATE: State = State { row: 0, col: 0, terminal: false };
const EPSILON: f32 = 0.01;

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
	steps: 			u32,
	state: 			State,
	reward: 		f32,
	agent: 			Agent,
    environment: 	Environment,
}

impl Main {
    pub fn new() -> Self {
		let bytes = include_bytes!("../assets/fonts/ariel.ttf");
		let font = Font::new(bytes).unwrap();

        Self { 
			steps: 0,
			state: STARTING_STATE,
			reward: 0.0,
			agent: Agent::new(STARTING_STATE, NUM_ROWS, NUM_COLS),
        	environment: Environment::new(NUM_ROWS, NUM_COLS),
    	}
	}

	fn draw(&self, graphics: &mut Graphics2D) {
		self.draw_grid(graphics);
		self.draw_states(graphics);
		self.draw_agent(graphics);
	}
	
	fn draw_grid(&self, graphics: &mut Graphics2D) {
		// Draw horizontal lines.
		for row in 0..(NUM_ROWS + 1) {
			let y = (Y_OFFSET + row * CELL_SIZE) as f32;
			let begin = (X_OFFSET as f32, y);
			let end = ((X_OFFSET + NUM_COLS * CELL_SIZE) as f32, y);
			graphics.draw_line(begin, end, 1.0, Color::GRAY)
		}
	
		// Draw vertical lines.
		for col in 0..(NUM_COLS + 1) {
			let x = (X_OFFSET + col * CELL_SIZE) as f32;
			let begin = (x, Y_OFFSET as f32);
			let end = (x, (Y_OFFSET + NUM_ROWS * CELL_SIZE) as f32);
			graphics.draw_line(begin, end, 1.0, Color::GRAY)
		}
	}
	
	fn draw_states(&self, graphics: &mut Graphics2D) {
		for row in &self.environment.states {
			for state in row  {
				// draw policy
				// if self.terminal() == false {
				// 	let policy = match self.policy {
				// 		Action::Up(_) => "U",
				// 		Action::Right(_) => "R",
				// 		Action::Down(_) => "D",
				// 		Action::Left(_) => "L",
				// 	};
				// 	let policy_text = format!("{}{}:{}", self.row, self.col, policy);
				// 	let value_block = font.layout_text(&policy_text, 0.4 * cell_size as f32, TextOptions::new());
				// 	let x = x_offset as f32 + self.col as f32 * cell_size as f32 + 0.5 * cell_size as f32 - 0.5 * value_block.width();
				// 	let y = y_offset as f32 + self.row as f32 * cell_size as f32 + 0.5 * cell_size as f32 - value_block.height();
				// 	graphics.draw_text((x.round(), y.round()), Color::WHITE, &value_block);
				// }

				// draw value
				// let value_text = format!("{:.4}", self.value);
				// let value_block = font.layout_text(&value_text, 0.4 * cell_size as f32, TextOptions::new());
				// let x = x_offset as f32 + self.col as f32 * cell_size as f32 + 0.5 * cell_size as f32 - 0.5 * value_block.width();
				// let y = y_offset as f32 + self.row as f32 * cell_size as f32 + 0.5 * cell_size as f32 - 0.25 * value_block.height();
				// graphics.draw_text((x.round(), y.round()), Color::WHITE, &value_block);
			}
		}
	}

	fn draw_agent(&self, graphics: &mut Graphics2D) {
	}
}

impl WindowHandler for Main {
	fn on_draw(
		self: &mut Main,
		helper: &mut WindowHelper, 
		graphics: &mut Graphics2D
	) {
		graphics.clear_screen(Color::BLACK);
		
		let action = self.agent.act(self.state, self.reward, EPSILON);
		let (state, reward) = self.environment.respond(self.state, action);
		self.state = state;
		self.reward = reward;
		
		self.steps += 1;
		println!("Step: {}", self.steps);
		
		self.draw(graphics);
		helper.request_redraw();
	}

	fn on_key_down(
		&mut self,
		_helper: &mut WindowHelper<()>,
		_virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
		_scancode: speedy2d::window::KeyScancode
	) {
		// self.converged = self.environment.act();
		// self.steps += 1;
		// if self.converged {
		// 	println!("Converged after {} steps.", self.steps - 1);
		// } else {
		// 	println!("Steps: {}", self.steps);
		// }
	}
}