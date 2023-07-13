use minirshell::readcmd;
use std::io::{self, Write};

fn main() {
    print!("> ");
    io::stdout().flush().unwrap();
    let cmd = readcmd::rs_readcmd();
    dbg!(&cmd);
}
