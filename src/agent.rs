const DEFAULT_ACTION_VALUE: f32 = 0.0;

#[derive(Copy, Clone, Debug)]
enum Action {
	Up(f32),
	Right(f32),
	Down(f32), 
	Left(f32),
}

struct State {
	pub row:        u32,
	pub col:        u32,
    pub actions:    Vec<Action>,
}

impl State {
    fn new(row: u32, col: u32) -> State {
        State { 
            row: row, 
            col: col,
            actions: vec![
                Action::Up(DEFAULT_ACTION_VALUE),
                Action::Right(DEFAULT_ACTION_VALUE),
                Action::Down(DEFAULT_ACTION_VALUE),
                Action::Left(DEFAULT_ACTION_VALUE),
            ],
        }
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        let actions = Vec::new();
        for action in self.actions {
            actions.push(action);
        }

        State {
            row: self.row,
            col: self.col,
            actions: actions,
        }
    }
}

pub struct Agent {
    last_row:       u32,
    last_col:       u32,
    last_action:    Option<Action>,
    states:         Vec<Vec<State>>,
}

impl Agent {
    pub fn new(start_state: crate::environment::State, num_rows: u32, num_cols: u32) -> Agent {
        Agent {
            last_row:       start_state.row,
            last_col:       start_state.col,
            last_action:    None,
            states:         Agent::create_state_grid(num_rows, num_cols),
        }
    }

    fn create_state_grid(num_rows: u32, num_cols: u32) -> Vec<Vec<State>> {
        let mut states =Vec::new();
		for row in 0..num_rows {
			let mut s: Vec<State> = Vec::new();
			for col in 0..num_cols  {
				s.push(State::new(row, col));
			}
			states.push(s);
		}
        states
    }

    pub fn act(&mut self, state: crate::environment::State, reward: f32, epsilon: f32) -> Option<crate::Action> {
        let state = self.convert_state(state);

        let last_action_value = self.get_last_action_value();
        let next_action_value = self.get_next_action_value(state);
        let action_value = last_action_value + reward + epsilon * (next_action_value - last_action_value);
        self.update_action_value(self.last_row, self.last_col, self.last_action, action_value);

        let next_action = self.get_next_action(state);
        match next_action {
            Some(a) => Some(Agent::unconvert_action(a)),
            None => None,
        }
    }

    fn get_last_action_value(&self) -> f32 {
        match self.last_action {
            Some(action) => Agent::get_action_value(action),
            None => DEFAULT_ACTION_VALUE,
        }
    }

    fn get_next_action_value(&self, state: State) -> f32 {
        let action = self.get_next_action(state);
        match action {
            Some(a) => Agent::get_action_value(a),
            None => DEFAULT_ACTION_VALUE,
        }
    }

    fn get_action_value(action: Action) -> f32 {
        match action {
            Action::Up(v)    => v,
            Action::Right(v) => v,
            Action::Down(v)  => v,
            Action::Left(v)  => v,
        }
    }

    fn update_action_value(&mut self, row: u32, col: u32, action: Option<Action>, action_value: f32) {

    }

    // Returns epsilon greedy action value for the state
    fn get_next_action(&self, state: State) -> Option<Action> {
        let mut next_action = None;
        let mut max_action_value = DEFAULT_ACTION_VALUE;

        for action in state.actions {
            let action_value = Agent::get_action_value(action);
            if action_value > max_action_value {
                max_action_value = action_value;
                next_action = Some(action);
            }
        }

        next_action
    }

    fn convert_state(&self, state: crate::environment::State) -> State {
        self.states[state.row as usize][state.col as usize]
    }

    fn unconvert_action(action: Action) -> crate::Action {
        match action {
            Action::Up(_) => crate::Action::Up,
            Action::Right(_) => crate::Action::Right,
            Action::Down(_) => crate::Action::Down,
            Action::Left(_) => crate::Action::Left,
        }
    }
}