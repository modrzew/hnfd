use std::collections::HashMap;

use models::{Yaku, Card};

fn get_hand_score(cards: &Vec<Card>, month: usize) -> usize {
    let mut point_counters: HashMap<usize, usize> = HashMap::new();
    let mut yaku_counters: HashMap<Yaku, usize> = HashMap::new();

    let mut points = 0;
    for card in cards {
        *point_counters.entry(card.points).or_insert(0) += 1;
        *yaku_counters.entry(card.yaku.clone()).or_insert(0) += 1;
        if card.month == month {
            *yaku_counters.entry(Yaku::Month).or_insert(0) += 1;
        }
    }
    // Point cards
    for (k, v) in point_counters {
        if k == 1 && v >= 10 {
            points += 1 + v - 10;
        }
        if k == 5 && v >= 5 {
            points += 1 + v - 5;
        }
        if k == 10 && v >= 5 {
            points += 1 + v - 5;
        }
        if k == 20 && v >= 3 {
            let no_gramps = !yaku_counters.contains_key(&Yaku::Gramps);
            if v == 3 && no_gramps {
                points += 5;
            } else if v == 4 {
                points += 7;
                if no_gramps {
                    points += 1;
                }
            } else if v == 5 {
                points += 10;
            }
        }
    }
    // Yaku
    if let Some(value) = yaku_counters.get(&Yaku::Purple) {
        if *value == 3 {
            points += 5;
        }
    }
    if let Some(value) = yaku_counters.get(&Yaku::Poetry) {
        if *value == 3 {
            points += 5;
        }
    }
    if let Some(value) = yaku_counters.get(&Yaku::Inoshikachou) {
        if *value == 3 {
            points += 5;
        }
    }
    if let Some(v1) = yaku_counters.get(&Yaku::Salamander) {
        if let Some(v2) = yaku_counters.get(&Yaku::Moon) {
            if *v1 == 1 && *v2 == 1 {
                points += 5;
            }
        }
    }
    if let Some(v1) = yaku_counters.get(&Yaku::Salamander) {
        if let Some(v2) = yaku_counters.get(&Yaku::Flowers) {
            if *v1 == 1 && *v2 == 1 {
                points += 5;
            }
        }
    }
    // Month
    if let Some(value) = yaku_counters.get(&Yaku::Month) {
        if *value == 4 {
            points += 4;
        }
    }
    points
}

#[cfg(test)]
mod get_hand_score_tests {
    use super::get_hand_score;
    use models::{Yaku, Card};

    /**
     * Produces vector full of cards with given point values
     */
    fn produce_cards(values: Vec<usize>) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::new();
        for value in values {
            result.push(Card::new(1, value));
        }
        result
    }

    fn produce_cards_yaku(values: Vec<Yaku>) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::new();
        for value in values {
            result.push(Card::new_yaku(1, 1, value));
        }
        result
    }

    #[test]
    fn one_point_cards() {
        let input_and_expected = vec![
            (produce_cards(vec![1, 1, 1]), 0),
            (produce_cards(vec![1, 1, 1, 1, 1, 1, 1]), 0),
            (produce_cards(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1),
            (produce_cards(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 2),
            (produce_cards(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 3),
            (produce_cards(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 4),
        ];
        for (i, e) in input_and_expected {
            assert_eq!(get_hand_score(&i, 12), e);
        }
    }

    #[test]
    fn five_point_cards() {
        let input_and_expected = vec![
            (produce_cards(vec![5, 5, 5]), 0),
            (produce_cards(vec![5, 5, 5, 5]), 0),
            (produce_cards(vec![5, 5, 5, 5, 5]), 1),
            (produce_cards(vec![5, 5, 5, 5, 5, 5]), 2),
            (produce_cards(vec![5, 5, 5, 5, 5, 5, 5]), 3),
            (produce_cards(vec![5, 5, 5, 5, 5, 5, 5, 5]), 4),
        ];
        for (i, e) in input_and_expected {
            assert_eq!(get_hand_score(&i, 12), e);
        }
    }

    #[test]
    fn ten_point_cards() {
        let input_and_expected = vec![
            (produce_cards(vec![10, 10, 10]), 0),
            (produce_cards(vec![10, 10, 10, 10]), 0),
            (produce_cards(vec![10, 10, 10, 10, 10]), 1),
            (produce_cards(vec![10, 10, 10, 10, 10, 10]), 2),
            (produce_cards(vec![10, 10, 10, 10, 10, 10, 10]), 3),
            (produce_cards(vec![10, 10, 10, 10, 10, 10, 10, 10]), 4),
        ];
        for (i, e) in input_and_expected {
            assert_eq!(get_hand_score(&i, 12), e);
        }
    }

    #[test]
    fn twenty_point_cards() {
        let mut input_and_expected = vec![
            (produce_cards(vec![20]), 0),
            (produce_cards(vec![20, 20]), 0),
            (produce_cards(vec![20, 20, 20]), 0), // with gramps - see below
            (produce_cards(vec![20, 20, 20]), 5), // without gramps
            (produce_cards(vec![20, 20, 20, 20]), 7), // with gramps - see below
            (produce_cards(vec![20, 20, 20, 20]), 8), // without gramps
            (produce_cards(vec![20, 20, 20, 20, 20]), 10),
        ];
        // Set gramps
        input_and_expected[2].0[2] = Card::new_yaku(1, 20, Yaku::Gramps);
        input_and_expected[4].0[3] = Card::new_yaku(1, 20, Yaku::Gramps);
        for (i, e) in input_and_expected {
            assert_eq!(get_hand_score(&i, 12), e);
        }
    }

    #[test]
    fn poetry() {
        let input_and_expected = vec![
            (produce_cards_yaku(vec![Yaku::Poetry]), 0),
            (produce_cards_yaku(vec![Yaku::Poetry, Yaku::Poetry]), 0),
            (produce_cards_yaku(vec![Yaku::Poetry, Yaku::Poetry, Yaku::Poetry]), 5),
        ];
        for (i, e) in input_and_expected {
            assert_eq!(get_hand_score(&i, 12), e);
        }
    }


    #[test]
    fn purple() {
        let input_and_expected = vec![
            (produce_cards_yaku(vec![Yaku::Purple]), 0),
            (produce_cards_yaku(vec![Yaku::Purple, Yaku::Purple]), 0),
            (produce_cards_yaku(vec![Yaku::Purple, Yaku::Purple, Yaku::Purple]), 5),
        ];
        for (i, e) in input_and_expected {
            assert_eq!(get_hand_score(&i, 12), e);
        }
    }

    #[test]
    fn inoshikachou() {
        let input_and_expected = vec![
            (produce_cards_yaku(vec![Yaku::Inoshikachou]), 0),
            (produce_cards_yaku(vec![Yaku::Inoshikachou, Yaku::Inoshikachou]), 0),
            (produce_cards_yaku(vec![Yaku::Inoshikachou, Yaku::Inoshikachou, Yaku::Inoshikachou]), 5),
        ];
        for (i, e) in input_and_expected {
            assert_eq!(get_hand_score(&i, 12), e);
        }
    }

    #[test]
    fn hanami_tsukimi_de_ippai() {
        let input_and_expected = vec![
            (produce_cards_yaku(vec![Yaku::Moon]), 0),
            (produce_cards_yaku(vec![Yaku::Moon, Yaku::Salamander]), 5),
            (produce_cards_yaku(vec![Yaku::Flowers]), 0),
            (produce_cards_yaku(vec![Yaku::Flowers, Yaku::Salamander]), 5),
            (produce_cards_yaku(vec![Yaku::Salamander]), 0),
            (produce_cards_yaku(vec![Yaku::Moon, Yaku::Flowers, Yaku::Salamander]), 10),
        ];
        for (i, e) in input_and_expected {
            assert_eq!(get_hand_score(&i, 12), e);
        }
    }

    #[test]
    fn month() {
        let input_and_expected = vec![
            (vec![Card::new(1, 1)], 0),
            (vec![Card::new(1, 1), Card::new(1, 1)], 0),
            (vec![Card::new(1, 1), Card::new(1, 1), Card::new(1, 5)], 0),
            (vec![Card::new(1, 1), Card::new(1, 1), Card::new(1, 5), Card::new(1, 20)], 4),
        ];
        for (i, e) in input_and_expected {
            assert_eq!(get_hand_score(&i, 1), e);
        }
    }

    #[test]
    fn combination_zero_points() {
        let mut input = produce_cards(vec![1, 1, 1, 1, 1, 1, 1, 10, 10, 20, 20]);
        input.append(&mut vec![
            Card::new_yaku(1, 5, Yaku::Poetry),
            Card::new_yaku(1, 5, Yaku::Poetry),
            Card::new_yaku(1, 5, Yaku::Purple),
            Card::new_yaku(1, 5, Yaku::Purple),
            Card::new_yaku(1, 10, Yaku::Inoshikachou),
            Card::new_yaku(1, 10, Yaku::Inoshikachou),
        ]);
        assert_eq!(get_hand_score(&input, 12), 0);
    }

    #[test]
    fn combination_lot_of_points() {
        let mut input = produce_cards(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 5, 10, 10, 10, 10, 20]);
        input.append(&mut vec![
            Card::new_yaku(1, 5, Yaku::Poetry),
            Card::new_yaku(1, 5, Yaku::Poetry),
            Card::new_yaku(1, 5, Yaku::Poetry),
            Card::new_yaku(1, 5, Yaku::Purple),
            Card::new_yaku(1, 5, Yaku::Purple),
            Card::new_yaku(1, 5, Yaku::Purple),
            Card::new_yaku(1, 10, Yaku::Inoshikachou),
            Card::new_yaku(1, 10, Yaku::Inoshikachou),
            Card::new_yaku(1, 10, Yaku::Inoshikachou),
            Card::new_yaku(1, 10, Yaku::Salamander),
            Card::new_yaku(1, 10, Yaku::None),
            Card::new(1, 20),
            Card::new_yaku(1, 20, Yaku::Gramps),
            Card::new_yaku(1, 20, Yaku::Moon),
            Card::new_yaku(1, 20, Yaku::Flowers),
        ]);
        assert_eq!(get_hand_score(&input, 12), 47);
    }

    #[test]
    fn combination_lot_of_points_no_yaku() {
        let input = produce_cards(vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,  // 3
            5, 5, 5, 5, 5, 5, 5, 5,  // 4
            10, 10, 10, 10, 10, 10, 10, 10, 10, // 5
            20, 20, 20, 20, 20  // 10
        ]);
        assert_eq!(get_hand_score(&input, 12), 22);
    }

    #[test]
    fn combination_ribbons() {
        let input = vec![
            Card::new_yaku(1, 5, Yaku::Poetry),
            Card::new_yaku(1, 5, Yaku::Poetry),
            Card::new_yaku(1, 5, Yaku::Poetry),
            Card::new_yaku(1, 5, Yaku::Purple),
            Card::new_yaku(1, 5, Yaku::Purple),
            Card::new_yaku(1, 5, Yaku::Purple),
        ];
        assert_eq!(get_hand_score(&input, 12), 12);
    }
}
