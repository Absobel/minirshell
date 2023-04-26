use std::io;

pub struct Commande {
    err: Option<String>,      // Erreur de syntaxe
    fin: Option<String>,      // Fichier de redirection d'entrée
    fout: Option<String>,     // Fichier de redirection de sortie
    bg: bool,               // Tâche en arrière plan
    seq: Vec<Vec<String>>,  // Séquence de commandes éventuellement séparées par |
}

pub fn input() -> Result<Option<Commande>, Box<dyn std::error::Error>> {
    let mut command = Commande {
        err: None,
        fin: None,
        fout: None,
        bg: false,
        seq: vec![vec![]],
    };
    
    let mut line = String::new();
    if io::stdin().read_line(&mut line)? == 0 {
        return Ok(None);
    }
    let mut args: Vec<String> = line.split_whitespace().map(String::from).collect();
    if args.is_empty() {
        return Ok(Some(command));
    }

    let mut iter = args.iter().enumerate();
        
    loop {
        match iter.next() {
            Some((i, arg)) => {
                match arg.as_str() {
                    ">" => {
                        if i + 1 >= args.len() {
                            command.err = Some("Missing output file".into());
                            return Ok(Some(command));
                        } else if command.fout.is_some() {
                            command.err = Some("Multiple output redirects".into());
                            return Ok(Some(command));
                        }
                        command.fout = Some(args[i + 1].clone());
                        // Remove the processed output file from args
                        iter.next();
                    }
                    "<" => {
                        if i + 1 >= args.len() {
                            command.err = Some("Missing input file".into());
                            return Ok(Some(command));
                        } else if command.fin.is_some() {
                            command.err = Some("Multiple input redirects".into());
                            return Ok(Some(command));
                        }
                        command.fin = Some(args[i + 1].clone());
                        iter.next(); // Remove the processed input file from args
                    }
                    "|" => {
                        if i + 1 >= args.len() {
                            command.err = Some("Missing command".into());
                            return Ok(Some(command));
                        }
                        command.seq.push(vec![]);
                    }
                    _ => {
                        command.seq.last_mut().ok_or("No last command")?.push(arg.clone());
                    }
                }
            }
            None => break,
        }
    }    

    command.bg = args.last().expect("Should have at least one argument") == "&";

    Ok(Some(command))
}