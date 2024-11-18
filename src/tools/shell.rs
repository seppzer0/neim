use std::process::Command;

fn run_command(command: &str) -> String {
    let args: Vec<&str> = command.split(" ").collect();
    let output = Command::new(args[0])
        .args(&args[1..])
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.to_string()
        },
        Err(error) => {
            println!("Command failed: {command}");
            eprintln!("error: {}", error);
            "".to_string()
        }
    }
}
