use std::thread;
use std::time::{Duration, Instant};
use crate::pomodoro_timer::TimerState::{Breaking, Idle, Paused, Working};

#[derive(PartialEq, Copy, Eq, Clone, Debug)]
pub enum TimerState {
    Idle,
    Working,
    Breaking,
    Paused,
}

pub struct PomodoroTimer{
    work_duration: Duration,
    break_duration: Duration,
    current_state: TimerState,
}

impl PomodoroTimer{

    // Constructor that creates a new PomodoroTimer instance
    pub fn new(work_duration_sec: u64, break_duration_sec: u64) -> PomodoroTimer {
        // Create the new timer
        PomodoroTimer {
            // Standard is 20 min
            work_duration: Duration::from_secs(work_duration_sec),
            // Standard is 5 min
            break_duration: Duration::from_secs(break_duration_sec),
            current_state: Idle,
        }
    }

    // Functions to manipulate the state
    pub fn get_state(&self) -> TimerState {
        self.current_state
    }

    fn update_state(&mut self, new_state: TimerState) {
        self.current_state = new_state;
    }

    // A run being an "Idle -> Working -> Break -> Idle" iteration
    pub fn start_run(&mut self){
        // Set the current state to working
        self.update_state(Working);
        // Start the timer
        println!("Starting pomodoro timer");
        self.run_timer(self.work_duration, Breaking);
    }

    pub fn pause_timer(&mut self){
        self.update_state(Paused);
    }

    fn run_timer(&mut self, duration: Duration, next_state: TimerState) {
        // Start the timer
        let start_time = Instant::now();
        while start_time.elapsed() < duration {
            let remaining = duration - start_time.elapsed();
            println!("Time remaining: {}:{}", remaining.as_secs() / 60, remaining.as_secs() % 60);
            thread::sleep(Duration::from_secs(10));

            if self.current_state == Paused {
                println!("The timer has been paused ... write resume to continue!")
            }

        }

        // Move to next state
        match next_state {
            Breaking => {
                self.update_state(Breaking);
                println!("Work session complete. Take a {}-minute break!", self.break_duration.as_secs() / 60);
                self.run_timer(self.break_duration, Idle);
            },
            Idle => {
                self.update_state(Idle);
                println!("Pomodoro session ended.")
            }
            _ => {},
        }
    }
}
