use std::fs::{self, create_dir, File};
use std::process;

use super::san_const::{sanjaiyan_html_content, SANJAIYAN_PASCAL_CONTENT};

#[derive(Debug)]
pub struct SanFileManagement {
    san_dir_name: String,
}

impl SanFileManagement {
    pub fn create(dir_name: &String) -> Self {
        SanFileManagement {
            san_dir_name: dir_name.to_string(),
        }
    }

    fn san_create_folders(&self, dir_path: &String) {
        if let Err(..) = create_dir(dir_path) {
            eprintln!(
                "Error: Could not create the '{dir_path}' folder. You can try it manually also."
            );
            process::exit(74);
        }
    }

    fn san_create_files(&self) {
        {
            let pascal_file_path = format!("{}/sanpasjs.pas", self.san_dir_name);

            if let Err(..) = File::create(&pascal_file_path) {
                eprintln!("Error: Could not create the Pascal file. You can try it manually also.");
                process::exit(74);
            }
            if let Err(..) = fs::write(&pascal_file_path, SANJAIYAN_PASCAL_CONTENT) {
                eprintln!("Failed to write to the pascal file");
            }
        };

        {
            let html_file_path = format!("{}/dist/index.html", self.san_dir_name);

            if let Err(..) = File::create(&html_file_path) {
                eprintln!("Error: Could not create the HTML file. You can try it manually also.");
                process::exit(74);
            }
            if let Err(..) = fs::write(
                &html_file_path,
                sanjaiyan_html_content(&self.san_dir_name).as_bytes(),
            ) {
                eprintln!("Failed to write to the html file");
            }
        };

        if let Err(..) = File::create(format!("{}/dist/index.js", self.san_dir_name)) {
            eprintln!("Error: Could not create the Javascript file. You can try it manually also.");
            process::exit(74);
        }
    }

    pub fn create_folder_and_files_sanjaiyan(&self) {
        self.san_create_folders(&self.san_dir_name);
        self.san_create_folders(&format!("{}/dist", &self.san_dir_name));

        self.san_create_files();

        println!(
            "Goto the project directory by running following command in your CLI -: \n\ncd {:?} ",
            &self.san_dir_name
        );
    }
}
