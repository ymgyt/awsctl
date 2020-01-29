use clap::App;

pub fn new<'a>(version: &'a str) -> App<'a> {
    App::new("awsctl")
        .version(version)
        .author("ymgyt")
        .subcommand(
            App::new("dynamo")
                .about("DynamoDB operations")
        )
}