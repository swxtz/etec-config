use std::io::stdin;

pub fn read_input(label: &str) -> Result<String, String> {
    let mut line = String::new();
    stdin()
        .read_line(&mut line)
        .expect("Erro ao ler input do usuario");

    if line.len() < 3 {
        return Err(format!("O {} deve ter no minino 3 caracteres", label));
    }

    return Ok(line);
}
