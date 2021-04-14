use std::{env, io};
use std::io::Write;
use std::path::Path;
use std::ffi::{CString, CStr};
use nix::unistd::{fork, ForkResult, execvp};
use nix::sys::wait::waitpid;

fn doCommand(arguments:Vec<String>) -> String {
    let mut output:String = "".to_string();
    let length = arguments.len();
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            waitpid(child, None);
        }
        Ok(ForkResult::Child) => {
            let argument= CString::new(arguments[0].as_str()).unwrap();
            let mut inputs = Vec::new();
            inputs.push(CString::new("").unwrap());
            for a in 1..length {
                let next = CString::new(arguments[a].as_str()).unwrap();
                inputs.push(next);
            }
            let out = execvp(argument.as_c_str(), &inputs);
            for ins in inputs {
                println!("{} <- input", ins.to_str().unwrap());
            }
            output = out.unwrap().to_string();
        },
        Err(_) => println!("Fork failed"),
    }
    return output;
}

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
    if input.ends_with("\n") {
        input.pop();
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
            arguments.push(arg.trim().to_string());
        }
        let length = arguments.len();
        match arguments[0].to_lowercase().as_str() {
             "exit" => {
                  exit = true;
             }
            "cd" => {
                let path = env::current_dir().unwrap();
                let mut to = path.display().to_string();
                to.push('/');
                to.push_str(arguments[1].as_str());
                let root = Path::new(&to);
                assert!(env::set_current_dir(&root).is_ok());
            }
             _ => {
                doCommand(arguments);
             }
        }
    }
}
