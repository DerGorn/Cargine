pub trait Event {}

pub struct EventBUS<T: Event> {
    listeners: Vec<Option<Box<dyn Fn(&T) + Send>>>,
}
impl<T: Event> EventBUS<T> {
    pub fn new() -> Self {
        EventBUS {
            listeners: Vec::new(),
        }
    }

    pub fn add_listener<F: Fn(&T) + Send + 'static>(&mut self, listener: F) -> usize {
        let index = self.listeners.len();
        self.listeners.push(Some(Box::new(listener)));
        index
    }

    pub fn remove_listener(&mut self, index: usize) {
        self.listeners[index] = None;
    }

    pub fn send(&self, event: &T) {
        for listener in &self.listeners {
            if let Some(listener) = listener {
                listener(event);
            }
        }
    }
}
