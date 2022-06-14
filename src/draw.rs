use std::rc::Rc;

use speedy2d::{Graphics2D, color::Color, dimen::Vector2, shape::Rectangle, font::{TextOptions, Font, TextLayout, FormattedTextBlock}};

use crate::{NUM_ROWS, NUM_COLS, X_OFFSET, Y_OFFSET, CELL_SIZE, GOALS, agent::{self, Agent, State, Action}, environment::Environment};

pub fn draw(graphics: &mut Graphics2D, font: &Font, agent: &Agent) {
    draw_grid(graphics);
    draw_goals(graphics);
    draw_agent_location(graphics, agent);
    draw_agent_action_values(graphics, font, agent);
}

fn draw_grid(graphics: &mut Graphics2D) {
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

fn draw_goals(graphics: &mut Graphics2D) {
    for goal in GOALS {
        let x = X_OFFSET + goal.col * CELL_SIZE;
        let y = Y_OFFSET + goal.row * CELL_SIZE;
        let top_left = Vector2::new(x as f32, y as f32);
        let bottom_right = Vector2::new((x + CELL_SIZE) as f32, (y + CELL_SIZE) as f32);
        graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), Color::from_rgb(51.0/255.0, 153.0/255.0, 102.0/255.0));
    }
}

fn draw_agent_location(graphics: &mut Graphics2D, agent: &Agent) {
    let x = X_OFFSET + agent.state.col * CELL_SIZE;
    let y = Y_OFFSET + agent.state.row * CELL_SIZE;
    let top_left = Vector2::new(x as f32, y as f32);
    let bottom_right = Vector2::new((x + CELL_SIZE) as f32, (y + CELL_SIZE) as f32);
    graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), Color::GRAY);
}

fn draw_agent_action_values(graphics: &mut Graphics2D, font: &Font, agent: &Agent) {	
    for row in &agent.states {
        for state in row  {
            // don't draw action values if a goal state
            if Environment::is_goal(state.row, state.col, GOALS) {
                continue;
            }

            for action in &state.actions {
                let color = color(font, agent, state, action);
                let text_block = text_block(font, agent, state, action);

                match action {
                    agent::Action::Up(_) => {
                        let x = (X_OFFSET + state.col * CELL_SIZE) as f32 + 0.5 * CELL_SIZE as f32 - 0.5 * text_block.width();
                        let y = (Y_OFFSET + state.row * CELL_SIZE) as f32 + 0.1 * CELL_SIZE as f32;
                        graphics.draw_text((x.round(), y.round()), color, &text_block);			
                    },
                    agent::Action::Right(_) => {
                        let x = (X_OFFSET + state.col * CELL_SIZE) as f32 + 0.9 * CELL_SIZE as f32 - text_block.width();
                        let y = (Y_OFFSET + state.row * CELL_SIZE) as f32 + 0.5 * CELL_SIZE as f32 - 0.5 * text_block.height();
                        graphics.draw_text((x.round(), y.round()), color, &text_block);	
                    },
                    agent::Action::Down(_) => {
                        let x = (X_OFFSET + state.col * CELL_SIZE) as f32 + 0.5 * CELL_SIZE as f32 - 0.5 * text_block.width();
                        let y = (Y_OFFSET + state.row * CELL_SIZE) as f32 + 0.9 * CELL_SIZE as f32 - text_block.height();
                        graphics.draw_text((x.round(), y.round()), color, &text_block);		
                    },
                    agent::Action::Left(_) => {
                        let x = (X_OFFSET + state.col * CELL_SIZE) as f32 + 0.1 * CELL_SIZE as f32;
                        let y = (Y_OFFSET + state.row * CELL_SIZE) as f32 + 0.5 * CELL_SIZE as f32 - 0.5 * text_block.height();
                        graphics.draw_text((x.round(), y.round()), color, &text_block);	
                    },
                }
            }
        }
    }
}

fn color(font: &Font, agent: &Agent, state: &State, action: &Action) -> Color {
    let mut color = Color::WHITE;
    if agent.state.row == state.row && agent.state.col == state.col {
        color = match action {
            Action::Up(_) => {
                match agent.action {
                    Action::Up(_) => Color::RED,
                    _ => Color::WHITE,
                }
            },
            Action::Right(_) => {
                match agent.action {
                    Action::Right(_) => Color::RED,
                    _ => Color::WHITE,
                }
            },
            Action::Down(_) => {
                match agent.action {
                    Action::Down(_) => Color::RED,
                    _ => Color::WHITE,
                }
            },
            Action::Left(_) => {
                match agent.action {
                    Action::Left(_) => Color::RED,
                    _ => Color::WHITE,
                }
            },
        };
    }
    color
}

fn text_block(font: &Font, agent: &Agent, state: &State, action: &Action) -> Rc<FormattedTextBlock> {
    let mut value_text = String::from("");
    if action.value() < 10.0 {
        value_text = format!("{:.2}", action.value());
    } else {
        value_text = format!("{:.0}", action.value());
    }

    font.layout_text(&value_text, 0.25 * CELL_SIZE as f32, TextOptions::new())
}