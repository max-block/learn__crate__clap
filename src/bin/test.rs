use clap::Clap;

#[derive(Clap, Debug)]
struct PlusCmd {
    a: i32,
    b: i32
}

/// It's a help message for the cli. Bla bla bla.
#[derive(Clap)]
enum SubCommand {
    #[clap()]
    Plus(PlusCmd)
}

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand

}

fn main() {
    let opts = Opts::parse();
    match opts.subcommand {
        SubCommand::Plus(cmd) => {
            println!("process `plus` cmd: {:?}", cmd);
        }        
    }
}
