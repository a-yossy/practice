use crate::users::UserCollection;
mod users;

fn main() {
    print!("Iterators are widely used in the standard library");

    let array = &[1, 2, 3];
    let iterator = array.iter();

    iterator.for_each(|e| print!("{e} "));

    println!("\n\nLet's test our own iterator.\n");

    let users = UserCollection::new();
    let mut iterator = users.iter();

    println!("1st element: {:?}", iterator.next());
    println!("2nd element: {:?}", iterator.next());
    println!("3rd element: {:?}", iterator.next());
    println!("4th element: {:?}", iterator.next());

    print!("\nAll elements in user collection: ");
    users.iter().for_each(|e| print!("{e} "));

    println!();
}
