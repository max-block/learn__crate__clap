use clap::App;

fn main() {
    let matches = App::new("my mega program")
        .version("0.1")
        .about("bla bla bla")
        .subcommand(App::new("plus").alias("p").arg("<x>").arg("<y>"))
        .subcommand(App::new("minus").alias("m").arg("<x>").arg("<y>"))
        .get_matches();

    match matches.subcommand() {
        Some(("plus", cmd_matches)) => {
            let x: f64 = cmd_matches.value_of_t_or_exit("x");
            let y: f64 = cmd_matches.value_of_t_or_exit("y");
            println!("result: {}", x + y)
        }
        Some(("minus", cmd_matches)) => {
            let x: f64 = cmd_matches.value_of_t_or_exit("x");
            let y: f64 = cmd_matches.value_of_t_or_exit("y");
            println!("result: {}", x - y)
        }
        _ => unreachable!(),
    }
}
