use std::io::{self, Write};

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
                let output_res = std::process::Command::new(&cmd.seq[0][0])
                    .args(&cmd.seq[0][1..])
                    .output();
                let output;
                match output_res {
                    Ok(output_ok) => output = output_ok,
                    Err(e) => {
                        eprintln!("minishell: {}", e);
                        continue;
                    }
                }
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