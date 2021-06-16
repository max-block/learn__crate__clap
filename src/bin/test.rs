use clap::Clap;

/// It's a help message for the cli. Bla bla bla.
#[derive(Clap)]
struct Opts {
    /// First argument
    name: String,

    /// Second argument
    age: u32,
}

fn main() {
    let opts = Opts::parse();
    println!("name: {}", opts.name);
    println!("age: {}", opts.age);
}
