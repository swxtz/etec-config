mod browser;
mod git;
mod input;

use std::io;
use crate::browser::open_in_browser;
use crate::git::run_git_command;
use colored::Colorize;
use input::read_input;

fn main() {
    greeting();

    println!("{}", "Digite seu nome para o git\n".cyan());

    let mut inputs: Vec<String> = Vec::new();

    match read_input("nome") {
        Ok(name) => {
            run_git_command(["config", "--global", "user.name", &*name].to_vec())
                .expect("Erro ao rodar commando do git para configurar nome");
            inputs.push(name);
            println!("\n------------------------\n");
        }
        Err(e) => panic!("{}", e),
    };

    println!("{}", "Digite seu Email para o git\n".cyan(),);

    match read_input("email") {
        Ok(email) => {
            run_git_command(["config", "--global", "user.email", &*email].to_vec())
                .expect("Erro ao rodar commando do git para configurar email");
            inputs.push(email);
            println!("\n------------------------\n");
        }
        Err(e) => panic!("{}", e),
    };

    show_resume(inputs);

    // set default branch to main
    run_git_command(["config", "--global", "init.defaultBranch", "main"].to_vec()).expect("Erro ao mudar configuração de branch no git config --global");

    //set

    println!("{}", "Pressione ENTER para continuar ... ".yellow());

    let mut confimation = String::new();
    let _ = io::stdin().read_line(&mut confimation);

    download_links();
}

fn greeting() {
    println!("{}", "------------------------\n".green());
    println!("{}", "| Configurador de PC's |\n".cyan());
    println!("{}", "------------------------ \n\n".green());
}

fn download_links() {
    let urls = [
        "https://github.com/cli/cli/releases/download/v2.45.0/gh_2.45.0_windows_amd64.msi",
        "https://github.com/coreybutler/nvm-windows/releases/download/1.1.12/nvm-setup.exe",
        "https://download.oracle.com/java/21/latest/jdk-21_windows-x64_bin.msi",
        "https://download.jetbrains.com/idea/ideaIC-2023.3.5.exe",
    ];

    for link in urls.iter() {
        open_in_browser(link);
    }
}

fn show_resume(infos: Vec<String>) {
    println!("{}", "Configuração do git atualizada para: \n".green());

    println!(
        "{} {}",
        "Nome:".cyan(),
        infos.clone().into_iter().nth(0).unwrap().to_string().cyan(),
    );
    println!(
        "{} {}",
        "Email:".cyan(),
        infos.into_iter().nth(1).unwrap().to_string().cyan(),
    )
}
