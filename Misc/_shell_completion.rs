// src/bin/__democli_shell_completion.rs
fn main() {
    let subcommands = vec!["add", "commit"];                                                                       
    let args : Vec<String> = std::env::args().collect();                                                           
    let word_being_completed = &args[2];                                                                           
    for subcommand in subcommands { 
       if subcommand.starts_with(word_being_completed) {
           println!("{}", subcommand);
       }
    }   
} 
