use std::io::{self, Write, stdout, BufRead, BufReader};
use std::{fs::{File, self}, env, process::{Command}};
use std::path::{Path};
use colored::*;
use sysinfo::{ProcessExt, System, SystemExt, UserExt, DiskExt};
use anyhow:: Result;

pub fn help() {
  println!("Commands: ('{}' means the command works '{}' means it's not and {} means it partially works)", "Red".truecolor(255, 0, 80), "Violet".truecolor(80, 16, 94), "Yellow".truecolor(200, 220, 0));
  println!();
  println!("    {}      -     displays this help message", "help".truecolor(255, 0, 80));
  println!("    {}      -     exits the program", "exit".truecolor(255, 0, 80));
  println!("    {}     -     clears the screen", "clear".truecolor(255, 0, 80));
  println!("    {}   -     get the cookies from the browser", "cookies".truecolor(80, 16, 94));
  println!("    {}   -     encrypts or decrypts the specified file ", "encrypt".truecolor(200, 220, 0));
  println!("    {}      -     finds a file in the scanned files", "find".truecolor(255, 0, 80));
  println!("    {}    -     removes the specified file / folder", "remove".truecolor(200, 220, 0));
  println!("    {}      -     scans the C: drive for files", "scan".truecolor(255, 0, 80));
  println!("    {}      -     displays the contents of a directory", "tree".truecolor(255, 0, 80));
  println!("    {}     -     displays where the nothing.exe is curently located", "where".truecolor(255, 0, 80));
  println!("    {}      -     get info about the target computer", "info".truecolor(255, 0, 80));
}

pub fn remove() {
  println!("Enter path to directory/file to remove:");
  print!("{}", "     rm > ".truecolor(120, 120, 120));
  io::stdout().flush().unwrap();
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let input = input.trim();
  println!();   
  if let Err(e) = fs::remove_dir_all(input) {
    println!("        Error: {}", e);
  } else {
    println!("        Removed: {}", input);
  }
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
        print!("{}", "     scan > ".truecolor(120, 120, 120));
        io::stdout().flush().unwrap();
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
    //ask the user for the file to search for
    println!("Enter the file to search for:");
     print!("{}", "     find > ".truecolor(120, 120, 120));
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input = input.trim();

    if std::path::Path::new("C:\\files\\files.txt").exists() {
        let file = File::open("C:\\files\\files.txt").unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            if line.contains(input) {
                println!("{}", line);
            }
        }
    } else {
        println!("        {} Please run 'scan' first!", "'files.txt' not found!".truecolor(255, 0, 0));
        println!("        Do you want to run 'scan' now? (y/n)");
        print!("{}", "     scan > ".truecolor(120, 120, 120));
        io::stdout().flush().unwrap();
    }
}

pub fn tree() {
    // Ask the user for a directory to search in
    println!("Enter the directory to search in:");
    print!("{}", "     tree > ".truecolor(120, 120, 120));
    io::stdout().flush().unwrap();
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

/*
pub fn cookies() -> Result<(), Box<dyn std::error::Error>> {
    // ask the user for the browser to search from
    println!("Enter the browser to search from:");
    print!("{}", "     cookies > ".truecolor(120, 120, 120));
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input = input.trim();

    let mut parts = input.splitn(2, ' ');
    let command = parts.next().unwrap_or("");
    
    match command {
        "firefox" => {
            // Get the cookies from Firefox
            // create a file to save the cookies to
            File::create("cookies.txt").unwrap();
            let file_path = "cookies.txt";
            println!("Cookies saved to {}", file_path);
        }
        _ => {
            println!("        {} Please enter a valid browser!", "Invalid browser!".truecolor(255, 0, 0));
        }
    }
    Ok(())
}
*/

pub fn info() {
    loop {
        println!("        What info do you want to see? ('os', 'memory / mem', 'disks', 'processes / procs', 'users', '*', type 'back' to go back to the main menu)");
        print!("{}", "     info > ".truecolor(120, 120, 120));
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");
        let input = input.trim();

        match input {
            "os" => {
                sysinfo();
            }
            "memory" | "mem" => {
                sysmem();
            }
            "disks" => {
                sysdisks();
            }
            "processes" | "procs" => {
                sysprocs();
            }
            "users" => {
                sysusers();
            }
            "all" | "*" => {
                sysall();
            }
            "back" | "exit" | "quit" => {
                break;
            }
            _ => {
                println!("        Invalid input");
            }
        }
    }
}

fn sysall() {
    // Run the "systeminfo" command and capture the output
    let output = Command::new("systeminfo")
        .output()
        .expect("Failed to run systeminfo command");

    // Convert the output to a string
    let output_string = String::from_utf8(output.stdout).expect("Failed to convert output to string");

    // Split the output into lines
    let lines: Vec<&str> = output_string.split('\n').collect();
    
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // display user name using `users` method but only the current username:
    println!("        => users:");
    for user in sys.users() {
        println!("        {}", user.name());
    }
    println!();

    // We display all disks' information:
    println!("        => disks:");
    for disk in sys.disks() {
        println!("        {:?} {:?}", disk.name(), disk.mount_point());
    }
    println!();

    println!("        => system:");
    // Find the "Total Memory" line
    let totalmem_line = lines.iter().find(|line| line.starts_with("Total Physical Memory")).expect("Host Name not found");
    let freemem_line = lines.iter().find(|line| line.starts_with("Available Physical Memory")).expect("Host Name not found");

    // Extract the total memory from the line
    let totalmem = totalmem_line.split(':').nth(1).expect("Failed to extract host name");
    let freemem = freemem_line.split(':').nth(1).expect("Failed to extract host name");

    println!("        Total Memory: {}", totalmem);
    println!("        Available Memory: {}", freemem);
    println!();

    // Find the "OS Name" and "OS Version" lines
    let os_name_line = lines.iter().find(|line| line.starts_with("OS Name")).expect("OS Name not found");
    let os_version_line = lines.iter().find(|line| line.starts_with("OS Version")).expect("OS Version not found");

    // Extract the OS name and version from the lines
    let os_name = os_name_line.split(':').nth(1).expect("Failed to extract OS name");
    let os_version = os_version_line.split(':').nth(1).expect("Failed to extract OS version");

    // Find the "Host Name" line
    let host_name_line = lines.iter().find(|line| line.starts_with("Host Name")).expect("Host Name not found");

    // Extract the host name from the line
    let host_name = host_name_line.split(':').nth(1).expect("Failed to extract host name");

    // Print the OS name, version, and host name
    println!("        OS Name: {}", os_name);
    println!("        OS Version: {}", os_version);
    println!("        Host Name: {}", host_name);

    // Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        println!("        [{}]    {}", pid, process.name());
    }
}

fn sysmem() {
    // Run the "systeminfo" command and capture the output
    let output = Command::new("systeminfo")
        .output()
        .expect("Failed to run systeminfo command");

    // Convert the output to a string
    let output_string = String::from_utf8(output.stdout).expect("Failed to convert output to string");

    // Split the output into lines
    let lines: Vec<&str> = output_string.split('\n').collect();

    // Find the "Total Memory" line
    let totalmem_line = lines.iter().find(|line| line.starts_with("Total Physical Memory")).expect("Host Name not found");
    let freemem_line = lines.iter().find(|line| line.starts_with("Available Physical Memory")).expect("Host Name not found");

    // Extract the total memory from the line
    let totalmem = totalmem_line.split(':').nth(1).expect("Failed to extract host name");
    let freemem = freemem_line.split(':').nth(1).expect("Failed to extract host name");

    println!("        Total Memory: {}", totalmem);
    println!("        Available Memory: {}", freemem);
}

fn sysusers() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("=> users:");
    for user in sys.users() {
        println!("{} is in {} groups", user.name(), user.groups().len());
    }
}

fn sysdisks() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("        => disks:");
    for disk in sys.disks() {
        println!("        {:?} {:?}", disk.name(), disk.mount_point());
    }
}

fn sysinfo() {
    // Run the "systeminfo" command and capture the output
    let output = Command::new("systeminfo")
        .output()
        .expect("Failed to run systeminfo command");

    // Convert the output to a string
    let output_string = String::from_utf8(output.stdout).expect("Failed to convert output to string");

    // Split the output into lines
    let lines: Vec<&str> = output_string.split('\n').collect();

    // Find the "OS Name" and "OS Version" lines
    let os_name_line = lines.iter().find(|line| line.starts_with("OS Name")).expect("OS Name not found");
    let os_version_line = lines.iter().find(|line| line.starts_with("OS Version")).expect("OS Version not found");

    // Extract the OS name and version from the lines
    let os_name = os_name_line.split(':').nth(1).expect("Failed to extract OS name");
    let os_version = os_version_line.split(':').nth(1).expect("Failed to extract OS version");

    // Find the "Host Name" line
    let host_name_line = lines.iter().find(|line| line.starts_with("Host Name")).expect("Host Name not found");

    // Extract the host name from the line
    let host_name = host_name_line.split(':').nth(1).expect("Failed to extract host name");

    // Print the OS name, version, and host name
    println!("        OS Name: {}", os_name);
    println!("        OS Version: {}", os_version);
    println!("        Host Name: {}", host_name);
}

fn sysprocs() {
    let mut sys = System::new_all();

    sys.refresh_all();

    for (pid, process) in sys.processes() {
        println!("        [{}]    {}    {}", pid, process.name(), process.exe().display());
    }
}

pub fn kill() {
    //ask the user for the process to kill by its PID
    println!("        Enter the PID of the process to kill:");
    print!("{}", "     kill > ".truecolor(120, 120, 120));
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input = input.trim();

    //convert the input to a u32
    let pid = match input.parse::<u32>() {
        Ok(pid) => pid,
        Err(e) => {
            println!("        Error parsing PID: {}", e);
            return;
        }
    };

    //kill the process using the easiest way possible
    match Command::new("taskkill").arg("/PID").arg(pid.to_string()).arg("/F").output() {
        Ok(_) => {
            println!("        Process killed successfully.");
        }
        Err(e) => {
            println!("        Error killing process: {}", e);
        }
    }
}

pub fn disable() {
    // Get the path of the current executable
    let exe_path = env::current_exe().unwrap();

    // Delete the current executable
    std::fs::remove_file(exe_path).unwrap();
}

pub fn elevate() {
    
}
