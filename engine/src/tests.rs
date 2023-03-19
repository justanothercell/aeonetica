use std::collections::{BTreeMap, HashMap};
use test::Bencher;
use crate::Id;
use crate::util::id_map::{IdMap, IdMap2};

#[bench]
fn bench_hashmap(b: &mut Bencher) {
    b.iter(|| {
        let mut map = HashMap::<Id, i32>::new();
        let mut keys = vec![];
        let n = test::black_box(100_000);
        (0..n).for_each(|i| {
            let k = Id::new();
            map.insert(k, i);
            keys.push(k);
        });
        assert_eq!(map.len(), keys.len());
        keys.iter().for_each(|k| {
            let _ = map.get(k);
        })
    });
}

#[bench]
fn bench_idmap(b: &mut Bencher) {
    b.iter(|| {
        let mut map = IdMap::<i32>::default();
        let mut keys = vec![];
        let n = test::black_box(100_000);
        (0..n).for_each(|i| {
            let k = Id::new();
            map.insert(k, i);
            keys.push(k);
        });
        assert_eq!(map.len(), keys.len());
        keys.iter().for_each(|k| {
            let _ = map.get(k);
        })
    });
}

#[bench]
fn bench_btree(b: &mut Bencher) {
    b.iter(|| {
        let mut map = BTreeMap::<u64, i32>::default();
        let mut keys = vec![];
        let n = test::black_box(100_000);
        (0..n).for_each(|i| {
            let k = u64::from_le_bytes(Id::new().into_bytes());
            map.insert(k, i);
            keys.push(k);
        });
        assert_eq!(map.len(), keys.len());
        keys.iter().for_each(|k| {
            let _ = map.get(k);
        })
    });
}