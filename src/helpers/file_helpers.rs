use std::io::prelude::*;
use std::{fs, io, path};

pub fn create_and_write_to_file(file_name: &str, content: &str) -> io::Result<()> {
    let mut flie_path = String::from("todos/");
    flie_path.push_str(file_name);
    flie_path.push_str(".txt");
    let is_existing_file = path::Path::new(&flie_path).exists();
    if is_existing_file {
        return Err(io::Error::from(io::ErrorKind::AlreadyExists));
    }
    let mut file = fs::File::create(flie_path)?;
    file.write(content.as_bytes())?;
    Ok(())
}

pub fn remove_file(file_name: &str) -> io::Result<()> {
    let mut path = String::from("todos/");
    path.push_str(file_name);
    path.push_str(".txt");
    fs::remove_file(path)?;
    Ok(())
}

pub fn update_file(file_name: &str, new_content: &str) -> io::Result<()> {
    let mut flie_path = String::from("todos/");
    flie_path.push_str(file_name);
    flie_path.push_str(".txt");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(false)
        .open(flie_path)?;
    file.write(new_content.as_bytes())?;
    Ok(())
}

pub fn read_files_from_dir(dir: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        println!("{:?}", entry.file_name());
    }
    Ok(())
}
