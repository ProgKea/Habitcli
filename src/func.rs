#![allow(unused, deprecated)]
// TODO: Prevent users from entering habits with spaces (use underscores instead)

use crate::args;
use std::env;
use std::fs;
use std::io::*;
use std::path::Path;

const FILENAME: &'static str = ".habits";

// check if habit name already exist in file
pub fn check_name(name: String) -> anyhow::Result<(bool)> {
    let mut file = fs::File::open(&FILENAME)?;
    let mut content = String::new();
    file.read_to_string(&mut content);
    for line in content.lines() {
        if line.split_whitespace().next().unwrap_or("") == name {
            return Ok((true));
        }
    }
    Ok((false))
}

pub fn add(args: args::Arguments) -> anyhow::Result<()> {
    env::set_current_dir(env::home_dir().unwrap());
    if Path::new(&FILENAME).exists() {
        if check_name(args.clone().name).unwrap() {
            return Err(anyhow::Error::new(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Habit name already exists",
            )));
        }
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(&FILENAME)
        .unwrap();

    write!(&mut file, "{} {}\n", args.name, args.count);
    Ok(())
}

pub fn remove(args: args::Arguments) -> anyhow::Result<()> {
    env::set_current_dir(env::home_dir().unwrap());
    if !Path::new(&FILENAME).exists() {
        return Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::NotFound,
            "You have to add a habit first to checkin",
        )));
    }
    let name = args.name;
    if !check_name(name.to_string()).unwrap() {
        return Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Habit name doesnt exist",
        )));
    }

    let mut file = fs::File::open(&FILENAME)?;

    let mut content = String::new();
    let mut new_content = String::new();
    file.read_to_string(&mut content);

    for line in content.lines() {
        if line.split_whitespace().next().unwrap_or("") == name {
            new_content = content.replace(&format!("{}\n", line), "");
        }
    }

    file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&FILENAME)
        .unwrap();

    write!(&mut file, "{}", new_content);
    listall();

    Ok(())
}

pub fn edit(args: args::Arguments) -> anyhow::Result<()> {
    env::set_current_dir(env::home_dir().unwrap());
    if !Path::new(&FILENAME).exists() {
        return Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::NotFound,
            "You have to add a habit first to checkin",
        )));
    }
    let name = args.name;
    if !check_name(name.to_string()).unwrap() {
        return Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Habit name doesnt exist",
        )));
    }
    let count = args.count;
    if count < 0 {
        return Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Count can't be smaller than 0",
        )));
    }

    let mut file = fs::File::open(&FILENAME)?;

    let mut content = String::new();
    let mut new_content = String::new();
    file.read_to_string(&mut content);

    for line in content.lines() {
        if line.split_whitespace().next().unwrap_or("") == name {
            let new_count = count;
            new_content = content.replace(line, &format!("{} {}", name, &new_count.to_string()));
        }
    }
    file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&FILENAME)
        .unwrap();

    write!(&mut file, "{}", new_content);
    list(name);

    Ok(())
}

pub fn checkin(args: args::Arguments) -> anyhow::Result<()> {
    env::set_current_dir(env::home_dir().unwrap());
    if !Path::new(&FILENAME).exists() {
        return Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::NotFound,
            "You have to add a habit first to checkin",
        )));
    }
    let name = args.name;
    if !check_name(name.to_string()).unwrap() {
        return Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Habit name doesnt exist",
        )));
    }

    let mut file = fs::File::open(&FILENAME)?;

    let mut content = String::new();
    let mut new_content = String::new();
    file.read_to_string(&mut content);

    for line in content.lines() {
        if line.split_whitespace().next().unwrap_or("") == name {
            let new_count = line
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap()
                + 1;
            new_content = content.replace(line, &format!("{} {}", name, &new_count.to_string()));
        }
    }
    file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&FILENAME)
        .unwrap();

    write!(&mut file, "{}", new_content);
    list(name);

    Ok(())
}

pub fn listall() -> anyhow::Result<()> {
    env::set_current_dir(env::home_dir().unwrap());
    if !Path::new(&FILENAME).exists() {
        return Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::NotFound,
            "You have to add a habit first to list habits",
        )));
    }

    let mut file = fs::File::open(&FILENAME).expect("Could not open file");
    let mut content = String::new();
    file.read_to_string(&mut content);

    for line in content.lines() {
        println!("Habit Name: {}", line);
    }

    Ok(())
}

pub fn list(name: String) -> anyhow::Result<()> {
    env::set_current_dir(env::home_dir().unwrap());
    if !Path::new(&FILENAME).exists() {
        return Err(anyhow::Error::new(Error::new(
            std::io::ErrorKind::NotFound,
            "You have to add a habit first to list habits",
        )));
    }
    let mut file = fs::File::open(&FILENAME).expect("Could not open file");
    let mut content = String::new();
    file.read_to_string(&mut content);

    for line in content.lines() {
        if line.split_whitespace().next().unwrap_or("") == name {
            println!("Habit Name: {}", line);
        }
    }
    Ok(())
}
