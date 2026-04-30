use std::{
    env,
    path::PathBuf,
    process::{Command, Output},
};

fn main() {
    let mut args: Vec<String> = env::args().collect(); // cats, arg1, arg2, etc

    if args.len() == 1 {
        println!("meow");
        return;
    }

    args.remove(0);

    if args[0] == "--help" || args[0] == "-h" {
        printhelp();
        return;
    } else if args[0] == "--cat-help" {
        runcmd(vec!["--help".to_string()], "cat");
    } else if args[0] == "--ls-help" {
        runcmd(vec!["--help".to_string()], "ls");
    }

    let mut path = PathBuf::from(&args[0]);
    for arg in &args {
        let prefix = match arg.chars().next() {
            Some(c) => c,
            None => 'x',
        };

        if prefix != '-' {
            path = PathBuf::from(arg);
            break;
        }
    }

    if path.is_file() {
        runcmd(args, "cat");
    } else {
        runcmd(args, "ls");
    }

    // println!("CATS: path: {:?}", path);
}

fn runcmd(args: Vec<String>, cmd: &str) -> () {
    println!("{}: ", cmd);

    let mut command: Vec<String> = vec![cmd.to_string()];

    command.extend(args);

    // println!("CATS: running command: {:?}", command);

    let mut cmd = Command::new(&command[0]);
    cmd.args(&command[1..]);

    match cmd.output() {
        Ok(o) => printcommandoutput(o),
        Err(_) => println!("failed to execute command {}", &command.join(" ")),
    }
}

fn printcommandoutput(output: Output) -> () {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if stdout.trim().is_empty() {
        for line in stderr.lines() {
            println!("{}", line);
        }
    } else if stderr.trim().is_empty() {
        for line in stdout.lines() {
            println!("{}", line);
        }
    } else {
        println!("no output")
    }
}

fn printhelp() -> () {
    println!("cats");
    println!("command line utility to combine both cat and ls");
    println!("args:");
    println!("  --help\t\tprints this");
    println!("  --cat-help\t\truns cat --help (idk why ud use this)");
    println!("  --ls-help\t\truns ls --help (??)");
}
