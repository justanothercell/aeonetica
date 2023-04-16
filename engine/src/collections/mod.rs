pub mod ordered_map;
pub use ordered_map::OrderedMap;

#[cfg(test)]
mod benches {
    use test::{bench, Bencher, black_box};

    use std::collections::BTreeMap;
    use crate::collections::OrderedMap;
    use crate::Id;

    const ITEM_LEN: usize = 10000;

    #[bench]
    fn ordered_map_insert(b: &mut Bencher) {
        let mut map = OrderedMap::new();
        let keys: Vec<_> = (0..ITEM_LEN).map(|_| Id::new().into_u64()).collect();
        b.iter(||{
            keys.iter().enumerate().for_each(|(i, key)| {
                black_box({
                    map.insert(*key, i)
                });
            });
        });
    }

    #[bench]
    fn ordered_map_get(b: &mut Bencher) {
        let mut map = OrderedMap::new();
        let keys: Vec<_> = (0..ITEM_LEN).map(|_| Id::new().into_u64()).collect();
        for (i, key) in keys.iter().enumerate() {
            map.insert(*key, i);
        }
        b.iter(||{
            let _ = keys.iter().filter_map(|key| {
                black_box({
                    map.get(key)
                })
            }).sum::<usize>();
        });
    }

    #[bench]
    fn ordered_map_nth(b: &mut Bencher) {
        let mut map = OrderedMap::new();
        let keys: Vec<_> = (0..ITEM_LEN).map(|_| Id::new().into_u64()).collect();
        for (i, key) in keys.iter().enumerate() {
            map.insert(*key, i);
        }
        b.iter(||{
            let _ = (0..ITEM_LEN).filter_map(|i| {
                black_box({
                    map.nth(i)
                })
            }).sum::<usize>();
        });
    }

    #[bench]
    fn ordered_map_iter_in_order(b: &mut Bencher) {
        let mut map = OrderedMap::new();
        let keys: Vec<_> = (0..ITEM_LEN).map(|_| Id::new().into_u64()).collect();
        for (i, key) in keys.iter().enumerate() {
            map.insert(*key, i);
        }
        b.iter(|| {
            let _ = black_box({
                map.iter().map(|(k, i)| {
                    *i
                }).sum::<usize>()
            });
        });
    }

    #[bench]
    fn btree_map_insert(b: &mut Bencher) {
        let mut map = BTreeMap::new();
        let keys: Vec<_> = (0..ITEM_LEN).map(|_| Id::new().into_u64()).collect();
        b.iter(||{
            keys.iter().enumerate().for_each(|(i, key)| {
                black_box({
                    map.insert(*key, i)
                });
            });
        });
    }

    #[bench]
    fn btree_map_get(b: &mut Bencher) {
        let mut map = BTreeMap::new();
        let keys: Vec<_> = (0..ITEM_LEN).map(|_| Id::new().into_u64()).collect();
        for (i, key) in keys.iter().enumerate() {
            map.insert(*key, i);
        }
        b.iter(||{
            let _ = keys.iter().filter_map(|key| {
                black_box({
                    map.get(key)
                })
            }).sum::<usize>();
        });
    }

    #[bench]
    fn btree_map_nth(b: &mut Bencher) {
        let mut map = BTreeMap::new();
        let keys: Vec<_> = (0..ITEM_LEN).map(|_| Id::new().into_u64()).collect();
        for (i, key) in keys.iter().enumerate() {
            map.insert(*key, i);
        }
        b.iter(||{
            let _ = (0..ITEM_LEN).filter_map(|i| {
                black_box({
                    map.values().nth(i)
                })
            }).sum::<usize>();
        });
    }

    #[bench]
    fn btree_map_iter_in_order(b: &mut Bencher) {
        let mut map = BTreeMap::new();
        let keys: Vec<_> = (0..ITEM_LEN).map(|_| Id::new().into_u64()).collect();
        for (i, key) in keys.iter().enumerate() {
            map.insert(*key, i);
        }
        b.iter(|| {
            let _ = black_box({
                map.iter().map(|(_k, i)| {
                    *i
                }).sum::<usize>()
            });
        });
    }
}