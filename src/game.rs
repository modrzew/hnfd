use mio;

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
    table: String,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            table: "".to_string(),
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
     * Handles single move in this game
     *
     * Returns messages that should go to player that did the move and his
     * opponent
     */
    pub fn handle<'a>(&self) -> (&'a str, &'a str) {
        (&"Czyki", &"bryki")
    }
}