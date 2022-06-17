use rand::prelude::SliceRandom;

const EPSILON: u32 = 5;
const ALPHA: f32 = 0.1;
const GAMMA: f32 = 0.8;
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

#[derive(Debug)]
pub struct State {
    pub visits:     u32,
	pub row:        u32,
	pub col:        u32,
    pub actions:    Vec<Action>,
}

impl State {
    fn new(row: u32, col: u32) -> Self {
        Self { 
            visits: 0,
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

    fn reset(&mut self) {
        self.visits = 0;
    }

    // Returns epsilon greedy action value for the state. Randomly break ties.
    fn select_action(&mut self) -> Action {
        self.visits += 1;

        if self.visits % EPSILON == 0 {
            return *self.actions.choose(&mut rand::thread_rng()).unwrap();
        }

        let mut actions = Vec::new();
        let mut max_action_value = self.actions.first().unwrap().value();

        for action in &self.actions {
            let action_value = action.value();
            if action_value == max_action_value {
                actions.push(*action);
            } else if action_value > max_action_value {
                max_action_value = action_value;
                actions.clear();
                actions.push(*action);
            }
        }

        *actions.choose(&mut rand::thread_rng()).unwrap()
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        let mut actions = Vec::new();
        for action in &self.actions {
            actions.push(*action);
        }

        Self {
            visits:     self.visits,
            row:        self.row,
            col:        self.col,
            actions:    actions,
        }
    }
}

#[derive(Debug)]
pub struct Agent {
    pub states:         Vec<Vec<State>>,
    pub last_state:     Option<State>,
    pub last_action:    Option<Action>,
}

impl Agent {
    // Returns an agent initialized with the starting state and first action selection.
    pub fn new(num_rows: u32, num_cols: u32) -> Self {
        let states = Agent::initialize_states(num_rows, num_cols);

        Self {
            states,
            last_state: None,
            last_action: None,
        }
    }

    pub fn reset(&mut self) {
        // reset state visits to zero
        for row in &mut self.states {
            for state in row {
                state.reset();
            }
        }
        
        self.last_state = None;
        self.last_action = None;
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

    pub fn iterate(&mut self, next_state: crate::environment::State, reward: f32) -> crate::Action {
        let mut next_state = Agent::convert_state(next_state, &self.states);
        let next_action = next_state.select_action();

        // Q(s, a) ← Q(s, a) + α (r + γQ(s', a') − Q(s, a))
        // if α = γ = 1, then Q(s, a) ← r + Q(s', a')
        let prediction = match self.last_action {
            Some(a) => a.value(),
            None => 0.0,
        };
        let target = reward + GAMMA * next_action.value();
        let updated_action_value = prediction + ALPHA * (target - prediction);
        self.update_last_action_value(updated_action_value);

        self.last_state = Some(next_state);
        self.last_action = Some(next_action);
        next_action.convert()
    }

    fn update_last_action_value(&mut self, new_action_value: f32) {
        let last_state = match &self.last_state {
            Some(s) => s,
            None => return,
        };

        let last_action = match &self.last_action {
            Some(a) => a,
            None => return,
        };

        let mut new_actions: Vec<Action> = Vec::new();
        for a in &last_state.actions {
            match a {
                Action::Up(_) => {
                    match last_action {
                        Action::Up(_) => {
                            new_actions.push(Action::Up(new_action_value));
                        },
                        _ => new_actions.push(*a),
                    }
                },
                Action::Right(_) => {
                    match last_action {
                        Action::Right(_) => {
                            new_actions.push(Action::Right(new_action_value));
                        },
                        _ => new_actions.push(*a),
                    }
                },
                Action::Down(_) => {
                    match last_action {
                        Action::Down(_) => {
                            new_actions.push(Action::Down(new_action_value));
                        },
                        _ => new_actions.push(*a),
                    }
                },
                Action::Left(_) => {
                    match last_action {
                        Action::Left(_) => {
                            new_actions.push(Action::Left(new_action_value));
                        },
                        _ => new_actions.push(*a),
                    }
                },
            }
        }

        self.states[last_state.row as usize][last_state.col as usize].actions = new_actions;
    }

    fn convert_state(state: crate::environment::State, states: &Vec<Vec<State>>) -> State {
        states[state.row as usize][state.col as usize].clone()
    }
}