use crate::{Action, NUM_GOALS};

#[derive(Copy, Clone, Debug)]
pub struct State {
	pub row: u32,
	pub col: u32,
	pub goal: bool,
}

pub struct Environment {
	num_rows: u32, 
	num_cols: u32, 
	pub states: Vec<Vec<State>>,
}

impl Environment {
    pub fn new(num_rows: u32, num_cols: u32, goals: [State; NUM_GOALS as usize]) -> Self {		
		Self { 
            num_rows,
            num_cols,
			states: Environment::initialize_states(num_rows, num_cols, goals),
		}
	}

	fn initialize_states(num_rows: u32, num_cols: u32, goals: [State; NUM_GOALS as usize]) -> Vec<Vec<State>> {
		let mut states = Vec::new();
		for row in 0..num_rows {
			let mut s: Vec<State> = Vec::new();
			for col in 0..num_cols  {
				s.push(State { 
					row: row, 
					col: col,
					goal: Environment::is_goal(row, col, goals),
				});
			}
			states.push(s);
		}
		states
	}

	pub fn is_goal(row: u32, col: u32, goals: [State; NUM_GOALS as usize]) -> bool {
		for goal in goals {
			if goal.row == row && goal.col == col {
				return true;
			}
		}
		false
	}

	// Returns (next state, -1.0) given the current state and action.
	// If state = goal then return (goal, 0.0)
	// If next state is goal then return (goal, 0.0)
	pub fn respond(&self, state: State, action: Action) -> (State, f32) {
		if state.goal {
			return (state, 0.0);
		}

		let mut response = match action {
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
					(self.states[state.row as usize][(state.col + 1) as usize].clone(), -1.0)
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
		};

		if response.0.goal {
			response.1 = 0.0;
		}

		response
	}
}