use std::env;
use std::io::{self, Write};

fn main() {
    let mut stderr = io::stderr();
    let mut stdout = io::stdout();
    let args: Vec<String> = env::args().collect();

    let hook = &args[1];

    if hook == "echo" {
        let _ = write!(stderr, "▙ "); 
        let _ = stderr.flush();
        let arg = &args[2];
        let _ = write!(stdout, "{arg}\n\n");
        let _ = stdout.flush();
    } else if hook == "cd" {
        let _ = write!(stderr, "▚ "); 
        let _ = stderr.flush();
        let arg = &args[2];
        let _ = write!(stdout, "{arg}");
        match env::set_current_dir(&arg) {
            Ok(_) => {}
            Err(_) => std::process::exit(1)
        }
        let path = env::current_dir().expect("Failed to get current directory");
        let _ = write!(stderr, "{}\n\n", path.display());
     } else if hook == "exit" {
        let exit_code: i32 = args[2].parse().unwrap();
        let _ = write!(stderr, "\n█\n"); 
        let _ = stderr.flush(); 
        std::process::exit(exit_code);
    } else if hook == "pwd" {
        let _ = write!(stderr, "▞ "); 
        let _ = stderr.flush();
        let path = env::current_dir().expect("Failed to get current directory");
        let _ = write!(stdout, "{}\n\n", path.display());
        let _ = stdout.flush();
    } else if hook == "read" {
        let _ = write!(stderr, "▌ "); 
        let _ = stderr.flush();
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                std::process::exit(1);
            }
        }
        let _ = write!(stdout, "{}", input.trim_end());
        let _ = stdout.flush();
        let _ = write!(stderr, "\n");
        let _ = stderr.flush();
    }
}

