use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    let open_file: io::Result<File> = File::open("hello.txt");

    let mut file: File = match open_file {
        Ok(file) => file,
        Err(e) => panic!("Failed to Open file: {e}"),
    };

    let mut user_file: String = String::new();

    match file.read_to_string(&mut user_file) {
        Ok(val) => val,
        Err(e) => panic!("Error to Read file {e}"),
    };

    println!("{}", user_file);

    let open_file_2: io::Result<File> = File::open("raja_mexico.txt");

    let mut file_2 = match open_file_2 {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                match File::create("raja_mexico.txt") {
                    Ok(new_file) => {
                        eprintln!("Failed to found file , file will be created");
                        new_file
                    }
                    Err(_) => panic!("failed to created file , exit program !"),
                };
                fs::write("raja_mexico.txt", "Ini Aku Raja Mexico ").unwrap_or_else(|err| {
                    panic!("success to create a file but failed to write: {err:?}, please Re run the program ");
                });
                panic!("success to create and write file. please re run the program");
            }
            _ => {
                panic!("failed to open file ")
            }
        },
    };

    let mut content_file2: String = String::new();
    file_2
        .read_to_string(&mut content_file2)
        .expect("Failed to Read String");
    println!("{}", content_file2);
}
