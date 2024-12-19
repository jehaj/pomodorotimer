
/// Personal pomodoro timer main function


use std::process;
use clap::Parser;
use pomodorotimer::commands::CommandStrings;
use pomodorotimer::pomodoro_timer::PomodoroTimer;

#[derive(Parser)]
struct Cli {
    /// The command to execute to look for
    command: String,
}


fn main() {
    let args = Cli::parse();

    println!("command to run: {:?}", args.command);

    let mut timer = PomodoroTimer::new(1200, 300);

    // Switch on the valid commands
    let command_to_run = match args.command.as_str() {
        "StartSession" => Some(CommandStrings::StartSession),
        "StopSession" => Some(CommandStrings::StopSession),
        "start" => Some(CommandStrings::StartTimer),
        "stop" => Some(CommandStrings::StopTimer),
        "pause" => Some(CommandStrings::PauseTimer),
        _ => {None}
    };

    // Issue error when command not found
    if command_to_run.is_none() {
        eprintln!("Command not recognized");
        process::exit(1);
    }

    // Run the command
    let command_to_run = command_to_run.unwrap();

    match command_to_run {
        CommandStrings::StartSession => {todo!("Start a session")}
        CommandStrings::StopSession => {todo!("End a session")}
        CommandStrings::StartTimer => {timer.start_run()}
        CommandStrings::StopTimer => {todo!("stop a session")}
        CommandStrings::PauseTimer => {todo!("pause a session")}
    }

}
