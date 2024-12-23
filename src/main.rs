
/// Personal pomodoro timer main function
use std::time::Duration;
use clap::Parser;
use crossterm::event;
use crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};
use pomodorotimer::pomodoro_timer::PomodoroTimer;
use std::io::{stdout, Result};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, text, Terminal};


#[derive(Parser)]
struct Cli {
    /// The command to execute to look for
    command: String,
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut timer = PomodoroTimer::new(10, 5);

    // Main loop
    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(frame.area());

            // Timer display in top half
            let time_remaining = timer.get_remaining_time().as_secs();
            let time_remaining_min = time_remaining / 60;
            let time_remaining_sec = time_remaining % 60;

            let work_duration = timer.get_work_duration().as_secs();
            let work_duration_min = work_duration / 60;
            let work_duration_sec = work_duration % 60;

            let break_duration = timer.get_break_duration().as_secs();
            let break_duration_min = break_duration / 60;
            let break_duration_sec = break_duration % 60;

            let timer_text = vec![
                text::Line::from(""),
                text::Line::from(format!("Time remaining: {:?}:{:?}", time_remaining_min, time_remaining_sec )),
                text::Line::from(""),
                text::Line::from(format!("Timer state: {:?}", timer.get_state() )),
                text::Line::from(""),
                text::Line::from(format!("Current stats: Working {:?}:{:?} and breaking {:?}:{:?}", work_duration_min, work_duration_sec, break_duration_min, break_duration_sec)),
            ];

            let timer_widget = Paragraph::new(timer_text)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Timer"))
                .style(Style::default().fg(Color::Green));

            // Command prompt in bottom half
            let prompt_text = Text::raw("Enter command (start/stop/pause/help): ");
            let prompt_widget = Paragraph::new(prompt_text)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Command"));

            frame.render_widget(timer_widget, chunks[0]);
            frame.render_widget(prompt_widget, chunks[1]);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Char('s') => timer.start_timer(),
                    event::KeyCode::Char('p') => timer.pause_timer(),
                    event::KeyCode::Char('x') => timer.stop_timer(),
                    event::KeyCode::Char('h') => {
                        // Could add help display here
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup and restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}


/*
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
                CommandStrings::Help => println!("Available commands: start, stop, pause, help, update, settings"),
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
*/
