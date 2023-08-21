pub enum Commands {
    Init,
    Install,
    Compile,
    Run,
    Help
}

pub fn get_command() -> Commands {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        return Commands::Help;
    }
    let command = args[1].as_str();

    return match command {
        "init" => Commands::Init,
        "install" => Commands::Install,
        "compile" => Commands::Compile,
        "run" => Commands::Run,
        _ => Commands::Help
    };

}

