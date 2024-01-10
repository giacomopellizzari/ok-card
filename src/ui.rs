use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    prelude::Alignment,
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing, CardCurrentlyEditing};

pub fn ui(f: &mut Frame, app: &App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "OK-CARD",
        Style::default().fg(Color::Green),
    ))
    .block(title_block).alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);
    let mut list_items = Vec::<ListItem>::new();
    
    for (index, deck) in app.decks.iter().enumerate() {
        let mut format = format!("- {} - {} -", index+1, deck.name);
        let mut style = Style::default().fg(Color::Blue);
        if Some(index) == app.selected_index {
            format = format!("-> {} - {} <-", index+1, deck.name);
            style = Style::default().fg(Color::White);
        }
        list_items.push(ListItem::new(Line::from(Span::styled(
            format,
            style,
        ))));
    }

    let deck_title = if app.decks.len() > 0 {
        Paragraph::new(Text::styled(
            "Decks:",
            Style::default().fg(Color::Green),
        ))
    } else {
        Paragraph::new(Text::styled(
            "Press (a) to add a deck",
            Style::default().fg(Color::Green),
        ))
    };

    let list_of_decks = List::new(list_items);
    if app.display_decks {
        f.render_widget(deck_title, chunks[1]);
        f.render_widget(list_of_decks, chunks[2]);
    }
    
    // Print all of the cards from a list
    let mut list_items = Vec::<ListItem>::new();

    // add if statement to check that decks is not empty
    if app.selected_index.is_some() {
        for (index, card) in app.decks[app.selected_index.unwrap_or_default()].cards.iter().enumerate() { //c
            let format = format!("{} - {}", index+1, card.front);
            let style = Style::default().fg(Color::Blue);
            list_items.push(ListItem::new(Line::from(Span::styled(
                format,
                style,
            ))));
        }

        let cards_paragraph_heading = if app.decks[app.selected_index.unwrap_or_default()].cards.len() > 0 {
            Paragraph::new(Text::styled(
                "Cards:",
                Style::default().fg(Color::Green),
            ))
        } else {
            Paragraph::new(Text::styled(
                "Press (a) to add a card",
                Style::default().fg(Color::Green),
            ))
        };

        let list_of_cards = List::new(list_items);
        if !app.display_decks {
            f.render_widget(cards_paragraph_heading, chunks[1]);
            f.render_widget(list_of_cards, chunks[2]);
        }
    }

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => {
                Span::styled("Showing Decks", Style::default().fg(Color::Green))
            }
            CurrentScreen::AddingDeck => {
                Span::styled("Adding Deck", Style::default().fg(Color::Red))
            }
            CurrentScreen::ViewingDeck => {
                Span::styled("Viewing Deck", Style::default().fg(Color::Blue))
            }
            CurrentScreen::EditingCard => {
                Span::styled("Editing Card", Style::default().fg(Color::LightBlue))
            }
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => {
                Span::styled("Exiting", Style::default().fg(Color::LightRed))
            }
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => Span::styled(
                        "Editing Json Key",
                        Style::default().fg(Color::Green),
                    ),
                    CurrentlyEditing::Value => Span::styled(
                        "Editing Json Value",
                        Style::default().fg(Color::LightGreen),
                    ),
                }
            } else {
                Span::styled(
                    "Not Editing Anything",
                    Style::default().fg(Color::DarkGray),
                )
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) quit / (a) to add deck",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::AddingDeck => Span::styled(
                "(ESC) cancel/ (ENTER) complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::ViewingDeck => Span::styled(
                "(q) to quit / (a) to add card",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::EditingCard => Span::styled(
                "(ESC) cancel/ (ENTER) complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) cancel/ (Tab) move/ (ENTER) complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) quit",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[3]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);

    if app.adding_deck {
        let popup_block = Block::default()
            .title("Enter name of new deck:")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, f.size());
        f.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1).constraints([
                Constraint::Percentage(70),
            ]).split(area);

        let name_block =
            Block::default().title("Name").borders(Borders::ALL);

        let _active_style =
            Style::default().bg(Color::LightYellow).fg(Color::Black);

        let name_text = Paragraph::new(app.name_input.clone()).block(name_block);
        f.render_widget(name_text, popup_chunks[0]);
    }

    if let Some(editing) = &app.card_currently_editing {
        let popup_block = Block::default()
            .title("Enter card information")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, f.size());
        f.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        let mut card_front_block = Block::default().title("Card Front").borders(Borders::ALL);
        let mut card_back_block =
            Block::default().title("Card back").borders(Borders::ALL);

        let active_style =
            Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CardCurrentlyEditing::CardFront => card_front_block = card_front_block.style(active_style),
            CardCurrentlyEditing::CardBack => {
                card_back_block = card_back_block.style(active_style)
            }
        };

        let card_front_text = Paragraph::new(app.front_input.clone()).block(card_front_block);
        f.render_widget(card_front_text, popup_chunks[0]);

        let card_back_text =
            Paragraph::new(app.back_input.clone()).block(card_back_block);
        f.render_widget(card_back_text, popup_chunks[1]);
    }

    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, f.size());
        f.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut value_block =
            Block::default().title("Value").borders(Borders::ALL);

        let active_style =
            Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditing::Key => key_block = key_block.style(active_style),
            CurrentlyEditing::Value => {
                value_block = value_block.style(active_style)
            }
        };

        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        f.render_widget(key_text, popup_chunks[0]);

        let value_text =
            Paragraph::new(app.value_input.clone()).block(value_block);
        f.render_widget(value_text, popup_chunks[1]);
    }

    if let CurrentScreen::Exiting = app.current_screen {
        f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, f.size());
        f.render_widget(exit_paragraph, area);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}