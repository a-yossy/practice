use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

struct Anime {
    name: &'static str,
    bechdel_pass: bool,
}

fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    sort_works(&mut table);

    let aria = Anime {
        name: "name",
        bechdel_pass: true,
    };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "name");
    assert_eq!((*anime_ref).name, "name");

    let mut v = vec![1, 2];
    v.sort();
    (&mut v).sort();

    let x = 10;
    let y = 20;
    let mut r = &x;
    if true {
        r = &y;
    }
    assert!(*r == 10 || *r == 20);

    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 729);

    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);
    assert!(!std::ptr::eq(rx, ry));

    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
}
