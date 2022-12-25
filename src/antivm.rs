use whoami;

// if the current users name is WDAGUtilityAccount kill the program as fast as possible
// if the system has vmtools installed kill the program as fast as possible

pub fn kill() {
    let user = whoami::username();
    if user == "WDAGUtilityAccount" {
        std::process::exit(0);
    }

    if std::path::Path::new("C:\\Program Files\\VMware\\VMware Tools\\vmtoolsd.exe").exists() {
        std::process::exit(0);
    }
}