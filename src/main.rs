use std::{error::Error, io};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, CardFace},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);


    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                    }
                    KeyCode::Char('a') => {
                        app.current_screen = CurrentScreen::AddingDeck;
                        app.adding_deck = true;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Char('k') => {
                        if let Some(index) = app.selected_index {
                            if index > 0 {
                                app.selected_index = Some(index - 1);
                            }
                        }
                    }
                    KeyCode::Char('j') => {
                        if let Some(index) = app.selected_index {
                            if index < app.decks.len() - 1 {
                                app.selected_index = Some(index + 1);
                            }
                        } else {
                            app.selected_index = Some(0);
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(_index) = app.selected_index {
                            app.display_decks = false;
                            app.current_screen = CurrentScreen::ViewingDeck;
                        }
                    }
                    _ => {}
                },
                CurrentScreen::ViewingDeck => match key.code {
                    KeyCode::Char('q') => {
                        app.display_decks = true;
                        app.current_screen = CurrentScreen::Main;
                        app.selected_card_index = None;
                    }
                    KeyCode::Char('k') => {
                        if let Some(index) = app.selected_card_index {
                            if index > 0 {
                                app.selected_card_index = Some(index - 1);
                            }
                        }
                    }
                    KeyCode::Char('j') => {
                        if let Some(index) = app.selected_card_index {
                            if index < app.decks[app.selected_index.unwrap_or_default()].cards.len() - 1 {
                                app.selected_card_index = Some(index + 1);
                            }
                        } else {
                            app.selected_card_index = Some(0);
                        }
                    }
                    KeyCode::Char('a') => {
                        app.card_currently_editing = Some(CardFace::CardFront);
                        app.current_screen = CurrentScreen::EditingCard;
                    }
                    KeyCode::Char('s') => {
                        app.selected_card_index = None;
                        app.next_card_to_learn();
                        app.current_screen = CurrentScreen::LearningMode;
                    }
                    _ => {}
                },
                CurrentScreen::LearningMode => match key.code {
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if let Some(face_showing) = &app.face_showing {
                            match face_showing {
                                CardFace::CardFront => {
                                    app.face_showing = Some(CardFace::CardBack);
                                }
                                CardFace::CardBack => {
                                }
                            }
                        }
                    }
                    KeyCode::Char('h') => {
                        app.next_card_to_learn();
                    }
                    KeyCode::Char('k') => {
                        app.next_card_to_learn();
                    }
                    KeyCode::Char('j') => {
                        app.next_card_to_learn();
                    }
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::EditingCard;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::ViewingDeck;
                        app.selected_card_index = None;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::AddingDeck => match key.code {
                    KeyCode::Enter => {
                        if !app.name_input.is_empty() {
                            app.add_deck(app.name_input.clone());
                            app.name_input = String::new();
                            app.adding_deck = false;
                            app.current_screen = CurrentScreen::Main;
                        }
                    }
                    KeyCode::Backspace => {
                        app.name_input.pop();
                    }
                    KeyCode::Esc => {
                        app.adding_deck = false;
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char(value) => {
                        app.name_input.push(value);
                    }
                    _ => {}
                },
                CurrentScreen::EditingCard => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &app.card_currently_editing {
                            match editing {
                                CardFace::CardFront => {
                                    app.card_currently_editing = Some(CardFace::CardBack);
                                }
                                CardFace::CardBack => {
                                    app.add_card();
                                    app.front_input = String::new();
                                    app.back_input = String::new();
                                    app.current_screen =
                                        CurrentScreen::ViewingDeck;
                                    app.card_currently_editing = None;
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.card_currently_editing {
                            match editing {
                                CardFace::CardFront => {
                                    app.front_input.pop();
                                }
                                CardFace::CardBack => {
                                    app.back_input.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::ViewingDeck;
                        app.card_currently_editing = None;
                    }
                    KeyCode::Tab => {
                        app.toggle_card_currently_editing();
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.card_currently_editing {
                            match editing {
                                CardFace::CardFront => {
                                    app.front_input.push(value);
                                }
                                CardFace::CardBack => {
                                    app.back_input.push(value);
                                }
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}