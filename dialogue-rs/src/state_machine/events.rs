pub enum Event {
    Marker,
    Command,
}

pub struct Events {
    events: Vec<Event>,
}

impl Events {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop()
    }
}
