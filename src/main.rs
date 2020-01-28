use awsctl::cmd;
use std::error::Error;

fn main() {
    let m = cmd::new().get_matches();

    let result: Result<(), Box<dyn Error>> = match m.subcommand() {
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
