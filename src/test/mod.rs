use crate::types::{BitSet, SetInt, StdHashSet};

mod bitset_tests {
    use super::BitSet;
    use crate::types::SetInt;

    #[test]
    fn test_insert_remove() {
        let mut set = BitSet::new();
        set.insert(5);
        assert!(set.contains(5));
        assert!(set.remove(5));
        assert!(!set.contains(5));
    }

    #[test]
    fn test_clear() {
        let mut set = BitSet::new();
        set.insert(10);
        set.insert(20);
        set.clear();
        assert!(!set.contains(10));
        assert!(!set.contains(20));
    }

    #[test]
    fn test_contains_not_present() {
        let mut set = BitSet::new();
        set.insert(5);
        assert!(!set.contains(10));
    }
}

mod stdhashset_tests {
    use super::StdHashSet;
    use crate::types::SetInt;

    #[test]
    fn test_insert_remove() {
        let mut set = StdHashSet::new();
        set.insert(5);
        assert!(set.contains(5));
        assert!(set.remove(5));
        assert!(!set.contains(5));
    }

    #[test]
    fn test_clear() {
        let mut set = StdHashSet::new();
        set.insert(10);
        set.insert(20);
        set.clear();
        assert!(!set.contains(10));
        assert!(!set.contains(20));
    }

    #[test]
    fn test_contains_not_present() {
        let mut set = StdHashSet::new();
        set.insert(5);
        assert!(!set.contains(10));
    }
}
