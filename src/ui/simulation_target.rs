use std::{
    io::{self, Error},
    time::Duration,
};

use crossterm::{
    event::{self, poll, Event, KeyCode},
    terminal,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Terminal,
};

use crate::techniques::Technique;

use super::{verify_directory_structure, Simulations, UIRenderState, UIState};

pub fn determine_simulation() -> Result<(bool, Option<Simulations>), Error> {
    // Determine if regions is empty
    let backend = CrosstermBackend::new(io::stdout());
    terminal::enable_raw_mode()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let mut current_state = if verify_directory_structure() {
        UIRenderState::SimulationType
    } else {
        UIRenderState::DirectoryStructure
    };
    let mut state = UIState::new();
    let mut quit = false;
    let mut exit = false;
    loop {
        terminal.draw(|f| {
            let base = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            f.render_widget(base, f.size());
            match current_state {
                UIRenderState::DirectoryStructure => {
                    state.no_yes.1 = UIRenderState::DirectoryStructure;
                    let sections = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [Constraint::Percentage(90), Constraint::Percentage(10)].as_ref(),
                        )
                        .split(f.size());
                    let top = Paragraph::new(vec![
                        Spans::from("The regions directory is currently empty, please place"),
                        Spans::from(".mca files in the directory before continuing."),
                    ]);
                    f.render_widget(top, sections[0]);
                    let bottom =
                        Paragraph::new(Span::styled("Continue", Style::default().fg(Color::Green)));
                    f.render_widget(bottom, sections[1]);
                }
                UIRenderState::SimulationType => {
                    state.no_yes.1 = UIRenderState::SimulationType;
                    let sections = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(f.size());
                    let top = Paragraph::new(Span::from(
                        "Please select simulation type. (Use arrow keys and enter)",
                    ));
                    f.render_widget(top, sections[0]);
                    let items: Vec<ListItem> = state
                        .sim_type
                        .items
                        .iter()
                        .map(|i| ListItem::new(i.as_str()))
                        .collect();
                    let list = List::new(items).highlight_style(Style::default().fg(Color::Cyan));
                    f.render_stateful_widget(list, sections[1], &mut state.sim_type.state);
                }
                UIRenderState::TechniqueSelect => {
                    state.no_yes.1 = UIRenderState::TechniqueSelect;
                    let sections = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(f.size());
                    let top = Paragraph::new(Span::from(
                        "Please select a technique. (Use arrow keys and enter)",
                    ));
                    f.render_widget(top, sections[0]);
                    let items: Vec<ListItem> = state
                        .technique
                        .items
                        .iter()
                        .map(|i| ListItem::new(i.as_str()))
                        .collect();
                    let list = List::new(items).highlight_style(Style::default().fg(Color::Cyan));
                    f.render_stateful_widget(list, sections[1], &mut state.technique.state);
                }
                UIRenderState::TechniquesSelect => {
                    state.no_yes.1 = UIRenderState::TechniquesSelect;
                    state.error.1 = UIRenderState::TechniquesSelect;
                    state.no_yes.1 = UIRenderState::TechniqueSelect;
                    let sections = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(f.size());
                    let top = Paragraph::new(Span::from(
                        "Please select techniques. (Use arrow keys and enter)",
                    ));
                    f.render_widget(top, sections[0]);
                    let mut techs = Technique::iterable();
                    techs.push(String::from("Done"));
                    let mut items = Vec::new();
                    for i in 0..techs.len() {
                        if i == state.techniques_current {
                            items.push(
                                ListItem::new(techs[i].clone())
                                    .style(Style::default().fg(Color::Green)),
                            );
                        } else if state.techniques.contains(&i) {
                            items.push(
                                ListItem::new(techs[i].clone())
                                    .style(Style::default().fg(Color::Cyan)),
                            );
                        } else {
                            items.push(ListItem::new(techs[i].clone()));
                        }
                    }
                    let list = List::new(items);
                    f.render_widget(list, sections[1]);
                }
                UIRenderState::ThreadCount => {
                    state.no_yes.1 = UIRenderState::ThreadCount;
                    state.error.1 = UIRenderState::ThreadCount;
                    let sections = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(f.size());
                    let top = Paragraph::new(Span::from(
                        "Please enter a thread count. (Enter to submit)",
                    ));
                    f.render_widget(top, sections[0]);
                    let text = Paragraph::new(format!("Threads: {}", state.threads));
                    f.render_widget(text, sections[1]);
                }
                UIRenderState::YLevel => {
                    state.no_yes.1 = UIRenderState::YLevel;
                    state.error.1 = UIRenderState::YLevel;
                    let sections = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(f.size());
                    let top =
                        Paragraph::new(Span::from("Please enter a y level. (Enter to submit)"));
                    f.render_widget(top, sections[0]);
                    let text = Paragraph::new(format!("Y: {}", state.y_level));
                    f.render_widget(text, sections[1]);
                }
                UIRenderState::YRange => {
                    state.no_yes.1 = UIRenderState::YLevel;
                    state.error.1 = UIRenderState::YLevel;
                    let sections = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(f.size());
                    let top = Paragraph::new(Span::from(
                        "Please enter a min and max y level. (Enter to submit)",
                    ));
                    f.render_widget(top, sections[0]);
                    let text = Paragraph::new(vec![
                        Spans::from(Span::styled(
                            format!("min: {}", state.min),
                            if !state.second_range {
                                Style::default().fg(Color::Cyan)
                            } else {
                                Style::default()
                            },
                        )),
                        Spans::from(Span::styled(
                            format!("max: {}", state.max),
                            if state.second_range {
                                Style::default().fg(Color::Cyan)
                            } else {
                                Style::default()
                            },
                        )),
                    ]);
                    f.render_widget(text, sections[1]);
                }
                UIRenderState::RegionSelect => {
                    state.no_yes.1 = UIRenderState::RegionSelect;
                    let sections = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(f.size());
                    let top = Paragraph::new(Spans::from(
                        "Please select a file. (Enter to submit, the list scrolls!)",
                    ));
                    f.render_widget(top, sections[0]);
                    let items: Vec<ListItem> = state
                        .files
                        .items
                        .iter()
                        .map(|i| ListItem::new(i.as_str()))
                        .collect();
                    let list = List::new(items).highlight_style(Style::default().fg(Color::Cyan));
                    f.render_stateful_widget(list, sections[1], &mut state.files.state);
                }
                UIRenderState::Simulate => exit = true,
                UIRenderState::Quit => {
                    let sections = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(f.size());
                    let top = Paragraph::new(vec![Spans::from("Exit? (Use arrow keys and enter)")]);
                    f.render_widget(top, sections[0]);
                    let items: Vec<ListItem> = state
                        .no_yes
                        .0
                        .items
                        .iter()
                        .map(|i| ListItem::new(i.as_str()))
                        .collect();
                    let list = List::new(items).highlight_style(Style::default().fg(Color::Cyan));
                    f.render_stateful_widget(list, sections[1], &mut state.no_yes.0.state)
                }
                UIRenderState::Error => {
                    state.no_yes.1 = UIRenderState::Error;
                    let sections = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [Constraint::Percentage(90), Constraint::Percentage(10)].as_ref(),
                        )
                        .split(f.size());
                    let top = Paragraph::new(vec![
                        Spans::from("Error:"),
                        Spans::from(state.error.0.clone()),
                    ]);
                    f.render_widget(top, sections[0]);
                    let bottom =
                        Paragraph::new(Span::styled("Continue", Style::default().fg(Color::Green)));
                    f.render_widget(bottom, sections[1]);
                }
            }
        })?;

        if poll(Duration::from_millis(1_000))? {
            let event = event::read()?;
            match event {
                Event::Key(key) => match key.code {
                    KeyCode::Up => match current_state {
                        UIRenderState::SimulationType => state.sim_type.previous(),
                        UIRenderState::TechniqueSelect => state.technique.previous(),
                        UIRenderState::TechniquesSelect => {
                            if state.techniques_current > 0 {
                                state.techniques_current -= 1;
                            }
                        }
                        UIRenderState::YRange => state.second_range = !state.second_range,
                        UIRenderState::RegionSelect => state.files.previous(),
                        UIRenderState::Quit => state.no_yes.0.previous(),
                        _ => {}
                    },
                    KeyCode::Down => match current_state {
                        UIRenderState::SimulationType => state.sim_type.next(),
                        UIRenderState::TechniqueSelect => state.technique.next(),
                        UIRenderState::TechniquesSelect => {
                            if state.techniques_current <= Technique::iterable().len() - 1 {
                                state.techniques_current += 1;
                            }
                        }
                        UIRenderState::YRange => state.second_range = !state.second_range,
                        UIRenderState::RegionSelect => state.files.next(),
                        UIRenderState::Quit => state.no_yes.0.next(),
                        _ => {}
                    },
                    KeyCode::Enter => match current_state {
                        UIRenderState::DirectoryStructure => {
                            current_state = UIRenderState::SimulationType
                        }
                        UIRenderState::SimulationType => {
                            // state.sim_type.state.selected().unwrap(); This is the current usize that relates to the selected state string
                            match state.sim_type.items[state.sim_type.state.selected().unwrap()]
                                .as_str()
                            {
                                "Single" => current_state = UIRenderState::TechniqueSelect,
                                "Range" => current_state = UIRenderState::TechniqueSelect,
                                "Techniques" => current_state = UIRenderState::TechniquesSelect,
                                "Parameters" => current_state = UIRenderState::TechniqueSelect,
                                "Chunk" => current_state = UIRenderState::YRange,
                                "Quit" => current_state = UIRenderState::Quit,
                                _ => unreachable!("There was a string error"),
                            }
                        }
                        UIRenderState::TechniqueSelect => {
                            match state.sim_type.items[state.sim_type.state.selected().unwrap()]
                                .as_str()
                            {
                                "Single" => current_state = UIRenderState::YLevel,
                                "Range" => current_state = UIRenderState::YRange,
                                "Parameters" => current_state = UIRenderState::YRange,
                                "Quit" => {
                                    current_state = UIRenderState::Quit;
                                }
                                _ => unreachable!("There was a string error"),
                            }
                        }
                        UIRenderState::TechniquesSelect => {
                            let i = state.techniques_current;
                            if i == Technique::iterable().len() {
                                match state.sim_type.items[state.sim_type.state.selected().unwrap()]
                                    .as_str()
                                {
                                    "Techniques" => current_state = UIRenderState::YRange,
                                    "Parameters" => current_state = UIRenderState::YRange,
                                    "Quit" => {
                                        current_state = UIRenderState::Quit;
                                    }
                                    _ => unreachable!("There was a string error"),
                                }
                            } else {
                                let pos = state.techniques.iter().position(|x| x == &i);
                                match pos {
                                    Some(loc) => {
                                        state.techniques.remove(loc);
                                    }
                                    None => {
                                        state.techniques.push(i);
                                    }
                                }
                            }
                        }
                        UIRenderState::ThreadCount => match state.threads.parse::<u32>() {
                            Ok(y) => {
                                if y < 1 {
                                    state.error.0 = String::from("Number should be greater than 0");
                                    state.error.1 = UIRenderState::ThreadCount;
                                    current_state = UIRenderState::Error;
                                } else {
                                    match state.sim_type.items
                                        [state.sim_type.state.selected().unwrap()]
                                    .as_str()
                                    {
                                        "Techniques" => current_state = UIRenderState::Simulate,
                                        "Parameters" => current_state = UIRenderState::Simulate,
                                        "Chunk" => current_state = UIRenderState::Simulate,
                                        "Quit" => {
                                            current_state = UIRenderState::Quit;
                                        }
                                        _ => unreachable!("There was a string error"),
                                    }
                                }
                            }
                            Err(_) => {
                                state.error.0 = String::from("Failed to parse input");
                                state.error.1 = UIRenderState::ThreadCount;
                                current_state = UIRenderState::Error;
                            }
                        },
                        UIRenderState::YLevel => match state.y_level.parse::<i32>() {
                            Ok(_) => {
                                match state.sim_type.items[state.sim_type.state.selected().unwrap()]
                                    .as_str()
                                {
                                    "Single" => current_state = UIRenderState::RegionSelect,
                                    "Quit" => {
                                        current_state = UIRenderState::Quit;
                                    }
                                    _ => unreachable!("There was a string error"),
                                }
                            }
                            Err(_) => {
                                state.error.0 = String::from("Failed to parse input");
                                state.error.1 = UIRenderState::YLevel;
                                current_state = UIRenderState::Error;
                            }
                        },
                        UIRenderState::YRange => match state.min.parse::<i32>() {
                            Ok(_) => match state.max.parse::<i32>() {
                                Ok(_) => {
                                    match state.sim_type.items
                                        [state.sim_type.state.selected().unwrap()]
                                    .as_str()
                                    {
                                        "Range" => current_state = UIRenderState::RegionSelect,
                                        "Techniques" => current_state = UIRenderState::ThreadCount,
                                        "Parameters" => current_state = UIRenderState::ThreadCount,
                                        "Chunk" => current_state = UIRenderState::ThreadCount,
                                        "Quit" => {
                                            current_state = UIRenderState::Quit;
                                        }
                                        _ => unreachable!("There was a string error"),
                                    }
                                }
                                Err(_) => {
                                    state.error.0 = String::from("Failed to parse input on max");
                                    state.error.1 = UIRenderState::YRange;
                                    current_state = UIRenderState::Error;
                                }
                            },
                            Err(_) => {
                                state.error.0 = String::from("Failed to parse input on min");
                                state.error.1 = UIRenderState::YRange;
                                current_state = UIRenderState::Error;
                            }
                        },
                        UIRenderState::RegionSelect => {
                            match state.sim_type.items[state.sim_type.state.selected().unwrap()]
                                .as_str()
                            {
                                "Single" => current_state = UIRenderState::Simulate,
                                "Range" => current_state = UIRenderState::Simulate,
                                "Quit" => {
                                    current_state = UIRenderState::Quit;
                                }
                                _ => unreachable!("There was a string error"),
                            }
                        }
                        UIRenderState::Quit => {
                            match state.no_yes.0.items[state.no_yes.0.state.selected().unwrap()]
                                .as_str()
                            {
                                "yes" => quit = true,
                                "no" => {
                                    current_state = state.no_yes.1;
                                }
                                _ => {}
                            }
                        }
                        UIRenderState::Error => current_state = state.error.1,
                        _ => {}
                    },
                    KeyCode::Char(c) => match c {
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' => {
                            match current_state {
                                UIRenderState::YLevel => state.y_level.push(c),
                                UIRenderState::ThreadCount => state.threads.push(c),
                                UIRenderState::YRange => match state.second_range {
                                    false => state.min.push(c),
                                    true => state.max.push(c),
                                },
                                _ => {}
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Backspace => match current_state {
                        UIRenderState::ThreadCount => {
                            if state.threads.len() > 0 {
                                state.threads =
                                    state.threads[0..state.threads.len() - 1].to_string();
                            }
                        }
                        UIRenderState::YLevel => {
                            if state.y_level.len() > 0 {
                                state.y_level =
                                    state.y_level[0..state.y_level.len() - 1].to_string();
                            }
                        }
                        UIRenderState::YRange => match state.second_range {
                            false => {
                                if state.min.len() > 0 {
                                    state.min = state.min[0..state.min.len() - 1].to_string();
                                }
                            }
                            true => {
                                if state.max.len() > 0 {
                                    state.max = state.max[0..state.max.len() - 1].to_string();
                                }
                            }
                        },
                        _ => {}
                    },
                    KeyCode::Esc => current_state = UIRenderState::Quit,
                    _ => {}
                },
                _ => {}
            }
        }
        if quit {
            terminal.clear()?;
            terminal.set_cursor(0, 0)?;
            return Ok((false, None));
        }
        if exit {
            terminal.clear().unwrap();
            terminal.set_cursor(0, 0).unwrap();
            // Get the selected sim type
            let sim = match state.sim_type.items[state.sim_type.state.selected().unwrap()].as_str()
            {
                "Single" => Simulations::Single(
                    Technique::from_string(
                        state.technique.items[state.technique.state.selected().unwrap()].clone(),
                    ),
                    state.files.items[state.files.state.selected().unwrap()].clone(),
                    state.y_level.parse::<i32>().unwrap(),
                ),
                "Range" => Simulations::Range(
                    Technique::from_string(
                        state.technique.items[state.technique.state.selected().unwrap()].clone(),
                    ),
                    state.files.items[state.files.state.selected().unwrap()].clone(),
                    state.min.parse::<i32>().unwrap(),
                    state.max.parse::<i32>().unwrap(),
                ),
                "Techniques" => Simulations::Techniques(
                    state
                        .techniques
                        .iter()
                        .map(|f| Technique::from_string(Technique::iterable().to_vec()[*f].clone()))
                        .collect::<Vec<Technique>>(),
                    state.min.parse::<i32>().unwrap(),
                    state.max.parse::<i32>().unwrap(),
                    state.threads.parse::<u32>().unwrap(),
                ),
                "Parameters" => Simulations::TechniqueParameters(
                    state
                        .techniques
                        .iter()
                        .map(|f| Technique::from_string(Technique::iterable().to_vec()[*f].clone()))
                        .collect::<Vec<Technique>>(),
                    state.min.parse::<i32>().unwrap(),
                    state.max.parse::<i32>().unwrap(),
                    state.threads.parse::<u32>().unwrap(),
                ),
                "Chunk" => Simulations::Chunks(
                    state.min.parse::<i32>().unwrap(),
                    state.max.parse::<i32>().unwrap(),
                    state.threads.parse::<u32>().unwrap(),
                ),
                _ => unreachable!("There are a set number of paths"),
            };
            return Ok((true, Some(sim)));
        }
    }
}
