use minirshell::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("minishell: {}", e);
    }
}