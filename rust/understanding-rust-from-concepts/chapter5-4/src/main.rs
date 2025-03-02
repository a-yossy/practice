trait SayHello {
    fn say_hello(&self);
}

trait SayThankyou {
    fn say_thankyou(&self);
}

struct EnglishPerson;
struct SpanishPerson;

impl SayHello for EnglishPerson {
    fn say_hello(&self) {
        println!("Hello");
    }
}

impl SayThankyou for EnglishPerson {
    fn say_thankyou(&self) {
        println!("Thank you");
    }
}

impl SayHello for SpanishPerson {
    fn say_hello(&self) {
        println!("Hola");
    }
}

impl SayThankyou for SpanishPerson {
    fn say_thankyou(&self) {
        println!("Gracias");
    }
}

fn say_hello_general<T: SayHello>(speaker: &T) {
    speaker.say_hello();
}

fn say_thankyou_general<T: SayThankyou>(speaker: &T) {
    speaker.say_thankyou();
}

trait Run {
    fn run(&self);
}

impl Run for EnglishPerson {
    fn run(&self) {
        println!("Run");
    }
}

impl Run for SpanishPerson {
    fn run(&self) {
        println!("Correr");
    }
}

fn say_thankyou_and_run<T: SayThankyou + Run>(person: &T) {
    person.say_thankyou();
    person.run();
}

fn main() {
}
