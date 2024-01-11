use serde_json::Result;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub enum CurrentScreen {
    Main,
    Editing,
    AddingDeck,
    ViewingDeck,
    EditingCard,
    Exiting,
}

pub enum CardCurrentlyEditing {
    CardFront,
    CardBack,
}

pub enum CurrentlyEditing {
    Key,
    Value,
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
    pub key_input: String, // the currently being edited json key.
    pub value_input: String, // the currently being edited json value.
    pub name_input: String, // the currently being edited deck name.
    pub front_input: String, // the currently being edited card front.
    pub back_input: String,
    pub decks: Vec<Deck>, // The different decks of cards
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
    pub card_currently_editing: Option<CardCurrentlyEditing>, // the optional state containing which of the card's front or back the user is editing. It is an option, because when the user is not directly editing a card, this will be set to `None`.
    pub adding_deck: bool, // the boolean state containing whether the user is adding a deck or not.
    pub display_decks: bool, // the boolean state containing whether the user is displaying a deck or not.
}

impl App {
    pub fn new() -> App {
        App {
            selected_index: None,
            selected_card_index: None,
            key_input: String::new(),
            value_input: String::new(),
            name_input: String::new(),
            front_input: String::new(),
            back_input: String::new(),
            decks: Vec::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            card_currently_editing: None,
            adding_deck: false,
            display_decks: true,
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

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_card_currently_editing(&mut self) {
        if let Some(edit_mode) = &self.card_currently_editing {
            match edit_mode {
                CardCurrentlyEditing::CardFront => {
                    self.card_currently_editing = Some(CardCurrentlyEditing::CardBack)
                }
                CardCurrentlyEditing::CardBack => {
                    self.card_currently_editing = Some(CardCurrentlyEditing::CardFront)
                }
            };
        } else {
            self.card_currently_editing = Some(CardCurrentlyEditing::CardFront);
        }
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => {
                    self.currently_editing = Some(CurrentlyEditing::Value)
                }
                CurrentlyEditing::Value => {
                    self.currently_editing = Some(CurrentlyEditing::Key)
                }
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn print_json(&self) -> Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}