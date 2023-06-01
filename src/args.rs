use clap::Command;

// #[derive(Debug)]
// #[allow(dead_code)]
// pub struct Arguments {
//     pub start: String,
//     pub name: Option<String>,
//     pub with: Option<String>,
// }

use std::io;

pub fn parse_args() -> Result<(), io::Error> {
    let sub_cmd = Command::new("start")
        .about("")
        .author("Ilango Rajagopal");

    let cmd = Command::new("zln")
        .bin_name("zln")
        .subcommand_required(true)
        .subcommand(sub_cmd);

    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("start", matches)) => matches,
        _ => unreachable!("shouldn't get here"),
    };

    dbg!(matches);

    Ok(())
}
