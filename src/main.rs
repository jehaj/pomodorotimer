
/// Personal pomodoro timer main function


use std::process;
use std::time::Duration;
use clap::Parser;
use dialoguer::Input;
use pomodorotimer::commands::CommandStrings;
use pomodorotimer::pomodoro_timer::PomodoroTimer;

#[derive(Parser)]
struct Cli {
    /// The command to execute to look for
    command: String,
}

fn main() {
    let args = Cli::parse();

    println!("Pomodoro Timer Starting up");

    // Switch on the valid commands
    let command_to_run = match args.command.as_str() {
        "start" => Some("s"),
        _ => {None}
    };

    // Issue error when command not found
    if command_to_run.is_none() {
        eprintln!("Command not recognized");
        process::exit(1);
    }

    let mut timer = PomodoroTimer::new(20 * 60, 5 * 60);

    // Commands for dialogue
    loop {
        let command : String = Input::new()
            .with_prompt("Command to run?")
            .interact_text()
            .unwrap();

        // Parse the command
        let command_to_run = match command.as_str() {
            "start" => Some(CommandStrings::StartTimer),
            "stop" => Some(CommandStrings::StopTimer),
            "pause" => Some(CommandStrings::PauseTimer),
            "help" => Some(CommandStrings::Help),
            "settings" => Some(CommandStrings::ViewSettings),
            "update" => Some(CommandStrings::UpdateDurations),
            _ => None
        };

        // Run the given command
        if let Some(command_to_run) = command_to_run {
            match command_to_run {
                CommandStrings::Help => println!("Available commands: start, stop, pause, help, update"),
                CommandStrings::ViewSettings => {
                    let work_duration = timer.get_work_duration();
                    let break_duration = timer.get_work_duration();
                    println!("Work duration: {:?}. Break duration: {:?}", work_duration, break_duration)
                },
                CommandStrings::UpdateDurations => {
                    let working : String = Input::new()
                        .with_prompt("How long should the working phase be (in sec)?")
                        .interact_text()
                        .unwrap();

                    let breaking : String = Input::new()
                        .with_prompt("How long should the breaking phase be (in sec)?")
                        .interact_text()
                        .unwrap();

                    let new_working_duration = working.parse::<u64>();
                    let new_break_duration = breaking.parse::<u64>();

                    if new_working_duration.is_ok() && new_break_duration.is_ok() {
                        timer.set_work_duration(Duration::from_secs(new_working_duration.unwrap()));
                        timer.set_break_duration(Duration::from_secs(new_break_duration.unwrap()));
                        println!("Durations now updated")
                    } else {
                        println!("Invalid durations - must be integers");
                    }
                },
                CommandStrings::StartTimer => timer.start_timer(),
                CommandStrings::StopTimer => timer.stop_timer(),
                CommandStrings::PauseTimer => timer.pause_timer(),
            }
        } else {
            eprintln!("Command not recognized");
        }


    }

}
