use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque, binary_heap::PeekMut},
    hash::{BuildHasher, Hash, Hasher},
};

use fnv::FnvBuildHasher;
use rand::seq::SliceRandom;

fn main() {
    let mut byte_vec = b"Misssssssissippi".to_vec();
    byte_vec.dedup();
    assert_eq!(&byte_vec, b"Misisipi");

    let mut byte_vec = b"Misssssssissippi".to_vec();
    let mut seen = HashSet::new();
    byte_vec.retain(|r| seen.insert(*r));
    assert_eq!(&byte_vec, b"Misp");

    assert_eq!([[1, 2], [3, 4], [5, 6]].concat(), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(
        [[1, 2], [3, 4], [5, 6]].join(&0),
        vec![1, 2, 0, 3, 4, 0, 5, 6]
    );

    let v = vec![0, 1, 2, 3];
    let a = &v[0];
    let b = &v[1];
    let mid = v.len() / 2;
    let font_half = &v[..mid];
    let back_half = &v[mid..];

    // let mut v = vec![0, 1, 2, 3];
    // let a = &mut v[0];
    // let b = &mut v[1];
    // *a = 6;
    // *b = 7;

    assert_eq!([1, 2, 3, 4].starts_with(&[1, 2]), true);
    assert_eq!([1, 2, 3, 4].ends_with(&[3, 4]), true);

    let mut my_vec = vec![1, 2, 3, 4];
    my_vec.shuffle(&mut rand::rng());

    let v = VecDeque::from(vec![1, 2, 3, 4]);

    let mut heap = BinaryHeap::from(vec![1, 2, 3, 4]);
    if let Some(top) = heap.peek_mut() {
        if *top > 10 {
            PeekMut::pop(top);
        }
    }

    struct Student;
    impl Student {
        pub fn new() -> Self {
            Student
        }
    }
    let mut student_map = HashMap::new();
    student_map.insert("John".to_string(), Student::new());
    let name = "John";
    if !student_map.contains_key(name) {
        student_map.insert(name.to_string(), Student::new());
    }
    let _record = student_map.get_mut(name).unwrap();
    let _record = student_map
        .entry(name.to_string())
        .or_insert_with(Student::new);

    let mut vote_counts: HashMap<String, usize> = HashMap::new();
    let ballots = vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Alice".to_string(),
        "Charlie".to_string(),
        "Bob".to_string(),
    ];
    for name in ballots {
        let count = vote_counts.entry(name).or_insert(0);
        *count += 1;
    }

    let mut word_occurrence: HashMap<&str, HashSet<&str>> = HashMap::new();
    let files = vec!["file1.txt", "file2.txt", "file3.txt"];
    fn read_words(file: &str) -> Result<Vec<&str>, std::io::Error> {
        // Simulate reading words from a file
        let words = vec!["hello", "world", "hello", "rust"];
        Ok(words)
    }
    for file in files {
        for word in read_words(file).unwrap() {
            let set = word_occurrence.entry(word).or_insert_with(HashSet::new);
            set.insert(file);
        }
    }

    let mut word_frequency: HashMap<&str, u32> = HashMap::new();
    let text = "hello world hello rust";
    for c in text.split_whitespace() {
        word_frequency
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let s1 = "hello".to_string();
    let s2 = "hello".to_string();
    println!("{:p}", &s1 as &str);
    println!("{:p}", &s2 as &str);

    struct Artifact {
        id: u32,
        name: String,
        cultures: Vec<String>,
        date: String,
    }

    impl PartialEq for Artifact {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl Eq for Artifact {}

    impl Hash for Artifact {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }
    let mut collection = HashSet::<Artifact>::new();

    fn compute_hash<B, T>(builder: &B, value: &T) -> u64
    where
        B: BuildHasher,
        T: Hash,
    {
        let mut hasher = builder.build_hasher();
        value.hash(&mut hasher);
        hasher.finish()
    }

    pub type FnvHashMap<K, V> = HashMap<K, V, FnvBuildHasher>;
    pub type FnvHashSet<T> = HashSet<T, FnvBuildHasher>;
}
