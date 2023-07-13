fn main() {
    cc::Build::new().file("c_src/readcmd.c").compile("readcmd");
}
