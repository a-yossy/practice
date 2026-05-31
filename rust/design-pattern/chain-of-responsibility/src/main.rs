mod department;
mod patient;

use department::{Cashier, Department, Doctor, Medical, Reception};
use patient::Patient;

fn main() {
    let cachier = Cashier::default();
    let medical = Medical::new(cachier);
    let doctor = Doctor::new(medical);
    let mut recption = Reception::new(doctor);

    let mut patient = Patient {
        name: "John".into(),
        ..Patient::default()
    };

    recption.execute(&mut patient);

    println!("\nThe patient has been already handled:\n");

    recption.execute(&mut patient);
}
