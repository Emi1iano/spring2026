use std::{io::{self, Write}, process::Command};

fn perform_operation(operation: FileOperation) {
    // Implement command execution based on the operation
    match operation {
        FileOperation::List(string) => {
            Command::new("ls").arg(string.trim()).status().expect("Failed to execute ls");
        }
        FileOperation::Display(string) => {
            print!("\n[Output of cat command showing \"");
            let _ = io::stdout().flush();
            Command::new("cat").arg(string.trim()).status().expect("Failed to execute cat");
            print!("\"]\n\n");
            let _ = io::stdout().flush();
        }
        FileOperation::Create(string1, string2) => {
            let command = format!("echo -n '{}' > {}", string2.trim(), string1);
            Command::new("sh").arg("-c").arg(command).status().expect("Failed to create file");
        }
        FileOperation::Remove(string) => {
            Command::new("rm").arg(string.trim()).status().expect("Failed to remove file");
        }
        FileOperation::Pwd => {
            print!("\nCurrent working directory: ");
            let _ = io::stdout().flush();
            Command::new("pwd").status().expect("Failed to execute pwd");
            println!("");
            let _ = io::stdout().flush();
        }
    }
}
enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

fn main() {
    loop {
        println!("File Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit\n");
        print!("Enter your choice (0-5): ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failure to read terminal Input.");

        let num = input.trim().parse::<u16>().expect("Failed to parse number from terminal input.");
        match num {
            0 => {
                println!("Goodbye!");
                break;
            }
            1 => {
                //let path = std::env::current_dir().expect("Error");
                let mut string = String::new();
                print!("Enter directory path: ");
                let _ = io::stdout().flush();
                io::stdin().read_line(&mut string).expect("Failure to read terminal Input.");

                println!("");
                let _ = io::stdout().flush();
                

                perform_operation(FileOperation::List(string.clone()));
                println!("");
                let _ = io::stdout().flush();
            }
            2 => {
                let mut string = String::new();
                print!("Enter file path: ");
                let _ = io::stdout().flush();
                io::stdin().read_line(&mut string).expect("Failure to read terminal Input.");
                
                perform_operation(FileOperation::Display(string.clone()));
            }
            3 => {
                let mut string = String::new();
                print!("Enter file path: ");
                let _ = io::stdout().flush();
                io::stdin().read_line(&mut string).expect("Failure to read terminal Input.");

                let mut content = String::new();
                print!("Enter content: ");
                let _ = io::stdout().flush();
                io::stdin().read_line(&mut content).expect("Failure to read terminal Input.");

                perform_operation(FileOperation::Create(string.clone(), content.clone()));

                print!("\nFile '{}' created successfully.\n\n", string.trim());
                let _ = io::stdout().flush();
            }
            4 => {
                let mut string = String::new();
                print!("Enter file path: ");
                let _ = io::stdout().flush();
                io::stdin().read_line(&mut string).expect("Failure to read terminal Input.");
                
                perform_operation(FileOperation::Remove(string.clone()));

                print!("\nFile '{}' removed successfully.\n\n", string.trim());
                let _ = io::stdout().flush();
            }
            5 => {
                perform_operation(FileOperation::Pwd);
            }
            _ => {
                continue;
            }
        }
    }
    
}
