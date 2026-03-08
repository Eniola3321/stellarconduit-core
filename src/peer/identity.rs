#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PeerIdentity {
    pub id: u64,
}

impl PeerIdentity {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}
