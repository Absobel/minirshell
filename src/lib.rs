use std::{io::{self, Write}, process::Output};

mod readcmd;

pub fn run() -> Result<(), Box<dyn std::error::Error>>{
    loop {
        print!("minirshell> ");
        io::stdout().flush()?;
        let cmd = readcmd::input()?;

        if let Some(cmd) = cmd {
            if let Some(err) = cmd.err {
                eprintln!("minirshell: {}", err);
            } else {
                let output = std::process::Command::new(&cmd.seq[0][0])
                    .args(&cmd.seq[0][1..])
                    .output()?;
                if output.status.success() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    eprintln!("minishell: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
        } else {
            break;
        }
    }
    Ok(())
}