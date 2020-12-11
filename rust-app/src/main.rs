fn main() {
    let name = ["mundo"];
    let job = std::thread::spawn(move || println!("Olá, {}!", name[0]));
    job.join().expect("unable to fork");
    println!("Adios!");
}
