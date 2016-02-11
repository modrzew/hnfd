use rand::{thread_rng, Rng};

use mio;
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};

use models;
use messages;

pub struct Player {
    pub hand: Vec<u8>,
    taken: Vec<u8>,
    pub token: mio::Token,
}

impl Player {
    pub fn new(token: mio::Token) -> Player {
        Player {
            hand: Vec::new(),
            taken: Vec::new(),
            token: token
        }
    }
}

pub struct Game {
    table: Vec<u8>,
    deck: Vec<u8>,
    pub players: Vec<Player>,
    started: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            table: Vec::new(),
            deck: Vec::new(),
            players: Vec::new(),
            started: false,
        }
    }

    /**
     * Returns the other player
     */
    pub fn get_other(&self, token: mio::Token) -> mio::Token {
        if token == self.players[0].token {
            self.players[1].token
        } else {
            self.players[0].token
        }
    }

    /**
     * Who cares about docstrings, right?
     * It's not like Rust has PEP257 or something.
     *
     * Man, this sounds c00l right now, but I'm sure I'll regret not writing
     * docstring in 3 months.
     */
    pub fn add_player(&mut self, player: mio::Token) {
        self.players.push(Player::new(player));
    }

    /**
     * LOL RUST DOESN'T HAVE range(48) BUILTIN? WTF
     *
     * TODO: FIND BETTER WAY OF DOING IT
     */
    fn get_new_deck(&self) -> [u8; 48] {
        let mut deck: [u8; 48] = [0; 48];
        for i in 0..48 {
            deck[i] = i as u8;
        }
        thread_rng().shuffle(&mut deck);
        deck
    }

    /**
     * Resets game state and shuffles all necessary cards.
     */
    pub fn start(&mut self) {
        let deck = self.get_new_deck();
        // First 16 for players
        for i in 0..8 {
            self.players[0].hand.push(deck[i]);
        }
        for i in 8..16 {
            self.players[1].hand.push(deck[i]);
        }
        for i in 16..24 {
            self.table.push(deck[i]);
        }
        for i in 24..48 {
            self.deck.push(deck[i]);
        }
        self.started = true;
    }

    fn get_state_message(&self) -> messages::StateMessage {
        messages::StateMessage{
            my_hand: self.players[0].hand.to_vec(),
            my_taken: self.players[0].taken.to_vec(),
            his_hand: self.players[1].hand.to_vec(),
            his_taken: self.players[1].taken.to_vec(),
            table: self.table.to_vec(),
            deck_left: self.deck.len() as u8,
        }
    }

    pub fn get_start_message(&self, index: usize) -> (String) {
        let other: usize;
        if index == 0 {
            other = 1;
        } else {
            other = 0;
        }
        json::encode(&messages::StateMessage{
            my_hand: self.players[index].hand.to_vec(),
            my_taken: Vec::new(),
            his_hand: self.players[other].hand.to_vec(),
            his_taken: Vec::new(),
            table: self.table.to_vec(),
            deck_left: self.deck.len() as u8,
        }).unwrap()
    }

    /**
     * Handles single move in this game
     *
     * Returns messages that should go to player that did the move (first
     * value) and his opponent (second value).
     */
    // TODO: this method should really return Message instead of string!
    pub fn handle(&self, cards: &Vec<models::Card>) -> (String, String) {
        let (current, opponent);
        // Normal move
        current = json::encode(&messages::MoveMessage{ from: 1, to: 2});
        opponent = json::encode(&messages::MoveMessage{ from: 1, to: 2});
        (current.unwrap(), opponent.unwrap())
    }
}
