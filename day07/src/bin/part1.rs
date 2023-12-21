use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let mut hands: Vec<_> = input.lines().map(parse_line).collect();

    hands.sort();
    let mut sum = 0;

    for (i, hand) in hands.iter().enumerate() {
        // println!("{}", hand);
        sum += hand.bid * (i as u64 + 1);
    }

    println!("{}", sum);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn parse(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Unknown card {}", c),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    ForOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: [Card; 5],
    type_: Type,
    bid: u64,
    cards_str: String, // for display
}

impl Hand {
    fn parse(cards_str: &str, bid_str: &str) -> Hand {
        let mut cards: [Card; 5] = [Card::Two; 5];
        assert!(cards_str.len() == 5);
        for (i, c) in cards_str.chars().enumerate() {
            cards[i] = Card::parse(c);
        }
        let type_ = get_type(&cards);
        Hand {
            cards: cards,
            type_: type_,
            bid: bid_str.parse().unwrap(),
            cards_str: cards_str.to_string(),
        }
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cards {} of type {:?} with bid {}",
            self.cards_str, self.type_, self.bid
        )
        // write!(f, "Cards {} with bid {}", self.cards, self.bid)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.type_ != other.type_ {
            self.type_.cmp(&other.type_)
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

fn get_type(cards: &[Card; 5]) -> Type {
    let mut card_counts = HashMap::new();
    for c in cards {
        card_counts.entry(c).or_insert(0);
        *card_counts.get_mut(c).unwrap() += 1;
    }
    let mut counts = card_counts.values().cloned().collect::<Vec<_>>();
    counts.sort();
    match counts.as_slice() {
        [1, 1, 1, 1, 1] => Type::HighCard,
        [1, 1, 1, 2] => Type::OnePair,
        [1, 2, 2] => Type::TwoPair,
        [1, 1, 3] => Type::ThreeOfAKind,
        [1, 4] => Type::ForOfAKind,
        [2, 3] => Type::FullHouse,
        [5] => Type::FiveOfAKind,
        _ => panic!("Unknown cards combination {:?}", card_counts.values()),
    }
}

fn parse_line(line: &str) -> Hand {
    let mut elems = line.split_whitespace();
    Hand::parse(elems.next().unwrap(), elems.next().unwrap())
}
