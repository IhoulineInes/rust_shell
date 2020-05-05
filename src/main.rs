use std::io::{self,Write};
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    //ECRITURE SUR LA STDOUT
    let stdout = io::stdout();
    let stdin = io::stdin();
    loop{
    {
        let mut handle = stdout.lock(); // STDOUT locked
        handle.write_all(b"-->")?; // locked
        handle.flush()? // locked
    } // STDOUT unlocked

    let _stdin = io::stdin();
    let mut user_input = String::with_capacity(256);
    //On prend une référence mutable
    // LECTURE sur la STDIN
    stdin.read_line(&mut user_input)?;
    let user_input = user_input.trim_end();


    
    //-----------------------//
    // Executer une commande //
    //-----------------------//


  
    Command::new(user_input).status()?;

    let ls = Command::new("ls")
        .arg("alh")
        .stdout(Stdio::piped())
        .spawn()
        .expect("I was pancaked while trying to launch ls.");

    let process = Command::new("wc")
        .arg("--lines")
        .stdin(Stdio::from(ls.stdout.expect("Sommething wrong with ls stdin")))
        .spawn()
        .expect("whopsiel wc failled to launch");
    }
    //? sert à " propager l'erreur"
    //c'est mieux que de crash avec un unwrap ou expert
    Ok(())
}