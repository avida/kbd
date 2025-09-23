pub const MAX_ID: u8 = u8::MAX;
pub struct IdGenerator {
    current: u8,
    
}

impl IdGenerator {
    pub fn new() -> Self {
        IdGenerator { current: 0 }
    }
}

impl Iterator for IdGenerator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.current;
        self.current = self.current.wrapping_add(1);
        Some(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generator_starts_at_zero() {
        let mut gn = IdGenerator::new();
        assert_eq!(gn.next(), Some(0));
    }

    #[test]
    fn test_id_generator_increments() {
        let mut gn = IdGenerator::new();
        assert_eq!(gn.next(), Some(0));
        assert_eq!(gn.next(), Some(1));
        assert_eq!(gn.next(), Some(2));
    }

    #[test]
    fn test_id_generator_wraps_around() {
        let mut gn = IdGenerator { current: 255};
        assert_eq!(gn.next(), Some(255));
        assert_eq!(gn.next(), Some(0));
        assert_eq!(gn.next(), Some(1));
    }

    #[test]
    fn test_id_generator_multiple_iterations() {
        let mut gn = IdGenerator::new();
        let ids: Vec<u8> = gn.by_ref().take(5).collect();
        assert_eq!(ids, vec![0, 1, 2, 3, 4]);
    }
}
