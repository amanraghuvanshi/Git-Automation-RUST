use std::process::{Command, exit};
use names:Generator;

fn update_commit_push(){
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .output("Failed to execute the git add command")
    
    if !add_command.status.success(){
        eprintln!("Error: Failed to add the files git repo!")
        exit(1);
    }
    
    
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("failed to execute the git commit command")

    if !commit_command.status.success(){
        eprintln!("Error: Failed to commit the changed git repo!")
        exit(1);
    }
    
    let push_command
}