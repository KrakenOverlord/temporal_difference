use crate::agent::Action;

#[derive(Copy, Clone, Debug)]
pub struct State {
	pub row: u32,
	pub col: u32,
	pub terminal: bool,
}

pub struct Environment {
	num_rows: u32, 
	num_cols: u32, 
	states: Vec<Vec<State>>,
}

impl Environment {
    pub fn new(num_rows: u32, num_cols: u32) -> Self {		
        let mut states = vec![];

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
		
		Self { 
            num_rows,
            num_cols,
			states,
		}
	}  

	pub fn respond(&self, state: State, action: Action) -> (State, f64) {
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