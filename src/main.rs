use minirshell::readcmd;

fn main() {
    let _cmd = readcmd::rs_readcmd();
    dbg!(&_cmd);
}
