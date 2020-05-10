use std::io::{self,Write};
use std::process::{Child,Command, Stdio};
use std::env;
use std::path::Path;

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
    
    let mut input_split = user_input.trim().split(" ");
    let _cmd = input_split.next().unwrap();
    let args = input_split;
   
      
   




    if user_input.trim().to_string() == "exit" {
        break
    }
    



    else if user_input.trim().contains("cd"){
        let new_dir = args.peekable().peek().map_or("/", |x| *x);
        let path = Path::new(new_dir);
        if let Err(e) = env::set_current_dir(&path) {
            eprintln!("{}", e);
        }

    }




    else if user_input.trim().contains("|"){
        // Fonction pour ls | grep a par exemple, mais il faut refaire un ls pour avoir le résultat
       
        let mut commands = user_input.trim().split(" | ").peekable();
        let mut previous_command = None;
            

        while let Some(command) = commands.next()  {

            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;
           
            let stdin = previous_command
                .map_or(Stdio::inherit(),
                    |output: Child| Stdio::from(output.stdout.unwrap()));

            let stdout = if commands.peek().is_some() {
                Stdio::piped()
            } else {
                Stdio::inherit()
            };
            let output = Command::new(command)
                .args(args)
                .stdin(stdin)
                .stdout(stdout)
                .spawn();
            match output {
                Ok(output) => { previous_command = Some(output); },
                Err(e) => {
                    previous_command = None;
                    eprintln!("{}", e);
                },
            };
        }
            


    }
// https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
   

    else {
        let mut input_split = user_input.trim().split(" ");
        let cmd = input_split.next().unwrap();
        let args = input_split;
        let cmd = Command::new(cmd)
            .args(args)
            .spawn();

        match cmd {
            Ok(mut cmd) => { cmd.wait()?; },
            Err(e) => eprintln!("{}", e),
        };    
   
     // Pipes 
    let ls = Command::new("ls")
        .stdout(Stdio::piped())
        .spawn()
        .expect("ls ?");

    let _process = Command::new("wc")
        .arg("--lines")
        .stdin(Stdio::from(ls.stdout.expect("Sommething wrong with ls stdin")))
        .spawn()
        .expect("whopsiel wc failled to launch");
    


    }
    

}



    //? sert à " propager l'erreur"
    //c'est mieux que de crash avec un unwrap ou expert
    Ok(())
}