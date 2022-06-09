pub fn act(&mut self) -> bool {
    let mut converged: bool = true;

    // Update every state
    for row in 0..self.num_rows {
        for col in 0..self.num_cols {
            let state = &self.states[row as usize][col as usize];
            if state.terminal() {
                continue;
            }

            let next_value = match POLICY {
                Policy::Iteration => {
                    self.get_expected_value(state)
                },
                Policy::IterationInPlace => {
                    self.get_expected_value(state)
                },
                Policy::ValueIteration => {
                    self.get_greedy_value(state).unwrap()
                },
            };

            // Did we converge?
            if self.states[row as usize][col as usize].value != next_value {
                converged = false;
            }

            match POLICY {
                Policy::Iteration => {
                    self.states[row as usize][col as usize].next_value = next_value;
                },
                Policy::IterationInPlace => {
                    self.states[row as usize][col as usize].value = next_value;
                },
                Policy::ValueIteration => {
                    self.states[row as usize][col as usize].next_value = next_value;
                },
            };
        }
    }

    // Copy new states into old states
    if POLICY == Policy::Iteration || POLICY == Policy::ValueIteration {
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                self.states[row as usize][col as usize].value = self.states[row as usize][col as usize].next_value;
            }
        }
    }

    if IMPROVE_POLICY {
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                let row = row as usize;
                let col = col as usize;
                let state = &self.states[row][col];
                if state.terminal() {
                    continue;
                }

                match self.get_greedy_action(state) {
                    Some(a) => {
                        self.states[row][col].policy = a;
                    },
                    None => (),
                }
            } 
        }
    }

    converged
}

fn get_expected_value(&self, state: &State) -> f32 {
    let mut value = 0.0;
    for action in &state.actions {
        let probability = match action {
            Action::Up(av) => {
                av.probability
            },
            Action::Right(av) => {
                av.probability
            },
            Action::Down(av) => {
                av.probability
            },
            Action::Left(av) => {
                av.probability
            },
        };

        value += probability * self.get_action_value(state, action)
    }

    value
}

fn get_greedy_value(&self, state: &State) -> Option<f32> {
    if state.actions.len() == 0 {
        return None;
    }

    let mut value = -1000.0;
    for action in &state.actions {
        let current_value = self.get_action_value(state, action);

        if current_value > value {
            value = current_value;
        }
    }

    Some(value)
}

fn get_greedy_action(&self, state: &State) -> Option<Action> {
    if state.actions.len() == 0 {
        return None;
    }

    let first_action = state.actions.first().unwrap();
    let mut value = self.get_action_value(state, first_action);
    let mut greedy_action = first_action;
    for action in &state.actions {
        let current_value = self.get_action_value(state, action);

        if current_value > value {
            greedy_action = action;
            value = current_value;
        }
    }

    Some(greedy_action.clone())
}

// assumptions: 
//		- there is just deterministic state for each action
//		- trying to move outside a wall leaves you in the same state
fn get_action_value(&self, state: &State, action: &Action) -> f32 {
    match action {
        Action::Up(av) => {
            if state.row == 0 {
                av.reward + state.value
            }
            else {
                av.reward + self.states[(state.row - 1) as usize][state.col as usize].value
            }	
        },
        Action::Right(av) => {
            if state.col == (self.num_cols - 1) {
                av.reward + state.value
            }
            else {
                av.reward + self.states[(state.row) as usize][(state.col + 1) as usize].value
            }	
        },
        Action::Down(av) => {
            if state.row == (self.num_rows - 1) {
                av.reward + state.value
            }
            else {
                av.reward + self.states[(state.row + 1) as usize][state.col as usize].value
            }	
        },
        Action::Left(av) => {
            if state.col == 0 {
                av.reward + state.value
            }
            else {
                av.reward + self.states[state.row as usize][(state.col - 1) as usize].value
            }	
        },
    }
}
