use whoami;
use std::env;
use std::fs;
use regex::Regex;
use std::process::Command;

pub fn antivm() {
    // Check the MAC address
    let pattern = Regex::new(r"^08:00:27|00:50:56|00:1C:14|00:0C:29|00:05:69").unwrap();
    let mac_address = get_mac_address().unwrap_or_else(|_| "".to_string());
    if pattern.is_match(&mac_address) {
        println!("Running in VirtualBox/VMware (MAC address: {})", mac_address);
        std::process::exit(0);
    }

    // Windows Sandbox
    let user = whoami::username();
    if user == "WDAGUtilityAccount" {
        println!("Running in Windows Sandbox");
        std::process::exit(0);
    }

    // Check the environment variables
    if let Ok(version) = env::var("VBOX_VERSION") {
        println!("Running in VirtualBox virtual machine (version: {})", version);
        std::process::exit(0);
    }

    // Check for VirtualBox-specific files or directories
    if fs::metadata("/VirtualBox").is_ok() || fs::metadata("/proc/vbox").is_ok() {
        println!("Running in VirtualBox");
        std::process::exit(0);
    }

    // Check for VMware-specific files or directories
    if std::path::Path::new("C:\\Program Files\\VMware\\VMware Tools\\vmtoolsd.exe").exists() {
        println!("Running in VMware");
        std::process::exit(0);
    }   else if std::path::Path::new("C:\\Program Files\\VMware").exists() {
        println!("Running in VMware");
        std::process::exit(0); 
    }

    // Check for VirtualBox-specific system calls
    let system_info = get_system_info().unwrap_or_else(|_| "".to_string());
    if system_info.contains("VirtualBox") || system_info.contains("VMware") {
        std::process::exit(0);
    } else {
        println!("Not running in VirtualBox/VMware");
    }
}

fn get_value(key: &str, output: &str) -> String {
    let lines: Vec<&str> = output.split("\n").collect();
    for line in lines {
        if line.starts_with(key) {
            let parts: Vec<&str> = line.split(":").collect();
            if parts.len() >= 2 {
                return parts[1].trim().to_string();
            }
        }
    }
    "".to_string()
}

fn get_system_info() -> Result<String, Box<dyn std::error::Error>> {
    // Run the "systeminfo" command and capture the output
    let output = Command::new("systeminfo")
        .output()
        .expect("Failed to run systeminfo command");
    let output_str = String::from_utf8_lossy(&output.stdout);

    let bios_version = get_value("BIOS Version", &output_str);
    let system_manufacturer = get_value("System Manufacturer", &output_str);
    let system_model = get_value("System Model", &output_str);

    let system_info = format!("{} {} {}", bios_version, system_manufacturer, system_model);
    Ok(system_info)
}

fn get_mac_address() -> Result<String, Box<dyn std::error::Error>> {
    // Use the wmic command to list the network adapters and filter the output
    // to show only the adapter that is currently connected
    let output = Command::new("wmic")
        .arg("nic")
        .arg("where")
        .arg("NetConnectionStatus=2")
        .arg("get")
        .arg("MACAddress")
        .output()?;

    // Convert the output to a string and split the string into lines
    let mac_address = String::from_utf8(output.stdout)?;
    let lines: Vec<&str> = mac_address.split_terminator('\n').collect();

    // Return the first line as the MAC address
    Ok(lines[1].to_string())
}
