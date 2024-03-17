mod browser;
mod git;
mod input;

use crate::browser::open_in_browser;
use crate::git::run_git_command;
use input::read_input;
use termion::color;
use termion::color::{Cyan, Fg, LightCyan, Reset};

fn main() {
    greeting();

    println!(
        "{}Digite seu nome para o git{}\n",
        Fg(color::Cyan),
        Fg(color::Reset)
    );

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

    println!(
        "{}Digite seu Email para o git{}\n ",
        Fg(color::Cyan),
        Fg(color::Reset)
    );

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
    //run_git_command(["config", "--global", "init.defaultBranch", "main"].to_vec());

    //download_things();
}

fn greeting() {
    println!("{}------------------------\n", Fg(color::Green));
    println!(
        "|{} Configurador de PC's {}|\n",
        Fg(color::Cyan),
        Fg(color::Green)
    );
    println!("------------------------{} \n\n", Fg(Reset));
}

fn download_things() {
    let urls = [
        "https://github.com/cli/cli/releases/download/v2.45.0/gh_2.45.0_windows_amd64.msi",
        "https://github.com/coreybutler/nvm-windows/releases/download/1.1.12/nvm-setup.exe",
        "https://download.oracle.com/java/21/latest/jdk-21_windows-x64_bin.msi",
    ];

    for link in urls.iter() {
        open_in_browser(link);
    }
}

fn show_resume(infos: Vec<String>) {
    println!("Configuração do git atualizada para: \n");

    println!(
        "{}Nome:{} {} {}\n",
        Fg(Cyan),
        Fg(LightCyan),
        infos.clone().into_iter().nth(0).unwrap().to_string(),
        Fg(Reset)
    );
    println!(
        "{}Email:{} {} {}\n",
        Fg(Cyan),
        Fg(LightCyan),
        infos.into_iter().nth(1).unwrap().to_string(),
        Fg(Reset)
    )
}
