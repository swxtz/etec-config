use std::process::Command;

pub fn run_git_command(args: Vec<&str>) -> Result<(), std::io::Error> {
    let tool = "git";
    let mut command = Command::new(&tool);

    command.args(args);

    let command_status = command.status();

    if command_status.is_ok() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Erro ao executar o comando: {:?}", command_status),
        ))
    }
}
