/// A representation of a connected player or device
#[derive(Default)]
pub struct Client {
    id: u128
}

impl Client {
    pub fn new(id: u128) -> Client {
        Client {
            id
        }
    }

    pub fn id(&self) -> u128 {
        self.id
    }

    pub fn is_authenticated(&self) -> bool {
        true
    }
}
