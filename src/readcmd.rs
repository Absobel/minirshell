use core::ffi::{c_char, CStr};

#[repr(C)]
#[derive(Debug)]
struct CmdLine {
    err: *mut c_char,
    fin: *mut c_char,
    fout: *mut c_char,
    bg: *mut c_char,
    seq: *mut *mut *mut c_char,
}

extern "C" {
    fn readcmd() -> *mut CmdLine;
}

#[derive(Debug)]
pub struct Commande {
    err: Option<String>,
    fin: Option<String>,
    fout: Option<String>,
    bg: Option<String>,
    seq: Option<Vec<Vec<String>>>,
}

fn cstr_to_some_string(c_str: *mut c_char) -> Option<String> {
    if c_str.is_null() {
        return None;
    }
    let c_str: &CStr = unsafe { CStr::from_ptr(c_str) };
    Some(c_str.to_str().expect("Should be valid UTF-8").to_string())
}

pub fn rs_readcmd() -> Option<Commande> {
    let cmd = unsafe { readcmd() };

    let mut command = Commande {
        err: None,
        fin: None,
        fout: None,
        bg: None,
        seq: None,
    };

    if cmd.is_null() {
        return None;
    }

    let cmd: &CmdLine = unsafe { &*cmd };

    command.err = cstr_to_some_string(cmd.err);
    command.fin = cstr_to_some_string(cmd.fin);
    command.fout = cstr_to_some_string(cmd.fout);
    command.bg = cstr_to_some_string(cmd.bg);

    let seq = cmd.seq;
    if !seq.is_null() {
        let mut seq_vec = Vec::new();

        let mut i = 0;
        // while cmd.seq[i] != NULL
        while unsafe { !(*cmd.seq.offset(i)).is_null() } {
            let cmd_i = unsafe { *cmd.seq.offset(i) };
            let mut cmd_vec = Vec::new();
            let mut j = 0;
            // while cmd.seq[i][j] != NULL
            while unsafe { !(*cmd_i.offset(j)).is_null() } {
                let arg_j = unsafe { *cmd_i.offset(j) };
                cmd_vec.push(cstr_to_some_string(arg_j).unwrap()); // would return None if argj was null which it can't because of the while condition
                j += 1;
            }
            seq_vec.push(cmd_vec);
            i += 1;
        }
        command.seq = Some(seq_vec);
    }

    Some(command)
}
