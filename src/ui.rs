use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    prelude::Alignment,
    Frame,
};

use crate::app::{App, CurrentScreen, CardFace};

pub struct ColorScheme {
    pub title: Color,
    pub normal: Color,
    pub selected: Color,
    pub highlight: Color,
    pub warning: Color,
    pub selected_box: Color,
    pub selected_box_text: Color,
}

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
        
    //initialize ColorScheme
    let color_scheme = ColorScheme {
        title: Color::LightYellow, 
        normal: Color::LightBlue,
        selected: Color::White,
        highlight: Color::Yellow,
        warning: Color::Red,
        selected_box: Color::LightYellow,
        selected_box_text: Color::Black,
    };

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "OK-CARD",
        Style::default().fg(color_scheme.title).add_modifier(ratatui::style::Modifier::BOLD),
    ))
    .block(title_block).alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);
    let mut list_items = Vec::<ListItem>::new();
    
    for (index, deck) in app.decks.iter().enumerate() {
        let mut format = format!("- {} - {} -", index+1, deck.name);
        let mut style = Style::default().fg(color_scheme.normal);
        if Some(index) == app.selected_index {
            format = format!("-> {} - {} <-", index+1, deck.name);
            style = Style::default().fg(color_scheme.selected);
        }
        list_items.push(ListItem::new(Line::from(Span::styled(
            format,
            style,
        ))));
    }

    let deck_title = if app.decks.len() > 0 {
        Paragraph::new(Text::styled(
            "Decks:",
            Style::default().fg(color_scheme.title),
        ))
    } else {
        Paragraph::new(Text::styled(
            "Press (a) to add a deck",
            Style::default().fg(color_scheme.title),
        ))
    };

    let list_of_decks = List::new(list_items);
    if app.display_decks {
        f.render_widget(deck_title, chunks[1]);
        f.render_widget(list_of_decks, chunks[2]);
    }
    

    //show the cards in the selected deck
    if let CurrentScreen::ViewingDeck = app.current_screen {
        let mut list_items = Vec::<ListItem>::new();
        for (index, card) in app.decks[app.selected_index.unwrap_or_default()].cards.iter().enumerate() { //c
            let mut format = format!("{} - {}", index+1, card.front);
            let mut style = Style::default().fg(color_scheme.normal);
            if Some(index) == app.selected_card_index {
                format = format!("{} - {} <-", index+1, card.front);
                style = Style::default().fg(color_scheme.selected);
            }
            list_items.push(ListItem::new(Line::from(Span::styled(
                format,
                style,
            ))));
        }

        let cards_paragraph_heading = if app.decks[app.selected_index.unwrap_or_default()].cards.len() > 0 {
            let text_title_display = format!("Cards for deck {}", &app.decks[app.selected_index.unwrap_or_default()].name);
            Paragraph::new(Text::styled(
                text_title_display,
                Style::default().fg(color_scheme.title),
            ))
        } else {
            let text_title_display = format!("Press (a) to add a card to deck {}", &app.decks[app.selected_index.unwrap_or_default()].name);
            Paragraph::new(Text::styled(
                text_title_display,
                Style::default().fg(color_scheme.title),
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
                Span::styled("Showing Decks", Style::default().fg(color_scheme.title))
            }
            CurrentScreen::AddingDeck => {
                Span::styled("Adding Deck", Style::default().fg(color_scheme.title))
            }
            CurrentScreen::ViewingDeck => {
                Span::styled("Viewing Deck", Style::default().fg(color_scheme.title))
            }
            CurrentScreen::EditingCard => {
                Span::styled("Editing Card", Style::default().fg(color_scheme.title))
            }
            CurrentScreen::LearningMode => {
                Span::styled("Learning Mode", Style::default().fg(color_scheme.title))
            }
            CurrentScreen::Exiting => {
                Span::styled("Exiting", Style::default().fg(color_scheme.warning))
            }
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(color_scheme.selected)),
        // The final section of the text, with hints on what the user is editing
        {
            Span::styled("Editing", Style::default().fg(color_scheme.title))
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) quit / (a) add deck",
                Style::default().fg(color_scheme.title),
            ),
            CurrentScreen::AddingDeck => Span::styled(
                "(ESC) cancel/ (ENTER) complete",
                Style::default().fg(color_scheme.title),
            ),
            CurrentScreen::ViewingDeck => Span::styled(
                "(q) back/ (a) add card",
                Style::default().fg(color_scheme.title),
            ),
            CurrentScreen::EditingCard => Span::styled(
                "(ESC) cancel/ (ENTER) complete",
                Style::default().fg(color_scheme.title),
            ),
            CurrentScreen::LearningMode => Span::styled(
                "(q) back",
                Style::default().fg(color_scheme.title),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) quit",
                Style::default().fg(color_scheme.warning),
            ),
        }
    };

    //display the footer
    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[3]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);

    //display the adding of the deck
    if app.adding_deck {
        let popup_block = Block::default()
            .title("Enter name of new deck:")
            .borders(Borders::NONE)
            .style(Style::default().bg(color_scheme.selected_box_text).fg(color_scheme.selected_box));

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
            Style::default().bg(color_scheme.selected_box).fg(color_scheme.selected_box_text);

        let name_text = Paragraph::new(app.name_input.clone()).block(name_block);
        f.render_widget(name_text, popup_chunks[0]);
    }

    // display the cards being added/edited
    if let Some(editing) = &app.card_currently_editing {
        let popup_block = Block::default()
            .title("Enter card information")
            .borders(Borders::NONE)
            .style(Style::default().bg(color_scheme.selected_box_text).fg(color_scheme.selected_box));

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
            Style::default().bg(color_scheme.selected_box).fg(color_scheme.selected_box_text);

        match editing {
            CardFace::CardFront => card_front_block = card_front_block.style(active_style),
            CardFace::CardBack => {
                card_back_block = card_back_block.style(active_style)
            }
        };
        

        let card_front_text = Paragraph::new(app.front_input.clone()).block(card_front_block);
        f.render_widget(card_front_text, popup_chunks[0]);

        let card_back_text =
            Paragraph::new(app.back_input.clone()).block(card_back_block);
        f.render_widget(card_back_text, popup_chunks[1]);
    }
    
    //display the cards being learned
    if let CurrentScreen::LearningMode = app.current_screen {
        let learning_area_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(3), Constraint::Length(3)])
            .split(chunks[2]);

        let commands_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
            .split(learning_area_chunks[2]);

        let front_card_block = Block::default().borders(Borders::NONE);
        let back_card_block = Block::default().borders(Borders::ALL);

        let front_text = Text::styled(
            app.decks[app.selected_index.unwrap_or_default()].cards[app.card_currently_learning.unwrap_or_default()].front.clone(),
            Style::default().fg(color_scheme.normal).add_modifier(ratatui::style::Modifier::BOLD),
        );

        let back_text = Text::styled(
            app.decks[app.selected_index.unwrap_or_default()].cards[app.card_currently_learning.unwrap_or_default()].back.clone(),
            Style::default().fg(color_scheme.normal),
        );

        let card_name_paragraph = Paragraph::new(front_text)
            .block(front_card_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });

        let card_back_paragraph = Paragraph::new(back_text)
            .block(back_card_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });

        let command_incorrect_paragraph = Paragraph::new(Text::styled(
            "(h) incorrect",
            Style::default().fg(color_scheme.warning),
        )).alignment(Alignment::Center);
        let command_correct_paragraph = Paragraph::new(Text::styled(
            "(j) correct",
            Style::default().fg(color_scheme.title),
        )).alignment(Alignment::Center);
        let command_easy_paragraph = Paragraph::new(Text::styled(
            "(k) easy",
            Style::default().fg(Color::Green),
        )).alignment(Alignment::Center);
        let command_show_back_paragraph = Paragraph::new(Text::styled(
            "press (ENTER) to reveal the back of the card",
            Style::default().fg(color_scheme.title),
        )).alignment(Alignment::Center);

        f.render_widget(card_name_paragraph, learning_area_chunks[0]);
        
        if let Some(face_showing) = &app.face_showing {
            match face_showing {
                CardFace::CardFront => {
                    f.render_widget(command_show_back_paragraph, commands_chunks[1]);
                }
                CardFace::CardBack => {
                    f.render_widget(card_back_paragraph, learning_area_chunks[1]);
                    f.render_widget(command_incorrect_paragraph, commands_chunks[0]);
                    f.render_widget(command_correct_paragraph, commands_chunks[1]);
                    f.render_widget(command_easy_paragraph, commands_chunks[2]);
                }
            };
        }
    }

    if let CurrentScreen::Exiting = app.current_screen {
        f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(color_scheme.selected_box_text).fg(color_scheme.selected_box));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(color_scheme.warning),
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