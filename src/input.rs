use colored::Colorize;
use std::io::stdin;

pub fn read_input(label: &str) -> Result<String, String> {
    let mut line = String::new();
    stdin()
        .read_line(&mut line)
        .expect("Erro ao ler input do usuario");
    if line.len() <= 4 {
        return Err(format!(
            "{} {} {}",
            "{} {}".red(),
            "Deve ter no minino 3 caracteres no campo".red(),
            label,
        ));
    }

    return Ok(line);
}
