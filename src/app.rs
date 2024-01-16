use serde_json::Result;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use rand::Rng;

pub enum CurrentScreen {
    Main,
    AddingDeck,
    ViewingDeck,
    EditingCard,
    LearningMode,
    Exiting,
}

#[derive(Clone, Copy)]
pub enum CardFace {
    CardFront,
    CardBack,
}

pub enum Guess {
    Easy,
    Correct,
    Incorrect,
    None,
}

pub struct Card {
    pub front: String,
    pub back: String,
    pub last_guess: Guess, 
}

pub struct Deck { // one deck of cards with a name an a list of cards
    pub name: String,
    //declare a vector of cards
    pub cards: Vec<Card>,
    pub date_last_learned: DateTime<Utc>,
}

pub struct App {
    pub selected_index: Option<usize>, // the currently selected index of the list of decks.
    pub selected_card_index: Option<usize>, // the currently selected index of the list of cards.
    pub name_input: String, // the currently being edited deck name.
    pub front_input: String, // the currently being edited card front.
    pub back_input: String,
    pub decks: Vec<Deck>, // The different decks of cards
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub card_currently_editing: Option<CardFace>, // the optional state containing which of the card's front or back the user is editing. It is an option, because when the user is not directly editing a card, this will be set to `None`.
    pub adding_deck: bool, // the boolean state containing whether the user is adding a deck or not.
    pub display_decks: bool, // the boolean state containing whether the user is displaying a deck or not.
    pub card_currently_learning: Option<usize>,
    pub face_showing: Option<CardFace>,
}

impl App {
    pub fn new() -> App {
        App {
            selected_index: None,
            selected_card_index: None,
            name_input: String::new(),
            front_input: String::new(),
            back_input: String::new(),
            decks: Vec::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            card_currently_editing: None,
            adding_deck: false,
            display_decks: true,
            card_currently_learning: None,
            face_showing: None,
        }
    }

    pub fn add_deck(&mut self, name: String) {
        self.decks.push(Deck { name: name, cards: Vec::new(), date_last_learned: Utc::now() });
    }

    pub fn add_card(&mut self) {
        if let Some(index) = self.selected_index {
            self.decks[index].cards.push(Card { front: self.front_input.clone(), back: self.back_input.clone(), last_guess: Guess::None });
        }
    }

    pub fn next_card_to_learn(&mut self) {
        self.face_showing = Some(CardFace::CardFront);
    
        let mut rng = rand::thread_rng();
        let card_index = rng.gen_range(0..self.decks[self.selected_index.unwrap_or(0)].cards.len());
        self.card_currently_learning = Some(card_index);
    }

    pub fn toggle_card_currently_editing(&mut self) {
        if let Some(edit_mode) = &self.card_currently_editing {
            match edit_mode {
                CardFace::CardFront => {
                    self.card_currently_editing = Some(CardFace::CardBack)
                }
                CardFace::CardBack => {
                    self.card_currently_editing = Some(CardFace::CardFront)
                }
            };
        } else {
            self.card_currently_editing = Some(CardFace::CardFront);
        }
    }

    pub fn print_json(&self) -> Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}