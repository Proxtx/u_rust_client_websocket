use enigo::Enigo;
use enigo::KeyboardControllable;

pub struct Simulate {
    enigo: Enigo,
}

impl Simulate {
    pub fn new() -> Self {
        Simulate {
            enigo: Enigo::new(),
        }
    }

    pub fn keys(&mut self, sequence: &str) {
        self.enigo.key_sequence_parse(sequence);
    }
}
