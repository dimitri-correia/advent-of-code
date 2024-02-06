use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
enum PokerHand {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn value(&self) -> u8 {
        use Card::*;
        match self {
            A => 14,
            K => 13,
            Q => 12,
            J => 11,
            T => 10,
            Nine => 9,
            Eight => 8,
            Seven => 7,
            Six => 6,
            Five => 5,
            Four => 4,
            Three => 3,
            Two => 2,
        }
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!(),
        }
    }
}

impl PokerHand {
    fn compare_cards(card1: &Card, card2: &Card) -> std::cmp::Ordering {
        card2.value().cmp(&card1.value())
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        // If hands have the same type, we compare cards
        if let (PokerHand::HighCard(card1), PokerHand::HighCard(card2))
        | (PokerHand::OnePair(card1), PokerHand::OnePair(card2))
        | (PokerHand::TwoPair(card1), PokerHand::TwoPair(card2))
        | (PokerHand::ThreeOfAKind(card1), PokerHand::ThreeOfAKind(card2))
        | (PokerHand::FullHouse(card1), PokerHand::FullHouse(card2))
        | (PokerHand::FourOfAKind(card1), PokerHand::FourOfAKind(card2))
        | (PokerHand::FiveOfAKind(card1), PokerHand::FiveOfAKind(card2)) = (self, other)
        {
            // Compare cards in each hand
            for (c1, c2) in card1.iter().zip(card2.iter()) {
                let card_ordering = PokerHand::compare_cards(c2, c1);
                if card_ordering != Ordering::Equal {
                    return card_ordering;
                }
            }
        }

        // Else, we compare the type of hands
        let type_ordering = match (self, other) {
            (PokerHand::HighCard(_), _) => Ordering::Less,
            (_, PokerHand::HighCard(_)) => Ordering::Greater,
            (PokerHand::OnePair(_), _) => Ordering::Less,
            (_, PokerHand::OnePair(_)) => Ordering::Greater,
            (PokerHand::TwoPair(_), _) => Ordering::Less,
            (_, PokerHand::TwoPair(_)) => Ordering::Greater,
            (PokerHand::ThreeOfAKind(_), _) => Ordering::Less,
            (_, PokerHand::ThreeOfAKind(_)) => Ordering::Greater,
            (PokerHand::FullHouse(_), _) => Ordering::Less,
            (_, PokerHand::FullHouse(_)) => Ordering::Greater,
            (PokerHand::FourOfAKind(_), _) => Ordering::Less,
            (_, PokerHand::FourOfAKind(_)) => Ordering::Greater,
            (PokerHand::FiveOfAKind(_), _) => Ordering::Less,
            //(_, PokerHand::FiveOfAKind(_)) => Ordering::Greater,
            //_ => Ordering::Equal,
        };

        if type_ordering != Ordering::Equal {
            return type_ordering;
        }

        Ordering::Equal
    }
}

fn part_1(input: &str) -> String {
    get_total_winnings(input).to_string()
}

fn get_total_winnings(input: &str) -> usize {
    let mut hands: Vec<(PokerHand, usize)> = vec![];

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let hand = get_kind_hand(parts.next().unwrap().to_string());
        let bid = parts.next().unwrap().parse::<usize>().unwrap();
        hands.push((hand, bid));
    }

    hands.sort_by(|a, b| a.0.cmp(&b.0));

    let mut res = 0;

    for (idx, (_, bid)) in hands.iter().enumerate() {
        let idx = idx + 1;
        res += idx * bid;
    }

    res
}

fn get_kind_hand(hand_string: String) -> PokerHand {
    let mut char_count = HashMap::new();

    for c in hand_string.chars() {
        let counter = char_count.entry(c).or_insert(0);
        *counter += 1;
    }

    let mut ordered_char_count: Vec<_> = char_count.iter().collect();
    ordered_char_count.sort_by_key(|&(_, count)| -count);

    let cards: Vec<Card> = hand_string.chars().map(Card::from).collect();

    if ordered_char_count[0].1 == &5 {
        return PokerHand::FiveOfAKind(cards);
    }
    if ordered_char_count[0].1 == &4 {
        return PokerHand::FourOfAKind(cards);
    }
    if ordered_char_count[0].1 == &3 {
        return if ordered_char_count[1].1 == &2 {
            PokerHand::FullHouse(cards)
        } else {
            PokerHand::ThreeOfAKind(cards)
        };
    }
    if ordered_char_count[0].1 == &2 {
        return if ordered_char_count[1].1 == &2 {
            PokerHand::TwoPair(cards)
        } else {
            PokerHand::OnePair(cards)
        };
    }

    PokerHand::HighCard(cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("250254244", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_1(input);
        assert_eq!("6440", r);
    }
}
