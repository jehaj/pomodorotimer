use std::sync::mpsc::Receiver;
use std::thread;
use std::time::{Duration, Instant};
use crate::timer_commander::TimerCommand;
use crate::timer_commander::TimerCommand::Stop;

#[derive(PartialEq)]
pub enum ExitCondition{
    Ok,
    Terminated
}

pub struct TimerRunner {
    command_receiver: Receiver<TimerCommand>,
}

impl TimerRunner {
    pub fn new(command_receiver: Receiver<TimerCommand>) -> Self {
        TimerRunner {
            command_receiver,
        }
    }

    pub fn run_timer(&mut self, duration: Duration) -> ExitCondition {
        // Start the timer
        let start_time = Instant::now();
        while start_time.elapsed() < duration {
            let remaining = duration - start_time.elapsed();

            // Print every 10 seconds
            println!("Time remaining: {}:{}", remaining.as_secs() / 60, remaining.as_secs() % 60);

            // Sleep for a second
            thread::sleep(Duration::from_secs(1));

            // Check for new commands
            let command = self.check_for_new_command();

            if let Some(command) = command {
                match command {
                    TimerCommand::Start => continue,
                    TimerCommand::Pause => if self.wait_for_resume() == Stop { return ExitCondition::Terminated; },
                    TimerCommand::Stop => return ExitCondition::Terminated,
                }
            }
        }

        ExitCondition::Ok
    }

    fn check_for_new_command(&mut self) -> Option<TimerCommand> {
        // Try and get the new command
        let command = self.command_receiver.try_recv();
        // When present return it else return none
        if let Ok(command) = command {
            Some(command)
        } else {
            None
        }
    }

    fn wait_for_resume(&mut self) -> TimerCommand {
        loop {
            let command = self.check_for_new_command();
            if let Some(command) = command {
                match command {
                    TimerCommand::Pause => continue,
                    TimerCommand::Start => return TimerCommand::Start,
                    Stop => return Stop,
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}

