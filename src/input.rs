use std::io::stdin;
use termion::color;
use termion::color::Fg;

pub fn read_input(label: &str) -> Result<String, String> {
    let mut line = String::new();
    stdin()
        .read_line(&mut line)
        .expect("Erro ao ler input do usuario");
    if line.len() <= 4 {
        return Err(format!(
            "{}O {} deve ter no minino 3 caracteres{}",
            Fg(color::Red),
            label,
            Fg(color::Reset)
        ));
    }

    return Ok(line);
}
