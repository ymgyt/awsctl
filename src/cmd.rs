use clap::App;

pub fn new<'a>() -> App<'a> {
    App::new("awsctl")
        .version("v0.0.1")
        .author("ymgyt")
        .subcommand(
            App::new("dynamo")
                .about("DynamoDB operations")
        )
}