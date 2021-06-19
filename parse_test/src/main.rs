use clap::{Arg, App};

fn main() {
    let matches = App::new("Clap test program")
        .version("0.1")
        .author("Araldo van de Kraats <a.vandekraats@gmail.com>")
        .about("Just a clap test")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .takes_value(true)
            .required(true)
        )
        .get_matches();
    let config = matches.value_of("config").unwrap();
    println!("Value of config: {}", config);
}
