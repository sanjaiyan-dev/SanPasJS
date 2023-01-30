use std::fs::{create_dir, File};
use std::process;
use std::thread;

#[derive(Debug)]
pub struct SanFileManagement {
    san_dir_name: String,
}
 
impl SanFileManagement {
    pub fn create(dir_name: String) -> Self {
        SanFileManagement {
            san_dir_name: dir_name,
        }
    }

    fn san_create_folders(&self, dir_path: String) {
        let san_folder_create = create_dir(&dir_path);
        if san_folder_create.is_err() {
            eprintln!(
                "Error: Could not create the {} folder. You can try it manually also.",
                dir_path
            );
            process::exit(74);
        }
    }

    fn san_create_files(&self) {
        {
            let mut pascal_file_san = File::create(format!("{}/sanpasjs.pas", self.san_dir_name));
            if pascal_file_san.is_err() {
                eprintln!("Error: Could not create the Pascal file. You can try it manually also.");
                process::exit(74);
            }
        };

        {
            let mut js_file_san = File::create(format!("{}/dist/index.js", self.san_dir_name));
            if js_file_san.is_err() {
                eprintln!(
                    "Error: Could not create the Javascript file. You can try it manually also."
                );
                process::exit(74);
            }
        };

        {
            let mut html_file_san = File::create(format!("{}/dist/index.html", self.san_dir_name));
            if html_file_san.is_err() {
                eprintln!("Error: Could not create the HTML file. You can try it manually also.");
                process::exit(74);
            }
        };
    }

    pub fn create_folder_and_files_sanjaiyan(&self) {
        println!("Creating the relevant files and folders.");

        self.san_create_folders(self.san_dir_name.to_string());
        self.san_create_folders(format!("{}/dist", &self.san_dir_name.to_string()));

        self.san_create_files();
    }
}
