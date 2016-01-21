pub enum Yaku {
    poetry,
    flowers,
    purple,
    butterflies,
    boar,
    moon,
    salamander,
    deer,
    gramps,
    none,
}


pub struct Card {
    month: i32,
    points: i32,
    yaku: Yaku,
}

impl Card {
    fn new_yaku(month: i32, points: i32, yaku: Yaku) -> Card {
        Card{
            month: month,
            points: points,
            yaku: yaku,
        }
    }

    fn new(month: i32, points: i32) -> Card {
        Card{
            month: month,
            points: points,
            yaku: Yaku::none,
        }
    }
}


pub struct Context {
    pub cards: Vec<Card>
}


fn fill_cards(cards: &mut Vec<Card>) {
    // January
    cards.push(Card::new(1, 1));
    cards.push(Card::new(1, 1));
    cards.push(Card::new_yaku(1, 5, Yaku::poetry));
    cards.push(Card::new(1, 20));
    // February
    cards.push(Card::new(2, 1));
    cards.push(Card::new(2, 1));
    cards.push(Card::new_yaku(2, 5, Yaku::poetry));
    cards.push(Card::new(2, 10));
    // March
    cards.push(Card::new(3, 1));
    cards.push(Card::new(3, 1));
    cards.push(Card::new_yaku(3, 5, Yaku::poetry));
    cards.push(Card::new_yaku(3, 20, Yaku::flowers));
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
    cards.push(Card::new_yaku(6, 5, Yaku::purple));
    cards.push(Card::new_yaku(6, 10, Yaku::butterflies));
    // July
    cards.push(Card::new(7, 1));
    cards.push(Card::new(7, 1));
    cards.push(Card::new(7, 5));
    cards.push(Card::new_yaku(7, 20, Yaku::boar));
    // August
    cards.push(Card::new(8, 1));
    cards.push(Card::new(8, 1));
    cards.push(Card::new(8, 10));
    cards.push(Card::new_yaku(8, 20, Yaku::moon));
    // September
    cards.push(Card::new(9, 1));
    cards.push(Card::new(9, 1));
    cards.push(Card::new_yaku(9, 5, Yaku::purple));
    cards.push(Card::new_yaku(9, 10, Yaku::salamander));
    // October
    cards.push(Card::new(10, 1));
    cards.push(Card::new(10, 1));
    cards.push(Card::new_yaku(10, 5, Yaku::purple));
    cards.push(Card::new_yaku(10, 10, Yaku::deer));
    // November
    cards.push(Card::new(11, 1));
    cards.push(Card::new(11, 5));
    cards.push(Card::new(11, 10));
    cards.push(Card::new_yaku(11, 20, Yaku::gramps));
    // December
    cards.push(Card::new(12, 1));
    cards.push(Card::new(12, 1));
    cards.push(Card::new(12, 1));
    cards.push(Card::new(12, 20));
}

pub fn initialize_context() -> Context {
    let mut cards: Vec<Card> = Vec::new();
    fill_cards(&mut cards);
    Context{
        cards: cards
    }
}
