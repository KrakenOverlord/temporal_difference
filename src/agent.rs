const ALPHA: f32 = 1.0;
const EPSILON: f32 = 0.01;
const GAMMA: f32 = 1.0;
const DEFAULT_ACTION_VALUE: f32 = 0.0;

#[derive(Copy, Clone, Debug)]
pub enum Action {
	Up(f32),
	Right(f32),
	Down(f32), 
	Left(f32),
}

impl Action {
    pub fn value(&self) -> f32 {
        match self {
            Action::Up(v)    => *v,
            Action::Right(v) => *v,
            Action::Down(v)  => *v,
            Action::Left(v)  => *v,
        }
    }

    pub fn convert(&self) -> crate::Action {
        match self {
            Action::Up(_) => crate::Action::Up,
            Action::Right(_) => crate::Action::Right,
            Action::Down(_) => crate::Action::Down,
            Action::Left(_) => crate::Action::Left,
        }
    }
}

pub struct State {
	pub row:        u32,
	pub col:        u32,
    pub actions:    Vec<Action>,
}

impl State {
    fn new(row: u32, col: u32) -> Self {
        Self { 
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

    // Returns epsilon greedy action value for the state
    fn next_action(&self) -> Action {
        let mut next_action = *self.actions.first().unwrap();
        let mut max_action_value = next_action.value();

        for action in &self.actions {
            let action_value = action.value();
            if action_value > max_action_value {
                max_action_value = action_value;
                next_action = *action;
            }
        }

        next_action
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        let mut actions = Vec::new();
        for action in &self.actions {
            actions.push(*action);
        }

        Self {
            row: self.row,
            col: self.col,
            actions: actions,
        }
    }
}

pub struct Agent {
    pub state:  State,
    pub action: Action,
    pub states: Vec<Vec<State>>,
}

impl Agent {
    // Returns an agent initialized with the starting state and first action selection.
    pub fn new(state: crate::environment::State, num_rows: u32, num_cols: u32) -> Agent {
        let states = Agent::initialize_states(num_rows, num_cols);
        let state = Agent::convert_state(state, &states);
        let action = state.next_action();
        
        Agent {
            state,
            action,
            states,
        }
    }

    fn initialize_states(num_rows: u32, num_cols: u32) -> Vec<Vec<State>> {
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

    pub fn iterate(&mut self, new_state: crate::environment::State, reward: f32) -> crate::Action {
        let new_state = Agent::convert_state(new_state, &self.states);

        let last_action_value = self.action.value();
        let next_action_value = new_state.next_action().value();
        let action_value = last_action_value + reward + ALPHA * (GAMMA * next_action_value - last_action_value);
        Agent::update_action_value(&mut self.states, self.state.clone(), self.action, action_value);

        let next_action = new_state.next_action();
        self.state = new_state;
        self.action = next_action;
        next_action.convert()
    }

    fn update_action_value(states: &mut Vec<Vec<State>>, state: State, action: Action, action_value: f32) {
        let mut new_actions: Vec<Action> = Vec::new();

        for a in state.actions {
            match a {
                Action::Up(_) => {
                    match action {
                        Action::Up(_) => {
                            new_actions.push(Action::Up(action_value));
                        },
                        _ => new_actions.push(a),
                    }
                },
                Action::Right(_) => {
                    match action {
                        Action::Right(_) => {
                            new_actions.push(Action::Right(action_value));
                        },
                        _ => new_actions.push(a),
                    }
                },
                Action::Down(_) => {
                    match action {
                        Action::Down(_) => {
                            new_actions.push(Action::Down(action_value));
                        },
                        _ => new_actions.push(a),
                    }
                },
                Action::Left(_) => {
                    match action {
                        Action::Left(_) => {
                            new_actions.push(Action::Left(action_value));
                        },
                        _ => new_actions.push(a),
                    }
                },
            }
        }

        states[state.row as usize][state.col as usize].actions = new_actions;
    }

    fn convert_state(state: crate::environment::State, states: &Vec<Vec<State>>) -> State {
        states[state.row as usize][state.col as usize].clone()
    }
}