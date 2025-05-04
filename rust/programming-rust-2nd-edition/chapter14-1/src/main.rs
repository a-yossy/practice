use std::thread;

struct City {
    name: String,
    population: i64,
    country: String,
    monster_attack_risk: f64,
}

fn city_population_descending(city: &City) -> i64 {
    -city.population
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}

#[derive(Clone, Copy)]
struct Static {
    population: i64,
}

impl City {
    fn get_statistic(&self, stat: Static) -> i64 {
        0
    }
}

fn sort_by_statistic(cities: &mut Vec<City>, stat: Static) {
    cities.sort_by_key(|city| -city.get_statistic(stat));
}

fn start_sorting_thread(mut cities: Vec<City>, stat: Static) -> thread::JoinHandle<Vec<City>> {
    let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) };

    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}

fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
where
    F: Fn(&City) -> bool,
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }

    count
}

fn has_monster_attacks(city: &City) -> bool {
    city.monster_attack_risk > 0.0
}

fn call_twice<F>(mut closure: F)
where
    F: FnMut(),
{
    closure();
    closure();
}

fn produce_glossary() -> std::collections::HashMap<String, String> {
    let mut dict = std::collections::HashMap::new();
    dict.insert("CityA".to_string(), "Glossary for CityA".to_string());
    dict.insert("CityB".to_string(), "Glossary for CityB".to_string());
    dict
}

fn main() {
    let my_key_fn: fn(&City) -> i64 = if true {
        city_population_descending
    } else {
        |city| city.population
    };

    let cities = vec![
        City {
            name: "CityA".to_string(),
            population: 1000,
            country: "CountryA".to_string(),
            monster_attack_risk: 0.1,
        },
        City {
            name: "CityB".to_string(),
            population: 2000,
            country: "CountryB".to_string(),
            monster_attack_risk: 0.2,
        },
    ];
    let n = count_selected_cities(&cities, has_monster_attacks);
    let n = count_selected_cities(&cities, |city| city.monster_attack_risk > 1.0);

    let my_str = "hello".to_string();
    let f = || drop(my_str);

    let dict = produce_glossary();
    let debug_dump_dict = || {
        for (key, value) in &dict {
            println!("{:?} - {:?}", key, value);
        }
    };

    debug_dump_dict();
    debug_dump_dict();

    let mut i = 0;
    let incr = || {
        i += 1;
        println!("Ding! i is now: {}", i);
    };
    call_twice(incr);
    call_twice(|| i += 1);

    let y = 10;
    let add_y = |x| x + y;
    let copy_of_add_y = add_y;
    assert_eq!(add_y(copy_of_add_y(5)), 25);

    // let mut x = 10;
    // let mut add_to_x = |n| {
    //     x += n;
    //     x
    // };
    // let copy_of_add_to_x = add_to_x;
    // assert_eq!(add_to_x(copy_of_add_to_x(5)), 25);

    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{}", greeting);
    };
    greet.clone()("World");
    greet.clone()("Rust");/
}
