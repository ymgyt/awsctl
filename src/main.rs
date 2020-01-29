use awsctl::cmd;
use std::error::Error;

fn main() {
    let version = env!("CARGO_PKG_VERSION");

    let arg = cmd::new(version).get_matches();

    let result: Result<(), Box<dyn Error>> = match arg.subcommand() {
        ("dynamo", Some(_sub_m)) => {
            println!("dynamo cmd called!");
            Ok(())
        },
        _ => Err("show help!".to_owned().into()),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

