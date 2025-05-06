// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// trait IntoIterator
// where
//     Self::IntoIter: Iterator<Item = Self::Item>,
// {
//     type Item;
//     type IntoIter: Iterator;
//     fn into_iter(self) -> Self::IntoIter;
// }

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ffi::OsStr,
    fmt::{Debug, format},
    iter::{Peekable, from_fn, once, repeat, successors},
    path::Path,
    str::FromStr,
};

use num::Complex;
use rand::random;

fn dump<T, U>(t: T)
where
    T: IntoIterator<Item = U>,
    U: Debug,
{
    for u in t {
        println!("{:?}", u);
    }
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let zero = Complex { re: 0.0, im: 0.0 };
    successors(Some(zero), |&z| Some(z * z + c))
        .take(limit)
        .enumerate()
        .find(|(_i, z)| z.norm_sqr() > 4.0)
        .map(|(i, _z)| i)
}

fn fibonacci() -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    std::iter::from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}

fn to_uppercase(string: String) -> String {
    string.chars().flat_map(char::to_uppercase).collect()
}

fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
where
    I: Iterator<Item = char>,
{
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n * 10 + r.to_digit(10).unwrap();
            }
            _ => return n,
        }
        tokens.next();
    }
}

struct Flaky(bool);
impl Iterator for Flaky {
    type Item = &'static str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 {
            self.0 = false;
            Some("totally the last item")
        } else {
            self.0 = true;
            None
        }
    }
}

fn main() {
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];
    for element in &v {
        println!("{}", element);
    }

    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }

    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));

    let path = Path::new("src/main.rs");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("src")));

    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky With Diamonds".to_string());
    favorites.insert("Liebesträume No. 3".to_string());
    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebesträume No. 3".to_string()));

    let lengths: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(1000)
        .collect();

    assert_eq!(
        fibonacci().take(8).collect::<Vec<_>>(),
        vec![1, 1, 2, 3, 5, 8, 13, 21]
    );

    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");

    let text = "  ponies  \n   giraffes\niguanas  \nsquid".to_string();
    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    ["earth", "water", "air", "fire"]
        .iter()
        .map(|elt| println!("{}", elt));

    let text = "1\nfrond .25  289\n3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
    {
        println!("{:4.2}", number.sqrt());
    }

    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);
    let countries = ["Japan", "Brazil", "Kenya"];
    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }

    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);
    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();
    assert_eq!(
        all_parks,
        vec![
            "Tadasu-no-Mori Forest",
            "Maruyama Koen",
            "Percy Warner Park",
            "Dragon Park",
            "Mt. Tabor Park",
            "Forest Park"
        ]
    );

    assert_eq!(
        vec![None, Some("day"), None, Some("one")]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
        vec!["day", "one"]
    );

    let message = "To: jimb\r\n\
               From: superego <editor@oreilly.com>\r\n\
               \r\n\
               Did you get any writing done today?\r\n\
               When will you stop wasting time plotting fractals?\r\n";
    for header in message.lines().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }

    for body in message.lines().skip_while(|l| !l.is_empty()).skip(1) {
        println!("{}", body);
    }

    let mut chars = "226153980,1766319049".chars().peekable();
    assert_eq!(parse_number(&mut chars), 226153980);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 1766319049);
    assert_eq!(chars.next(), None);

    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));

    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);

    let bee_parts = ["head", "thorax", "abdomen"];
    let mut iter = bee_parts.iter();
    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    let meals = ["breakfast", "lunch", "dinner"];
    let mut iter = meals.iter().rev();
    assert_eq!(iter.next(), Some(&"dinner"));

    let upper_case: String = "große"
        .chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!("after: {:?}", c))
        .collect();
    assert_eq!(upper_case, "GROSSE");

    let v: Vec<i32> = (1..4).chain([20, 30, 40]).rev().collect();
    assert_eq!(v, [40, 30, 20, 3, 2, 1]);

    let columns = 800;
    let rows = 600;
    let mut pixels = vec![0; columns * rows];
    let mut threads = 8;
    let band_rows = rows / threads + 1;
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(band_rows * columns).collect();
    for (i, band) in bands.into_iter().enumerate() {
        let top = band_rows * i;
    }
    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, [(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);

    let endings = ["once", "twice", "chicken soup with rice"];
    let rhyme: Vec<_> = repeat("going").zip(endings).collect();
    assert_eq!(
        rhyme,
        vec![
            ("going", "once"),
            ("going", "twice"),
            ("going", "chicken soup with rice")
        ]
    );

    let message = "To: jimb\r\n\
               From: id\r\n\
               \r\n\
               Oooooh, donuts!!\r\n";
    let mut lines = message.lines();
    println!("Headers:");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }
    println!("\nBody:");
    for body in lines {
        println!("{}", body);
    }

    let a = ['1', '2', '3', '∞'];
    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().cloned().next(), Some('1'));

    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().cycle();
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));
    assert_eq!(spin.next(), Some(&"West"));
    assert_eq!(spin.next(), Some(&"North"));

    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });
    for line in fizz_buzz {
        println!("{}", line);
    }
}
