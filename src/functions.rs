use std::io::{self, Write, BufRead, BufReader};
use std::fs;
use std::path::Path;
use std::fs::File;


pub fn remove() {
    //ask the user for the directory and file name to remove
    //if the file name is blank, ask the user if he wants to remove the directory or list every file in the directory
    //if the directory is blank, list all the files with that name in the C:\ScannedFiles\ScanResult.txt

    
}

pub fn whereis() {
    //find the current directory
    let path = std::env::current_dir().unwrap();
    println!("Current directory: {}", path.display());
}

pub fn scan() {
    //create a folder in the C: drive
    let path = Path::new("C:\\ScannedFiles");
    fs::create_dir_all(path).unwrap();
    println!("Created folder: {}", path.display());

    //scan C: drive
    let dir = Path::new("C:\\");
    let mut file = File::create("C:\\ScannedFiles\\ScannedFiles.txt").unwrap();
    scan_dir(&dir, &mut file).unwrap();
}

pub fn scan_dir(dir: &Path, file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    file.write_all(format!("{}\n",path.to_string_lossy()).as_bytes())?;
                } else if path.is_dir() {
                    scan_dir(&path, file)?;
                }
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading directory {}: {}", dir.display(), e);
            Ok(())
        }
    }
}

pub fn find() {
    //ask the user for the file to search for and search it from C:\ScannedFiles\ScannedFiles.txt
    println!("Enter the file to search for:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input = input.trim();

    let file = File::open("C:\\ScannedFiles\\ScannedFiles.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(input) {
            println!("{}", line);
        }
    }
}

pub fn tree_search() {
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

pub fn display_directory_contents(path: &Path, depth: usize, max_depth: usize) {
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
        let indent = "        ".repeat(depth);
        println!("{}{}", indent, entry_path.display());

        // If the entry is a directory, recursively display its contents
        if entry_path.is_dir() {
            display_directory_contents(&entry_path, depth + 1, max_depth);
        }
    }
}
