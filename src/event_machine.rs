use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use crate::Card;

pub trait Event<T: Card> {}
pub trait EventState<C: Card, E: Event<C>>:
    for<'a> From<&'a E> + PartialEq + Eq + Hash + Default
{
}

pub trait Consumer<C: Card, E: Event<C>, S: EventState<C, E>, P: ConsumerPriority = DefaultPriority>
{
    fn prioritize(&self, event: &E) -> P {
        Default::default()
    }
    fn handle_event(&mut self, event: &E) -> (Vec<E>, Vec<Box<dyn Consumer<C, E, S, P>>>);
    fn possible_states(&self) -> Vec<S>;
}
pub trait ConsumerPriority: Default + PartialEq + PartialOrd + Eq + Ord {}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct DefaultPriority {}
impl Default for DefaultPriority {
    fn default() -> Self {
        DefaultPriority {}
    }
}
impl ConsumerPriority for DefaultPriority {}

pub struct EventMachine<
    C: Card,
    E: Event<C>,
    S: EventState<C, E>,
    P: ConsumerPriority = DefaultPriority,
> {
    phantom_card: PhantomData<C>,
    phantom_priority: PhantomData<P>,
    state: S,
    event_queue: HashMap<S, Vec<E>>,
    consumers: Vec<Box<dyn Consumer<C, E, S, P>>>,
    event_consumers: HashMap<S, Vec<usize>>,
    transition: Box<dyn Fn(&S, &Self) -> (Option<S>, Vec<E>)>,
}
impl<C: Card, E: Event<C>, S: EventState<C, E>, P: ConsumerPriority> EventMachine<C, E, S, P> {
    pub fn new(transition: impl Fn(&S, &Self) -> (Option<S>, Vec<E>) + 'static) -> Self {
        EventMachine {
            phantom_card: PhantomData,
            phantom_priority: PhantomData,
            state: Default::default(),
            event_queue: HashMap::new(),
            consumers: Vec::new(),
            event_consumers: HashMap::new(),
            transition: Box::new(transition),
        }
    }

    pub fn add_consumer(&mut self, consumer: Box<dyn Consumer<C, E, S, P>>) {
        let index = self.consumers.len();
        let states = consumer.possible_states();
        self.consumers.push(consumer);
        for state in states {
            match self.event_consumers.get_mut(&state) {
                Some(v) => v.push(index),
                None => {
                    self.event_consumers.insert(state, vec![index]);
                }
            }
        }
    }

    fn enqueue(&mut self, event: E) {
        let state = S::from(&event);
        match self.event_queue.get_mut(&state) {
            Some(v) => v.push(event),
            None => {
                self.event_queue.insert(state, vec![event]);
            }
        }
    }

    fn prioritize_consumers(&self, event: &E) -> Vec<usize> {
        let mut consumers = match self.event_consumers.get(&S::from(event)) {
            Some(v) => v.clone(),
            None => return Vec::new(),
        };
        consumers.sort_by(|a, b| {
            self.consumers[*a]
                .prioritize(event)
                .cmp(&self.consumers[*b].prioritize(event))
        });
        consumers
    }

    fn dequeue(&mut self) -> Option<E> {
        match self.event_queue.get_mut(&self.state) {
            Some(v) => v.pop(),
            None => None,
        }
    }

    pub fn run(&mut self) {
        loop {
            let event = self.dequeue();
            match event {
                Some(event) => {
                    let consumers = self.prioritize_consumers(&event);
                    for consumer in consumers {
                        let consumer = self.consumers.get_mut(consumer).unwrap();
                        let (new_events, new_consumers) = consumer.handle_event(&event);
                        for new_consumer in new_consumers {
                            self.add_consumer(new_consumer);
                        }
                        for new_event in new_events {
                            self.enqueue(new_event);
                        }
                    }
                }
                None => {
                    let (new_state, new_events) = (self.transition)(&self.state, &self);
                    match  new_state {
                        Some(state) => self.state = state,
                        None => break,
                    };
                    for new_event in new_events {
                        self.enqueue(new_event);
                    }
                }
            }
        }
    }
}

