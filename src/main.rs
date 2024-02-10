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
    
    // remote repo
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("failed to execute the git push command")

    if !push_command.status.success(){
        eprintln!("Error: Failed to push the files changes to remote!")
        exit(1);
    }

    println!("Success in adding, committing and pushing all the changes")
}

fn name_generator() -> String{
    let mut generator = Generator::default();
    generatpr.next().unwrap()
}

fn main(){
    update_commit_push();
}