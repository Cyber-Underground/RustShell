#[allow(non_snake_case)]

use std::io::{self, Write, stdout};
use std::ffi::OsStr;
use std::fs;
use std::os::windows::ffi::OsStrExt;
use winapi::um::wincon::SetConsoleTitleW;
use colored::*;
use std::path::Path;
use std::{time::Duration};

// This function recursively prints the contents of a directory in a tree view-like format.
// The `indent` parameter controls the amount of indentation for each level of the tree.
fn print_directory_tree(path: &Path, indent: usize) {
    // Print the name of the current directory, indented by the specified amount
    println!("{:indent$}{}", "", path.to_string_lossy(), indent = indent * 4);

    // Exit the function if the current indent level is greater than or equal to 2
    // Change the indent value to make it search further
    
    if indent >= 3 {
        return;
    }
    
    // Get an iterator over the entries in the directory
    let entries = fs::read_dir(path).unwrap();

    // Iterate over the entries in the directory
    for entry in entries {
        // Get the path of the entry
        let entry_path = entry.unwrap().path();

        // If the entry is a directory, recursively print its contents
        if entry_path.is_dir() {
            print_directory_tree(&entry_path, indent + 0);
        }
        // Otherwise, just print the name of the entry
        else {
            println!("{:indent$}{}", "", entry_path.display(), indent = (indent + 1) * 4);
        }
    }
}

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
            "exit" => break,
            "rm" | "del" => {
                // Get the first argument after the `rm` command, which should be the path to the file or directory to be deleted
                let arg = parts.next().unwrap_or("");
                match fs::remove_dir_all(arg) {
                    // If the directory or file was successfully deleted, move on to the next one
                    Ok(_) => continue,
                    // If the error is that the path does not exist, print an error message and move on to the next one
                    Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                        println!("        rm: {}: No such file or directory", arg);
                        continue;
                    }
                    // If the error is that the file cannot be deleted, print an error message and move on to the next one
                    Err(ref e) if e.kind() == io::ErrorKind::PermissionDenied => {
                        println!("        rm: {}: Cannot delete file: permission denied", arg);
                        continue;
                    }
                    // If the error is something other than the path not existing or permission denied, print the error and move on to the next one
                    Err(e) => {
                        eprintln!("        rm: {}: {}", arg, e);
                        continue;
                    }
                }
            }
            "files" => {
                // Get the directory and indent level from the command line arguments
                let mut args = std::env::args_os();

                let directory = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());


                let indent_level = if args.len() > 2 {
                    args.nth(2).unwrap().to_string_lossy().parse::<usize>().unwrap()
                } else {
                    0
                };

                // Call the print_directory_tree function to print the contents of the directory
                print_directory_tree(Path::new(&directory), indent_level);
            }
            "clear" | "cls" => {
                print!("{}[2J", 27 as char);
                stdout().flush().unwrap();
                for line in &lines {
                    println!("{}", line.truecolor(80, 0, 255));
                }
            }
            _ => println!("        Unknown command"),
        }

    }

    Ok(())
}
