use std::{collections::HashMap, hash::Hash};

pub trait Event: Eq + Hash {}

pub struct EventBUS<T: Event> {
    listeners: HashMap<T, Vec<Option<Box<dyn Fn(&T, &Self)>>>>,
}
impl<T: Event> EventBUS<T> {
    pub fn new() -> Self {
        EventBUS {
            listeners: HashMap::new(),
        }
    }

    pub fn add_listener<F: Fn(&T, &Self) + 'static>(&mut self, event: T, listener: F) -> usize {
        let index = self.listeners.len();
        match self.listeners.get_mut(&event) {
            Some(v) => v.push(Some(Box::new(listener))),
            None => {
                self.listeners.insert(event, vec![Some(Box::new(listener))]);
            }
        };

        index
    }

    pub fn remove_listener(&mut self, event: T, index: usize) {
        self.listeners.get_mut(&event).unwrap()[index] = None;
    }

    pub fn send(&self, event: &T) {
        for listener in &self.listeners[event] {
            if let Some(listener) = listener {
                listener(event, self);
            }
        }
    }
}
