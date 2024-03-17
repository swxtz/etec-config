mod browser;
mod git;
mod input;

use crate::browser::open_in_browser;
use crate::git::run_git_command;
use input::read_input;

fn main() {
    greeting();

    println!("Digite seu nome para o git\n");

    match read_input("nome") {
        Ok(name) => {
            run_git_command(["config", "--global", "user.name", &*name].to_vec())
                .expect("Erro ao rodar commando do git para configurar nome");
            println!("\n------------------------\n");
        }
        Err(e) => panic!("{}", e),
    };

    println!("Digite seu Email para o git\n");

    match read_input("email") {
        Ok(email) => open_in_browser("https://youtube.com"),
        Err(e) => panic!("{}", e),
    };
}

fn greeting() {
    println!("------------------------\n");
    println!("| Configurador de PC's |\n");
    println!("------------------------\n\n");
}
