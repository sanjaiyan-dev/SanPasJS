mod files;
mod lexer;
mod parser;

use std::thread;

use clap::{Arg, Command, ValueHint};
use lexer::san_lex::SanjaiyanPascalCode;

use crate::parser::san_parser::SanjaiyanPascalTokens;

fn main() {
    let sanjaiyan_command_line = Command::new("Sanjaiyan_Pas_Js")
        .bin_name("sanpasjs")
        .author("Sanjaiyan")
        .version("v0.0.3")
        .about("Compile your Pascal program to Javascript")
        .subcommand(
            Command::new("new")
                .short_flag('n')
                .alias("create")
                .arg(
                    Arg::new("name")
                        .required(true)
                        .short('n')
                        .long("name")
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
                        .long("name")
                        .help("Enter your project name."),
                )
                .about("Initialize new project folder with relevant files for quick start."),
        )
        .subcommand(
            Command::new("compile")
                .short_flag('c')
                .about(
                    "Compile your Pascal program to Javascript code which can be executed in the web browsers.",
                )
                .arg(
                    Arg::new("input_file")
                        .aliases(["pas", "pascal"])
                        .default_value("./sanpasjs.pas")
                        .short('i')
                        .long("input")
                        .help("Pascal file path needed to be compiled")
                        .value_hint(ValueHint::FilePath),
                )
                .arg(
                    Arg::new("output_file")
                        .aliases(["js", "javascript"])
                        .default_value("./dist/index.js")
                        .short('o')
                        .long("output")
                        .help("Javascript file path where compiled file should be placed.")
                        .value_hint(ValueHint::FilePath),
                ),
        )
        .get_matches();

    if let Some(san_cmd) = sanjaiyan_command_line.subcommand() {
        match san_cmd.0.to_lowercase().as_str() {
            "new" | "init" => {
                if let Some(sanjaiyan_prj_name) = san_cmd.1.get_one::<String>("name") {
                    let sanjaiyan_project_create =
                        files::san_file_manager::SanFileManagement::create(sanjaiyan_prj_name);

                    let san_thread = thread::spawn(move || {
                        sanjaiyan_project_create.create_folder_and_files_sanjaiyan();
                    });

                    println!("Creating the project named '{:?}' ", &sanjaiyan_prj_name);
                    println!("Creating the relevant files and folders needed to get started :) ");

                    san_thread.join().unwrap();
                }
            }
            "compile" => {
                let sanjaiyan_input_pascal_file = {
                    match san_cmd.1.get_one::<String>("input_file") {
                        Some(san_file) => san_file,
                        None => "./sanpasjs.pas",
                    }
                };

                let sanjaiyan_output_javascript_file = {
                    match san_cmd.1.get_one::<String>("output_file") {
                        Some(san_file) => san_file,
                        None => "./dist/index.js",
                    }
                };

                {
                    let sanjaiyan_pascal_code_struct =
                        SanjaiyanPascalCode::new(sanjaiyan_input_pascal_file);

                    let sanjaiyan_pascal_token_parser = SanjaiyanPascalTokens::new(
                        sanjaiyan_pascal_code_struct.sanjaiyan_organize_tokens(),
                    );

                    sanjaiyan_pascal_token_parser
                        .sanjaiyan_write_to_js_file_san(sanjaiyan_output_javascript_file)
                }
            }
            _ => {}
        }
    }
}
