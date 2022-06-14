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
    pub state:  State,
    pub action: Action,
    pub states: Vec<Vec<State>>,
}

impl Agent {
    // Returns an agent initialized with the starting state and first action selection.
    pub fn new(state: crate::environment::State, num_rows: u32, num_cols: u32) -> Agent {
        let states = Agent::initialize_states(num_rows, num_cols);
        let mut state = Agent::convert_state(state, &states);
        let action = state.select_action();
        
        println!("Agent initialized to {:#?}", state);
        println!("Agent initialized to action: {:#?}", action);

        Agent {
            state,
            action,
            states,
        }
    }

    pub fn reset(&mut self, state: crate::environment::State) {
        // reset state visits to zero
        for row in &mut self.states {
            for state in row {
                state.reset();
            }
        }
        
        let mut state = Agent::convert_state(state, &self.states);
        self.state = state.clone();
        self.action = state.select_action();
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
        let mut new_state = Agent::convert_state(new_state, &self.states);
        let new_action = new_state.select_action();

        // Q(s, a) ← Q(s, a) + α (r + γQ(s0, a0) − Q(s, a))
        let prediction = self.action.value();
        let target = reward + GAMMA * new_action.value();
        let updated_action_value = prediction + ALPHA * (target - prediction);
        Agent::update_action_value(&mut self.states, self.state.clone(), self.action, updated_action_value);

        self.state = new_state;
        self.action = new_action;
        new_action.convert()
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