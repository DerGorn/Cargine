use std::hash::Hash;

use rand::{Rng, SeedableRng};

mod event_machine;
trait Card: Clone {}

#[cfg(test)]
mod tests {
    use self::event_machine::{Consumer, Event, EventState};

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

    enum BlackJackEvent {
        Init,
        StartRound,
        RequestDraw,
        Draw(PlayingCard),
    }
    impl Event<PlayingCard> for BlackJackEvent {}

    #[derive(PartialEq, Eq, Hash, Default)]
    enum BlackJackStates {
        #[default]
        Start,
        Init,
        StartRound,
        RequestDraw,
        Draw,
        End,
    }
    impl EventState<PlayingCard, BlackJackEvent> for BlackJackStates {}
    impl From<&BlackJackEvent> for BlackJackStates {
        fn from(event: &BlackJackEvent) -> Self {
            match event {
                BlackJackEvent::Init => BlackJackStates::Init,
                BlackJackEvent::StartRound => BlackJackStates::StartRound,
                BlackJackEvent::RequestDraw => BlackJackStates::RequestDraw,
                BlackJackEvent::Draw(_) => BlackJackStates::Draw,
            }
        }
    }

    struct Deck<C: Card> {
        cards: Vec<C>,
    }
    impl Deck<PlayingCard> {
        fn new() -> Deck<PlayingCard> {
            Deck { cards: vec![] }
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

        fn fill(&mut self) {
            self.cards = vec![
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
            ];
        }
    }
    impl Consumer<PlayingCard, BlackJackEvent, BlackJackStates> for Deck<PlayingCard> {
        fn possible_states(&self) -> Vec<BlackJackStates> {
            vec![
                BlackJackStates::Init,
                BlackJackStates::StartRound,
                BlackJackStates::RequestDraw,
            ]
        }
        fn handle_event(
            &mut self,
            event: &BlackJackEvent,
        ) -> (
            Vec<BlackJackEvent>,
            Vec<
                Box<
                    dyn Consumer<
                        PlayingCard,
                        BlackJackEvent,
                        BlackJackStates,
                        event_machine::DefaultPriority,
                    >,
                >,
            >,
        ) {
            match event {
                BlackJackEvent::Init => {
                    self.fill();
                    self.seeded_shuffle(&[0; 32]);
                    (vec![], vec![])
                }
                BlackJackEvent::StartRound => (
                    vec![BlackJackEvent::RequestDraw, BlackJackEvent::RequestDraw],
                    vec![],
                ),
                BlackJackEvent::RequestDraw => {
                    let card = match self.cards.pop() {
                        Some(card) => card,
                        None => {
                            self.fill();
                            self.seeded_shuffle(&[0; 32]);
                            self.cards.pop().unwrap()
                        }
                    };
                    (vec![BlackJackEvent::Draw(card)], vec![])
                }
                BlackJackEvent::Draw(_) => unreachable!("Deck should not receive Draw event"),
            }
        }
    }

    struct Blind {
        cards: Vec<PlayingCard>,
    }
    impl Consumer<PlayingCard, BlackJackEvent, BlackJackStates> for Blind {
        fn possible_states(&self) -> Vec<BlackJackStates> {
            vec![BlackJackStates::StartRound, BlackJackStates::Draw]
        }
        fn handle_event(
            &mut self,
            event: &BlackJackEvent,
        ) -> (
            Vec<BlackJackEvent>,
            Vec<
                Box<
                    dyn Consumer<
                        PlayingCard,
                        BlackJackEvent,
                        BlackJackStates,
                        event_machine::DefaultPriority,
                    >,
                >,
            >,
        ) {
            match event {
                BlackJackEvent::StartRound => {
                    self.cards.clear();
                    (vec![], vec![])
                }
                BlackJackEvent::Draw(card) => {
                    self.cards.push(card.clone());
                    println!("Blind: {:?}", self.cards);
                    (vec![], vec![])
                }
                BlackJackEvent::RequestDraw | BlackJackEvent::Init => {
                    unreachable!("Blind should not receive RequestDraw or Init events")
                }
            }
        }
    }

    #[test]
    fn black_jack() {
        let deck = Deck::new();
        let blind = Blind { cards: vec![] };
        let mut event_machine = event_machine::EventMachine::new(|state, manager| match state {
            BlackJackStates::Start => (Some(BlackJackStates::Init), vec![BlackJackEvent::Init]),
            BlackJackStates::Init => (
                Some(BlackJackStates::StartRound),
                vec![BlackJackEvent::StartRound],
            ),
            BlackJackStates::StartRound => (Some(BlackJackStates::RequestDraw), vec![]),
            BlackJackStates::RequestDraw => (Some(BlackJackStates::Draw), vec![]),
            BlackJackStates::Draw => (Some(BlackJackStates::End), vec![]),
            BlackJackStates::End => (None, vec![]),
        });
        event_machine.add_consumer(Box::new(deck));
        event_machine.add_consumer(Box::new(blind));

        event_machine.run();
        panic!("End of test");
    }
}
