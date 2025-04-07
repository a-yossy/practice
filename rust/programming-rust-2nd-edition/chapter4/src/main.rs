#[derive(Debug)]
struct Person {
    name: String,
    birth: i32,
}

fn main() {
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");

    let mut composers = Vec::new();
    composers.push(Person {
        name: "name1".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "name2".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "name3".to_string(),
        birth: 1632,
    });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = &s;
    let u = s;

    let mut s = "string".to_string();
    s = "new_string".to_string();

    let mut s = "string".to_string();
    let t = s;
    s = "new_string".to_string();
    println!("{}", s);

    
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    assert_eq!(v, vec!["101", "104", "substitute"]);
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}
