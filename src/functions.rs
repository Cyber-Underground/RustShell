use std::io::{self, Write, stdout, BufRead, BufReader};
use std::fs;
use std::path::Path;
use std::fs::File;
use colored::*;

pub fn remove() {
    /*
    //ask the file or directory to remove
    println!("Enter the file or directory to remove: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let path = Path::new(&input);
    fs::remove_dir_all(path).unwrap();
    println!("Removed: {}", path.display());
    */
}

pub fn whereis() {
    //find the current directory
    let path = std::env::current_dir().unwrap();
    println!("        Current directory: {}", path.display());
}

pub fn scan() {
    // if file already exists, ask the user if they want to overwrite it
    let path = Path::new("C:\\files");
    fs::create_dir_all(path).unwrap();
    let path = Path::new("C:\\files\\files.txt");
    if path.exists() {
        println!("        File already exists!");
        println!("        Do you want to overwrite it? (y/n)");
        print!("        > ");
        stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        println!();
        if input == "y" {
            fs::remove_file(path).unwrap();
            println!("        Removed: {}", path.display());
            println!("        Creating new folder...");

            let success = [
            "        Scan Complete!            ",
            ];

            //scan C: drive
            let dir = Path::new("C:\\");
            println!("        Creating new file...");
            let mut file = File::create("C:\\files\\files.txt").unwrap(); // create in 'C:\files'
            println!("        Created file: C:\\files\\files.txt");
            let mut counter = 0;
            println!();
            println!("        This Might Take A While...");
            scan_dir(&dir, &mut file, &mut counter).unwrap();
            for line in &success {
                println!("{}", line.truecolor(0, 255, 0));
                stdout().flush().unwrap();
            }
        } else {
            println!("        Aborting...");
        }
    } else {

        //scan C: drive
        let dir = Path::new("C:\\");
        println!("        Creating new file...");
        let mut file = File::create("C:\\files\\files.txt").unwrap(); // create in 'C:\files'
        println!("        Created file: C:\\files\\files.txt");
        let mut counter = 0;
        println!();
        println!("        This Might Take A While...");
        scan_dir(&dir, &mut file, &mut counter).unwrap();
        print!("{}", "        Scan Complete!            ".truecolor(0, 255, 0));
        stdout().flush().unwrap();
        println!();
    }
}

fn scan_dir(dir: &Path, file: &mut File, counter: &mut i32) -> Result<(), Box<dyn std::error::Error>> {
    //if *counter % 1 == 0 { // uncomment this line to change how often the 'counter' is updated
        //print!("Scanning: {}{}\r", dir.display(), " ".repeat(70));
        print!("        Scanned {} files {}\r", counter, " ".repeat(10));
        stdout().flush().unwrap();
    //}

    // The rest of the code remains unchanged
    let blacklisted_dirs: Vec<String> = vec![
        "C:\\Windows".to_string(), 
        "C:\\ProgramData\\Microsoft\\Windows\\Containers\\BaseImages".to_string(),
        "C:\\Users\\All Users".to_string(),
        "C:\\Documents and Settings".to_string(),
        "C:\\ProgramData\\Application Data".to_string(),
        "C:\\ProgramData\\Desktop".to_string(),
        "C:\\ProgramData\\Documents".to_string(),
        "C:\\ProgramData\\Start Menu".to_string(),
        "C:\\ProgramData\\Templates".to_string(),
        "C:\\Users\\Default".to_string(),
        ];
    if !blacklisted_dirs.contains(&dir.to_string_lossy().to_string()) {
        match fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_file() {
                        file.write_all(format!("{}\n",path.to_string_lossy()).as_bytes())?;
                        *counter += 1;
                    } else if path.is_dir() {
                        scan_dir(&path, file, counter)?;
                    }
                }
                Ok(())
            }
            Err(e) => {
                let mut log_file = File::create("error.log").unwrap();
                writeln!(log_file, "Error reading directory {}: {}", dir.display(), e).unwrap();
                Ok(())
            }
        }
    } else {
        Ok(())
    }
} 

pub fn find() {
    //ask the user for the file to search for and search it from C:\ScannedFiles\ScannedFiles.txt
    println!("Enter the file to search for:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input = input.trim();

    let file = File::open("C:\\files\\files.txt").unwrap(); // uncomment this line to search from the C:\ScannedFiles\ScannedFiles.txt file
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(input) {
            println!("{}", line);
        }
    }
}

pub fn tree() {
    // Ask the user for a directory to search in
    println!("Enter the directory to search in:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input = input.trim();

    // Convert the input to a Path object
    let path = Path::new(input);

    // Check if the path is a directory
    if path.is_dir() {
        // If it is a directory, ask the user for the maximum depth to search in
        println!("Enter the maximum depth to search in (0 for no limit):");
        let mut depth_input = String::new();
        io::stdin().read_line(&mut depth_input).expect("Error reading input");
        let depth_input = depth_input.trim();

        // Convert the depth input to a usize
        let max_depth = match depth_input.parse::<usize>() {
            Ok(depth) => depth,
            Err(e) => {
                println!("Error parsing depth: {}", e);
                return;
            }
        };

        // Display the contents of the directory
        display_directory_contents(path, 0, max_depth);
    } else {
        println!("The input is not a directory.");
    }
}

fn display_directory_contents(path: &Path, depth: usize, max_depth: usize) {
    // Check if the maximum depth has been reached
    if max_depth > 0 && depth >= max_depth {
        return;
    }

    // Get an iterator over the entries in the directory
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(e) => {
            println!("Error reading directory: {}", e);
            return;
        }
    };

    // Iterate over the entries
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                println!("Error reading entry: {}", e);
                continue;
            }
        };

        // Get the path of the entry
        let entry_path = entry.path();

        // Display the entry name, indented by the depth
        let indent = "    ".repeat(depth);
        println!("{}{}", indent, entry_path.display());

        // If the entry is a directory, recursively display its contents
        if entry_path.is_dir() {
            display_directory_contents(&entry_path, depth + 1, max_depth);
        }
    }
}

//help function that explains how to use the commands
pub fn help() {
    println!("Commands: ('{}' means the command is implemented '{}' means it's not)", "Violet".truecolor(80, 16, 94), "Red".truecolor(255, 0, 80));
    println!("{}  -  scans the C: drive for files", "scan".truecolor(80, 16, 94));
    println!("{} -  displays where the nothing.exe is curently located", "where".truecolor(80, 16, 94));
    println!("{}  -  finds a file in the scanned files", "find".truecolor(80, 16, 94));
    println!("{}  -  displays the contents of a directory", "tree".truecolor(80, 16, 94));
    println!("{}  -  displays this help message", "help".truecolor(80, 16, 94));
    println!("{} -  clears the screen", "clear".truecolor(80, 16, 94));
    println!("{}  -  displays the cookies from the browser", "cookies".truecolor(80, 16, 94));
    println!("{}  -  exits the program", "exit".truecolor(80, 16, 94));
}

// Get cookies from the browser
pub fn cookies() {

}