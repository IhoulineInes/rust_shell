use std::error::Error;
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

    // recuperer la commande de l'utilisateur

    let _stdin = io::stdin();
    let mut user_input = String::with_capacity(256);

    //On prend une référence mutable
    stdin.read_line(&mut user_input)?;

    if user_input.trim() == "exit" {
        break
    } else if user_input.trim() == "" {
        continue
    }

    
    let mut input_split = user_input.trim().split(" ");


    // Executer une commande 

    let cmd = input_split.next().unwrap();
    let args = input_split;
  
    let mut cmd = Command::new(cmd).args(args).spawn().unwrap();


    


     // Pipes 
    let ls = Command::new("ls")
        .arg("alh")
        .stdout(Stdio::piped())
        .spawn()
        .expect("ls ?");

    let process = Command::new("wc")
        .arg("--lines")
        .stdin(Stdio::from(ls.stdout.expect("Sommething wrong with ls stdin")))
        .spawn()
        .expect("whopsiel wc failled to launch");
    
    cmd.wait();
    
    }
    //? sert à " propager l'erreur"
    //c'est mieux que de crash avec un unwrap ou expert
    Ok(())
}