trait Event {}

struct EventListener<T: Event> {
    listeners: Vec<Option<Box<dyn Fn(&T) + Send>>>,
}
impl<T: Event> EventListener<T> {
    fn new() -> Self {
        EventListener {
            listeners: Vec::new(),
        }
    }

    fn add_listener<F: Fn(&T) + Send + 'static>(&mut self, listener: F) -> usize {
        let index = self.listeners.len();
        self.listeners.push(Some(Box::new(listener)));
        index
    }

    fn remove_listener(&mut self, index: usize) {
        self.listeners[index] = None;
    }

    fn fire(&self, event: &T) {
        for listener in &self.listeners {
            if let Some(listener) = listener {
                listener(event);
            }
        }
    }
}
