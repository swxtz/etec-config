mod git;
mod input;

use crate::git::run_git_command;
use input::read_input;

fn main() {
    greeting();

    println!("Digite seu nome para o git");

    match read_input("nome") {
        Ok(name) => run_git_command(["config", "--list"].to_vec())
            .expect("Erro ao rodar commando do git para configurar nome"),
        Err(e) => panic!("{}", e),
    };

    println!("Digite seu Email para o git");

    match read_input("email") {
        Ok(email) => println!("{}", email),
        Err(e) => panic!("{}", e),
    };
}

fn greeting() {
    println!("------------------------\n");
    println!("| Configurador de PC's |\n");
    println!("------------------------\n\n");
}
