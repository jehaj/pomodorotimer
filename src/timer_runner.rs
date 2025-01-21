use std::sync::mpsc::{Receiver, Sender};
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
    time_sender: Sender<Duration>
}

impl TimerRunner {
    pub fn new(command_receiver: Receiver<TimerCommand>, time_sender: Sender<Duration>) -> Self {
        TimerRunner {
            time_sender,
            command_receiver,
        }
    }

    pub fn run_timer(&mut self, duration: Duration) -> ExitCondition {
        // Start the timer
        let start_time = Instant::now();
        while start_time.elapsed() < duration {
            let remaining = duration - start_time.elapsed();

            // Check for new commands
            let command = self.check_for_new_command();

            if let Some(command) = command {
                match command {
                    TimerCommand::Start => continue,
                    TimerCommand::Pause => if self.wait_for_resume(remaining) == Stop { return ExitCondition::Terminated; },
                    Stop => return ExitCondition::Terminated,
                    TimerCommand::GetTimeRemaining => self.time_sender.send(remaining).unwrap(),
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

    fn wait_for_resume(&mut self, remaining: Duration) -> TimerCommand {
        while let Ok(command) = self.command_receiver.recv() {
            match command {
                TimerCommand::Pause => continue,
                TimerCommand::Start => return TimerCommand::Start,
                TimerCommand::GetTimeRemaining => {
                    self.time_sender.send(remaining).unwrap();
                    continue;
                }
                Stop => return Stop,
            }
        }
        Stop
    }
}

