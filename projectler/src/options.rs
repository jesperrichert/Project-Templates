use fancy::printcoln;
use std::process::Command;

pub struct Options {
    run_open_command: bool,
    run_install_command: bool,
}

impl Options {
    pub fn new(run_open_command: bool, run_install_command: bool) -> Self {
        Options {
            run_open_command,
            run_install_command,
        }
    }

    pub fn run_open_command(&self, command: &String, path: &String) {
        if !self.run_open_command {
            return;
        };

        printcoln!("[bold|white]>> Run open command: {}", command);

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .current_dir(path)
                .arg("/C")
                .arg(command)
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("bash")
                .current_dir(path)
                .arg("-c")
                .arg(command)
                .output()
                .expect("failed to execute process")
        };

        println!("[");
        printcoln!("[white]{}", String::from_utf8(output.stdout).unwrap());
        println!("]");

        printcoln!("[bold|green]>> Command has been executed successfully.",);
    }

    pub fn run_install_command(&self, command: &String, path: &String) {
        if !self.run_install_command {
            return;
        };

        printcoln!("[bold|white]>> Run install command...",);

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .current_dir(path)
                .arg("/C")
                .arg(command)
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("bash")
                .current_dir(path)
                .arg("-c")
                .arg(command)
                .output()
                .expect("failed to execute process")
        };
        println!("[");
        printcoln!("[white]{}", String::from_utf8(output.stdout).unwrap());
        println!("]");

        printcoln!("[bold|green]>> Successfully installed",);
    }
}
