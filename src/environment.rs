use crate::Action;

#[derive(Copy, Clone, Debug)]
pub struct State {
	pub row: u32,
	pub col: u32,
	pub terminal: bool,
}

pub struct Environment {
	num_rows: u32, 
	num_cols: u32, 
	pub states: Vec<Vec<State>>,
}

impl Environment {
    pub fn new(num_rows: u32, num_cols: u32) -> Self {		
		Self { 
            num_rows,
            num_cols,
			states: Environment::create_state_grid(num_rows, num_cols),
		}
	}

	fn create_state_grid(num_rows: u32, num_cols: u32) -> Vec<Vec<State>> {
		let mut states = Vec::new();
		for row in 0..num_rows {
			let mut s: Vec<State> = Vec::new();
			for col in 0..num_cols  {
				s.push(State { 
					row: row, 
					col: col,
					terminal: false,
				});
			}
			states.push(s);
		}
		states
	}

	pub fn respond(&self, state: State, action: Option<Action>) -> Option<(State, f32)> {
		match action {
			Some(a) => {
				let s = self.iterate(state, a);
				Some(s)
			},
			None => None,
		}
	}

	// Returns the next state and reward given the current state and action.
	fn iterate(&self, state: State, action: Action) -> (State, f32) {
		match action {
			Action::Up => {
				if state.row == 0 {
					(state, -1.0)
				}
				else {
					(self.states[(state.row - 1) as usize][state.col as usize].clone(), -1.0)
				}	
			},
			Action::Right => {
				if state.col == (self.num_cols - 1) {
					(state, -1.0)
				}
				else {
					(self.states[(state.row) as usize][(state.col + 1) as usize].clone(), -1.0)
				}	
			},
			Action::Down => {
				if state.row == (self.num_rows - 1) {
					(state, -1.0)
				}
				else {
					(self.states[(state.row + 1) as usize][state.col as usize].clone(), -1.0)
				}	
			},
			Action::Left => {
				if state.col == 0 {
					(state, -1.0)
				}
				else {
					(self.states[state.row as usize][(state.col - 1) as usize].clone(), -1.0)
				}	
			},
		}
	}
}