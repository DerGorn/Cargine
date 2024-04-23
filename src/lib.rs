use std::hash::Hash;

use event_listener::{Event, EventBUS};
use rand::{Rng, SeedableRng};
mod event_listener;
trait Card: Clone + Default {}

trait CardContainer<T: Card> {
    fn cards(&self) -> &Vec<T>;
    fn cards_mut(&mut self) -> &mut Vec<T>;
    fn add(&mut self, card: T) {
        self.cards_mut().push(card);
    }
    fn draw(&mut self) -> Option<T> {
        self.cards_mut().pop()
    }
}

struct Deck<T: Card> {
    cards: Vec<T>,
}
impl<T: Card> CardContainer<T> for Deck<T> {
    fn cards(&self) -> &Vec<T> {
        &self.cards
    }
    fn cards_mut(&mut self) -> &mut Vec<T> {
        &mut self.cards
    }
}
impl<T: Card> Deck<T> {
    const fn new(cards: Vec<T>) -> Self {
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in (1..self.cards.len()).rev() {
            let j = rng.gen_range(0..=i);
            self.cards.swap(i, j);
        }
    }

    fn seeded_shuffle(&mut self, seed: &[u8; 32]) {
        let mut rng = rand::rngs::StdRng::from_seed(*seed);
        for i in (1..self.cards.len()).rev() {
            let j = rng.gen_range(0..=i);
            self.cards.swap(i, j);
        }
    }

    fn add_bottom(&mut self, card: T) {
        self.cards.insert(0, card);
    }
}

struct DiscardPile<T: Card> {
    cards: Vec<T>,
}
impl<T: Card> CardContainer<T> for DiscardPile<T> {
    fn cards(&self) -> &Vec<T> {
        &self.cards
    }
    fn cards_mut(&mut self) -> &mut Vec<T> {
        &mut self.cards
    }
}
impl<T: Card> DiscardPile<T> {
    const fn new() -> Self {
        DiscardPile { cards: Vec::new() }
    }
}

struct Hand<T: Card> {
    cards: Vec<T>,
}
impl<T: Card> CardContainer<T> for Hand<T> {
    fn cards(&self) -> &Vec<T> {
        &self.cards
    }
    fn cards_mut(&mut self) -> &mut Vec<T> {
        &mut self.cards
    }
}
impl<T: Card> Hand<T> {
    const fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    fn draw_from_deck<D: CardContainer<T>>(&mut self, deck: &mut D) -> Option<&T> {
        if let Some(card) = deck.draw() {
            self.add(card);
            self.cards.last()
        } else {
            None
        }
    }
}

struct PlayArea<T: Card> {
    cards: Vec<T>,
}
impl<T: Card> CardContainer<T> for PlayArea<T> {
    fn cards(&self) -> &Vec<T> {
        &self.cards
    }
    fn cards_mut(&mut self) -> &mut Vec<T> {
        &mut self.cards
    }
}
impl<T: Card> PlayArea<T> {
    const fn new() -> Self {
        PlayArea { cards: Vec::new() }
    }

    fn play(&mut self, card: T, bus: &EventBUS<CardTimings<T>>) {
        {
            let c = CardTimings::Play(card.clone());
            bus.send(&c);
        }
        self.add(card)
    }
}

enum CardTimings<T: Card> {
    Play(T),
    Draw(T),
}
impl<T: Card> PartialEq for CardTimings<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Play(_) => match other {
                Self::Play(_) => true,
                _ => false,
            },
            Self::Draw(_) => match other {
                Self::Draw(_) => true,
                _ => false,
            },
        }
    }
}
impl<T: Card> Eq for CardTimings<T> {}
impl<T: Card> Hash for CardTimings<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Play(_) => "Play".hash(state),
            Self::Draw(_) => "Draw".hash(state),
        }
    }
}
impl<T: Card> Event for CardTimings<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    enum Suit {
        Clubs,
        Diamonds,
        Hearts,
        Spades,
    }
    #[derive(Clone, Debug)]
    enum Rank {
        Ace,
        Two,
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
    }
    #[derive(Clone, Debug)]
    struct PlayingCard {
        suit: Suit,
        rank: Rank,
    }
    impl Card for PlayingCard {}

    fn new_deck() -> Deck<PlayingCard> {
        Deck::new(vec![
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::Ace,
            },
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::Two,
            },
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::Three,
            },
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::Four,
            },
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::Five,
            },
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::Six,
            },
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::Seven,
            },
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::Eight,
            },
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::Nine,
            },
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::Ten,
            },
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::Jack,
            },
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::Queen,
            },
            PlayingCard {
                suit: Suit::Clubs,
                rank: Rank::King,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::Ace,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::Two,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::Three,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::Four,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::Five,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::Six,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::Seven,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::Eight,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::Nine,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::Ten,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::Jack,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::Queen,
            },
            PlayingCard {
                suit: Suit::Diamonds,
                rank: Rank::King,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::Ace,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::Two,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::Three,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::Four,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::Five,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::Six,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::Seven,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::Eight,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::Nine,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::Ten,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::Jack,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::Queen,
            },
            PlayingCard {
                suit: Suit::Hearts,
                rank: Rank::King,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::Ace,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::Two,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::Three,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::Four,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::Five,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::Six,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::Seven,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::Eight,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::Nine,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::Jack,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::Queen,
            },
            PlayingCard {
                suit: Suit::Spades,
                rank: Rank::King,
            },
        ])
    }
    impl Default for PlayingCard {
        fn default() -> Self {
            Self {
                suit: Suit::Clubs,
                rank: Rank::Ace,
            }
        }
    }

    #[test]
    fn black_jack() {
        let initial_blind_size = 2;
        let seed = [0; 32];

        let mut bus: EventBUS<CardTimings<PlayingCard>> = EventBUS::new();
        bus.add_listener(CardTimings::Play(PlayingCard::default()), |timing_event, _| {
            match timing_event {
                CardTimings::Play(card) => {
                    println!("{:?}", card)
                }
                _ => unreachable!("SHOULD ONLY RECEIVE PLAY EVENTS")
            }
        });

        let mut deck = new_deck();
        deck.seeded_shuffle(&seed);

        let mut blind = PlayArea::new();
        bus.add_listener(CardTimings::Draw(PlayingCard::default()), |timing_event, bus| {
            match timing_event {
                CardTimings::Draw(card) => {
                    blind.play(card.clone(), bus)
                }
                _ => unreachable!()
            }
        });
        let mut i = 0;
        while i < initial_blind_size {
            match deck.draw() {
                Some(card) => {
                    bus.send(&CardTimings::Draw(card));
                    i += 1;
                }
                None => {
                    deck = new_deck();
                    deck.seeded_shuffle(&seed);
                }
            }
        }
        panic!("")
    }
}
