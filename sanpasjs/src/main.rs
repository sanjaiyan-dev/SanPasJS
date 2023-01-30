mod files;
mod lexer;

use clap::{builder, Arg, Command, ValueHint};
use logos::Logos;

fn main() {
    let sanjaiyan_command_line =
        Command::new("Sanjaiyan_Pas_Js")
            .bin_name("sanpasjs")
            .author("Sanjaiyan")
            .version("v0.0.3")
            .about("Compile your Pascal program to Javascript")
            .subcommand(
                Command::new("new")
                    .short_flag('n')
                    .arg(
                        Arg::new("name")
                            .required(true)
                            .short('n')
                            .help("Enter your project name."),
                    )
                    .about("Create new project folder with relevant files for quick start."),
            )
            .subcommand(
                Command::new("init")
                    .short_flag('i')
                    .arg(
                        Arg::new("name")
                            .required(true)
                            .short('n')
                            .help("Enter your project name."),
                    )
                    .about("Initialize new project folder with relevant files for quick start."),
            )
            .subcommand(Command::new("compile").short_flag('c').about(
                "Compile your Pascal program to Javascript which can be ran in web browsers.",
            ))
            .get_matches();

    match sanjaiyan_command_line.subcommand() {
        Some(san_cmd) => {
            println!("{:?}", &san_cmd.0);

            match san_cmd.0.to_lowercase().as_str() {
                "new" | "init" => {
                    let sanjaiyan_prj_name = san_cmd.1.get_one::<String>("name");
                }
                _ => {}
            }
        }
        None => {}
    }
}
