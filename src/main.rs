mod agent;

use agent::Agent;

mod environment;
use environment::{Environment, State};

use speedy2d::{Window, window::{WindowHelper, WindowHandler}, Graphics2D, color::Color, font::{Font, TextOptions, TextLayout}};

const MANUAL_STEP: bool = true;
const STARTING_STATE: State = State { row: 4, col: 4, goal: false };

const NUM_GOALS: u32 = 1;
const GOALS: [State; NUM_GOALS as usize] = [State { row: 0, col: 0, goal: true }];

const NUM_COLS: u32 = 8;
const NUM_ROWS: u32 = 8;
const X_OFFSET: u32 = 50;
const Y_OFFSET: u32 = 50;
const CELL_SIZE: u32 = 100;

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
	font: 			Font,
}

impl Main {
    pub fn new() -> Self {
		let bytes = include_bytes!("../assets/fonts/ariel.ttf");
		let font = Font::new(bytes).unwrap();

		let agent = Agent::new(STARTING_STATE, NUM_ROWS, NUM_COLS);
		let environment = Environment::new(NUM_ROWS, NUM_COLS, GOALS);
		let response = environment.respond(STARTING_STATE, agent.action.convert());

        Self { 
			steps: 0,
			state: response.0,
			reward: response.1,
			agent,
        	environment,
			font,
    	}
	}

	fn on_step(&mut self) {
		let action = self.agent.iterate(self.state, self.reward);
		let response = self.environment.respond(self.state, action);
		self.state = response.0;
		self.reward = response.1;
		
		self.steps += 1;
		println!("Step: {}", self.steps);
	}

	fn draw(&self, graphics: &mut Graphics2D) {
		self.draw_grid(graphics);
		self.draw_agent_states(graphics);
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
	
	fn draw_agent_states(&self, graphics: &mut Graphics2D) {
		let value = self.agent.action.value();

		let value_text = format!("{:.4}", value);
		let value_block = self.font.layout_text(&value_text, 0.4 * CELL_SIZE as f32, TextOptions::new());
		let x = X_OFFSET as f32 + self.agent.state.col as f32 * CELL_SIZE as f32 + 0.5 * CELL_SIZE as f32 - 0.5 * value_block.width();
		let y = Y_OFFSET as f32 + self.agent.state.row as f32 * CELL_SIZE as f32 + 0.5 * CELL_SIZE as f32 - 0.25 * value_block.height();
		graphics.draw_text((x.round(), y.round()), Color::WHITE, &value_block);
		
		// for row in &self.agent.states {
		// 	for state in row  {
		// 		draw policy
		// 		if self.terminal() == false {
		// 			let policy = match self.policy {
		// 				Action::Up(_) => "U",
		// 				Action::Right(_) => "R",
		// 				Action::Down(_) => "D",
		// 				Action::Left(_) => "L",
		// 			};
		// 			let policy_text = format!("{}{}:{}", self.row, self.col, policy);
		// 			let value_block = font.layout_text(&policy_text, 0.4 * cell_size as f32, TextOptions::new());
		// 			let x = x_offset as f32 + self.col as f32 * cell_size as f32 + 0.5 * cell_size as f32 - 0.5 * value_block.width();
		// 			let y = y_offset as f32 + self.row as f32 * cell_size as f32 + 0.5 * cell_size as f32 - value_block.height();
		// 			graphics.draw_text((x.round(), y.round()), Color::WHITE, &value_block);
		// 		}

		// 		draw value
		// 		let value_text = format!("{:.4}", state.last_action);
		// 		let value_block = font.layout_text(&value_text, 0.4 * cell_size as f32, TextOptions::new());
		// 		let x = x_offset as f32 + self.col as f32 * cell_size as f32 + 0.5 * cell_size as f32 - 0.5 * value_block.width();
		// 		let y = y_offset as f32 + self.row as f32 * cell_size as f32 + 0.5 * cell_size as f32 - 0.25 * value_block.height();
		// 		graphics.draw_text((x.round(), y.round()), Color::WHITE, &value_block);
		// 	}
		// }
	}

	fn draw_agent(&self, graphics: &mut Graphics2D) {
		let agent_text = String::from("x");
		let value_block = self.font.layout_text(&agent_text, 0.4 * CELL_SIZE as f32, TextOptions::new());
		let x = X_OFFSET as f32 + self.agent.state.col as f32 * CELL_SIZE as f32 + 0.5 * CELL_SIZE as f32 - 0.5 * value_block.width();
		let y = Y_OFFSET as f32 + self.agent.state.row as f32 * CELL_SIZE as f32 + 0.5 * CELL_SIZE as f32 - 0.5 * value_block.height();
		graphics.draw_text((x.round(), y.round()), Color::WHITE, &value_block);
	}
}

impl WindowHandler for Main {
	fn on_draw(
		self: &mut Main,
		helper: &mut WindowHelper, 
		graphics: &mut Graphics2D
	) {		
		if MANUAL_STEP == false {
			self.on_step();
		}
		
		graphics.clear_screen(Color::BLACK);
		self.draw(graphics);
		helper.request_redraw();
	}

	fn on_key_down(
		&mut self,
		_helper: &mut WindowHelper<()>,
		_virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
		_scancode: speedy2d::window::KeyScancode
	) {
		self.on_step();
	}
}