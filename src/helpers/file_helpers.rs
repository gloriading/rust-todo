use std::io::prelude::*;
use std::{fs, io, path};

fn build_path(file_name: &str) -> String {
    let mut file_path = String::from("todos/");
    file_path.push_str(file_name);
    file_path.push_str(".txt");
    file_path
}

pub fn read_file(file_name: &str) -> Result<String, io::Error> {
    let file_path = build_path(file_name);
    let s1 = fs::read_to_string(file_path)?;
    Ok(s1)
}

pub fn create_and_write_to_file(file_name: &str, content: &str) -> io::Result<()> {
    let file_path = build_path(file_name);
    let is_existing_file = path::Path::new(&file_path).exists();
    if is_existing_file {
        return Err(io::Error::from(io::ErrorKind::AlreadyExists));
    }
    let mut file = fs::File::create(file_path)?;
    file.write(content.as_bytes())?;
    Ok(())
}

pub fn remove_file(file_name: &str) -> io::Result<()> {
    let file_path = build_path(file_name);
    fs::remove_file(file_path)?;
    Ok(())
}

pub fn update_file(file_name: &str, new_content: &str) -> io::Result<()> {
    let file_path = build_path(file_name);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(false)
        .open(file_path)?;
    file.write(new_content.as_bytes())?;
    Ok(())
}

// TODO: handle 2 unwraps
pub fn read_files_from_dir(dir: &str) -> io::Result<()> {
    println!("file count: {}", fs::read_dir(dir).unwrap().count());

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        println!(
            "{:?}",
            path::Path::new(entry.file_name().to_str().unwrap())
                .file_stem()
                .unwrap()
        );
    }
    Ok(())
}
