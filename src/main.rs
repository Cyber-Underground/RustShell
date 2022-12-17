#[allow(non_snake_case)]

use std::io::{self, Write, stdout};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::wincon::SetConsoleTitleW;
use colored::*;
use std::{time::Duration};

mod functions;

fn main() -> io::Result<()> {
    ansi_term::enable_ansi_support().unwrap();

    // Clear the command prompt
    print!("{}[2J", 27 as char);
    stdout().flush().unwrap();

    // Set the console title
    let new_title = "Nothing";
    let title: Vec<u16> = OsStr::new(new_title)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    unsafe { 
        SetConsoleTitleW(title.as_ptr()) 
    };

    let lines = [
        "",
        "",
        "     /$$$$$$$   /$$$$$$  /$$   /$$ /$$$$$$$$ /$$       /$$      ",
        "    | $$__  $$ /$$__  $$| $$  | $$| $$_____/| $$      | $$      ",
        "    | $$  \\ $$| $$  \\__/| $$  | $$| $$      | $$      | $$      ",
        "    | $$$$$$$/|  $$$$$$ | $$$$$$$$| $$$$$   | $$      | $$      ",
        "    | $$__  $$ \\____  $$| $$__  $$| $$__/   | $$      | $$      ",
        "    | $$  \\ $$ /$$  \\ $$| $$  | $$| $$      | $$      | $$      ",
        "    | $$  | $$|  $$$$$$/| $$  | $$| $$$$$$$$| $$$$$$$$| $$$$$$$$",
        "    |__/  |__/ \\______/ |__/  |__/|________/|________/|________/",
        "",
    ];

    let access_denied = [
        "    /$$$$$$                                                                         /$$    ",
        "   |_  $$_/                                                                        | $$    ",
        "     | $$   /$$$$$$$   /$$$$$$$  /$$$$$$   /$$$$$$   /$$$$$$   /$$$$$$   /$$$$$$$ /$$$$$$  ",
        "     | $$  | $$__  $$ /$$_____/ /$$__  $$ /$$__  $$ /$$__  $$ /$$__  $$ /$$_____/|_  $$_/  ",
        "     | $$  | $$  \\ $$| $$      | $$  \\ $$| $$  \\__/| $$  \\__/| $$$$$$$$| $$        | $$    ",
        "     | $$  | $$  | $$| $$      | $$  | $$| $$      | $$      | $$_____/| $$        | $$ /$$",
        "    /$$$$$$| $$  | $$|  $$$$$$$|  $$$$$$/| $$      | $$      |  $$$$$$$|  $$$$$$$  |  $$$$/",
        "   |______/|__/  |__/ \\_______/ \\______/ |__/      |__/       \\_______/ \\_______/   \\___/  ",
        "",
    ];

    let access_granted = [
        "     /$$$$$$                                                      /$$    ",
        "    /$$__  $$                                                    | $$    ",
        "   | $$  \\__/  /$$$$$$   /$$$$$$   /$$$$$$   /$$$$$$   /$$$$$$$ /$$$$$$  ",
        "   | $$       /$$__  $$ /$$__  $$ /$$__  $$ /$$__  $$ /$$_____/|_  $$_/  ",
        "   | $$      | $$  \\ $$| $$  \\__/| $$  \\__/| $$$$$$$$| $$        | $$    ",
        "   | $$    $$| $$  | $$| $$      | $$      | $$_____/| $$        | $$ /$$",
        "   |  $$$$$$/|  $$$$$$/| $$      | $$      |  $$$$$$$|  $$$$$$$  |  $$$$/",
        "    \\______/  \\______/ |__/      |__/       \\_______/ \\_______/   \\___/  ",
        "",
    ];

    for line in &lines {
        println!("{}", line.truecolor(80, 0, 255));
    }

    let password = "1337";

    print!("        Please enter the password: ");
    io::stdout().flush()?;

    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("        Failed to read line");

    if user_input.trim() == password {
        print!("{}[2J", 27 as char);
        stdout().flush().unwrap();
        for line in &access_granted {
            println!("{}", line.truecolor(57, 255, 20));
        }
        std::thread::sleep(Duration::new(1, 0));
        print!("{}[2J", 27 as char);
        stdout().flush().unwrap();
        for line in &lines {
            println!("{}", line.truecolor(80, 0, 255));
        }
    } else {
        print!("{}[2J", 27 as char);
        stdout().flush().unwrap();
        for line in &access_denied {
            println!("{}", line.truecolor(255, 0, 80));
        }
        std::thread::sleep(Duration::new(2, 0));
        std::process::exit(0);
    }

    loop {
        print!("    rshell > ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");

        if input.is_empty() {
            // If the user didn't enter any input, print the prompt again and continue
            io::stdout().flush()?;
            continue;
        }
        
        match command {
            "exit" => {
                break;
            }
            "rm" | "del" => {
                functions::remove();
            }
            "files" => {
                functions::tree_search()
            }
            "clear" | "cls" => {
                print!("{}[2J", 27 as char);
                stdout().flush().unwrap();
                for line in &lines {
                    println!("{}", line.truecolor(80, 0, 255));
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
            _ => println!("        Unknown command"),
        }
    }
    Ok(())
}
