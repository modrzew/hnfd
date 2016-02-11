use std::fmt;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Yaku {
    Poetry,
    Flowers,
    Purple,
    Inoshikachou,
    Moon,
    Salamander,
    Gramps,
    Month,
    None,
}

impl fmt::Display for Yaku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            &Yaku::Poetry => "Poetry",
            &Yaku::Flowers => "Flowers",
            &Yaku::Purple => "Purple",
            &Yaku::Inoshikachou => "Inoshikachou",
            &Yaku::Moon => "Moon",
            &Yaku::Salamander => "Salamander",
            &Yaku::Gramps => "Gramps",
            &Yaku::Month => "Month",
            &Yaku::None => "None",
        };
        write!(f, "{}", value)
    }
}

pub struct Card {
    pub month: usize,
    pub points: usize,
    pub yaku: Yaku,
}

impl Card {
    pub fn new_yaku(month: usize, points: usize, yaku: Yaku) -> Card {
        Card{
            month: month,
            points: points,
            yaku: yaku,
        }
    }

    pub fn new(month: usize, points: usize) -> Card {
        Card{
            month: month,
            points: points,
            yaku: Yaku::None,
        }
    }

    pub fn get_info(&self) -> String {
        format!("M: {}, P: {}, Y: {}", self.month, self.points, self.yaku)
    }
}

pub fn get_cards() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    // January
    cards.push(Card::new(1, 1));
    cards.push(Card::new(1, 1));
    cards.push(Card::new_yaku(1, 5, Yaku::Poetry));
    cards.push(Card::new(1, 20));
    // February
    cards.push(Card::new(2, 1));
    cards.push(Card::new(2, 1));
    cards.push(Card::new_yaku(2, 5, Yaku::Poetry));
    cards.push(Card::new(2, 10));
    // March
    cards.push(Card::new(3, 1));
    cards.push(Card::new(3, 1));
    cards.push(Card::new_yaku(3, 5, Yaku::Poetry));
    cards.push(Card::new_yaku(3, 20, Yaku::Flowers));
    // April
    cards.push(Card::new(4, 1));
    cards.push(Card::new(4, 1));
    cards.push(Card::new(4, 5));
    cards.push(Card::new(4, 10));
    // May
    cards.push(Card::new(5, 1));
    cards.push(Card::new(5, 1));
    cards.push(Card::new(5, 5));
    cards.push(Card::new(5, 10));
    // June
    cards.push(Card::new(6, 1));
    cards.push(Card::new(6, 1));
    cards.push(Card::new_yaku(6, 5, Yaku::Purple));
    cards.push(Card::new_yaku(6, 10, Yaku::Inoshikachou));
    // July
    cards.push(Card::new(7, 1));
    cards.push(Card::new(7, 1));
    cards.push(Card::new(7, 5));
    cards.push(Card::new_yaku(7, 20, Yaku::Inoshikachou));
    // August
    cards.push(Card::new(8, 1));
    cards.push(Card::new(8, 1));
    cards.push(Card::new(8, 10));
    cards.push(Card::new_yaku(8, 20, Yaku::Moon));
    // September
    cards.push(Card::new(9, 1));
    cards.push(Card::new(9, 1));
    cards.push(Card::new_yaku(9, 5, Yaku::Purple));
    cards.push(Card::new_yaku(9, 10, Yaku::Salamander));
    // October
    cards.push(Card::new(10, 1));
    cards.push(Card::new(10, 1));
    cards.push(Card::new_yaku(10, 5, Yaku::Purple));
    cards.push(Card::new_yaku(10, 10, Yaku::Inoshikachou));
    // November
    cards.push(Card::new(11, 1));
    cards.push(Card::new(11, 5));
    cards.push(Card::new(11, 10));
    cards.push(Card::new_yaku(11, 20, Yaku::Gramps));
    // December
    cards.push(Card::new(12, 1));
    cards.push(Card::new(12, 1));
    cards.push(Card::new(12, 1));
    cards.push(Card::new(12, 20));
    cards
}
