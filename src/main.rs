use std::ffi::OsStr;
use std::fs;
use std::io::{self, Read, Write};
use std::os::windows::ffi::OsStrExt;
use std::process::{Command, Stdio};
use winapi::um::wincon::SetConsoleTitleW;
// extern crate libc;
// use libc::geteuid;

fn main() -> io::Result<()> {
    // Set the console title
    let new_title = "Nothing";
    let title: Vec<u16> = OsStr::new(new_title)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    unsafe { SetConsoleTitleW(title.as_ptr()) };

    io::stdout().flush().unwrap();


    //Checks if the current user has administrator privileges.
    /*
    let euid = unsafe { geteuid() };
    if euid == 0 {
        println!("The current user is an administrator.");
    } else {
        println!("The current user is not an administrator.");
    }
*/
    // Print the lines
    let lines = [
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
        println!("{}", line);
    }

    print!("    rshell> ");
    io::stdout().flush()?;

    loop {
        // Read a line of input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Split the input into words (i.e. the command and its arguments)
        let words: Vec<&str> = input.split_whitespace().collect();

        if words.is_empty() {
            // If the user didn't enter any input, print the prompt again and continue
            print!("    rshell> ");
            io::stdout().flush()?;
            continue;
        }

        // Get the command and its arguments
        let command = words[0];
        let args = &words[1..];

        if command == "exit" {
            // If the user entered the "exit" command, break out of the loop and end the program
            break;
        } else if command == "rm" {
    // If the user entered the "rm" command, delete the specified file or directory
    if args.len() != 1 {
        println!("rm: missing operand");
        print!("    rshell> ");
        continue;
    } else {
        let path = args[0];
        match fs::metadata(path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    match fs::remove_file(path) {
                        Ok(_) => println!("rm: removed '{}'", path),
                        Err(err) => println!("rm: failed to remove '{}': {}", path, err),
                    }
                } else if metadata.is_dir() {
                    match fs::remove_dir_all(path) {
                        Ok(_) => println!("rm: removed '{}'", path),
                        Err(err) => println!("rm: failed to remove '{}': {}", path, err),
                    }
                }
            },
            Err(err) => println!("rm: failed to get metadata for '{}': {}", path, err),
        }
    }
    io::stdout().flush()?;
    continue;
} else if command == "pwsh" {
            // If the user entered the "pwsh" command, run the specified command in PowerShell
            let mut child = Command::new("powershell.exe")
                .args(args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?;

            // Wait for the command to finish executing
            let _result = child.wait()?;
            continue;
        } else if command == "elevate" {
            // Create a new process that runs the "runas" command
            let mut child = Command::new("runas")
                .arg("/user:Administrator")
                .arg(std::env::current_exe()?)
                .args(args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?;

            // Wait for the program to finish executing
            let _result = child.wait()?;
        } else if command == "connect" || command == "attach" {
            // Create a new process that runs the specified program
            let mut child = Command::new(args[0])
                .args(&args[1..])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .spawn()?;

            // Connect the standard input and output of the current program to the other program
            let stdin = child.stdin.as_mut().unwrap();
            let stdout = child.stdout.as_mut().unwrap();

            // Read input from the user and write it to the other program
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            stdin.write_all(input.as_bytes())?;
            stdin.flush()?;

            // Read output from the other program and write it to the console
            let mut output = String::new();
            stdout.read_to_string(&mut output)?;
            println!("{}", output);
        } else {
            println!("  rshell: command not found: {}", command);
            print!("    rshell> ");
            io::stdout().flush()?;
            continue;
        }
    }

    Ok(())
}
