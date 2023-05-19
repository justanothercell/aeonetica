pub(crate) struct WorldGenerator {
    pub seed: u64
}

impl WorldGenerator {
    pub(crate) fn new(seed: u64) -> Self {
        Self { seed }
    }
}