use rand::prelude::*;

fn main() {
    let name = ["mundo"];
    let job = std::thread::spawn(move || println!("Olá, {}!", name[0]));
    job.join().expect("unable to fork");
    println!("Here it's a random number for you: {}", random::<u8>());
    println!("Adios!");
}
