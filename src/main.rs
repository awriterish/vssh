use std::{env, io};
use std::io::Write;
use std::path::Path;
use std::ffi::CString;
use nix::unistd::{fork, ForkResult, execvp};
use nix::sys::wait::waitpid;

fn printPath() -> std::io::Result<()> {
    let path = env::current_dir()?;
    print!("{}$ ", path.display());
    Ok(())
}

fn readLine() -> String {
    printPath();
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(..) => {

        }
        Err(error) => println!("error: {}", error),
    }
    return input;
}

fn main() {
    let mut exit = false;
    while exit == false {
        let input = readLine();
        let args: Vec<&str> = input.split(" ").collect();
        let mut arguments: Vec<String> = Vec::new();
        for arg in args {
            arguments.push(arg.to_string());
        }
        let length = arguments.len();
        arguments[length-1].pop();
        arguments[length-1].pop();

        if length > 1 {

        }
        match arguments[0].to_lowercase().as_str() {
             "exit" => {
                  exit = true;
             }
            "cd" => {
                let path = env::current_dir().unwrap();
                let mut to = path.display().to_string();
                to.push('\\');
                to.push_str(arguments[1].as_str());
                let root = Path::new(&to);
                assert!(env::set_current_dir(&root).is_ok());
            }
             _ => {
                 match unsafe{fork()} {
                     Ok(ForkResult::Parent { child, .. }) => {
                         waitpid(child);
                     }
                     Ok(ForkResult::Child) => {
                         let argument= CString::new(arguments[0].as_str()).unwrap();
                         execvp(argument.as_c_str(), &arguments[1..length]);
                     },
                     Err(_) => println!("Fork failed"),
                 }
             }
        }
    }
}
