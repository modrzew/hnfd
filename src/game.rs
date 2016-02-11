use mio;
use rustc_serialize::json;

use models;
use messages;

pub struct Player {
    hand: Vec<usize>,
    taken: Vec<usize>,
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
    table: Vec<usize>,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            table: Vec::new(),
            players: Vec::new(),
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
     * Handles single move in this game
     *
     * Returns messages that should go to player that did the move (first
     * value) and his opponent (second value).
     */
    pub fn handle(&self, cards: &Vec<models::Card>) -> (messages::MoveMessage, messages::MoveMessage) {
        (
            messages::MoveMessage{ from: 1, to: 2},
            messages::MoveMessage{ from: 1, to: 2},
        )
    }
}
