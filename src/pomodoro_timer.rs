use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use crate::pomodoro_timer::TimerState::{Breaking, Idle, Working};
use crate::timer_commander::TimerCommander;
use crate::timer_runner::{ExitCondition, TimerRunner};

pub struct PomodoroTimer{
    work_duration: Duration,
    break_duration: Duration,
    current_state: Arc<Mutex<TimerState>>,
    commander: Option<TimerCommander>,
    receiver: Option<Receiver<Duration>>
}

#[derive(PartialEq, Copy, Eq, Clone, Debug, Hash)]
pub enum TimerState {
    Idle,
    Working,
    Breaking
}


impl PomodoroTimer{
    // Constructor that creates a new PomodoroTimer instance
    pub fn new(work_duration_sec: u64, break_duration_sec: u64) -> PomodoroTimer {
        let pomodoro_timer = PomodoroTimer {
            work_duration: Duration::from_secs(work_duration_sec),
            break_duration: Duration::from_secs(break_duration_sec),
            current_state: Arc::new(Mutex::new(Idle)),
            commander: None,
            receiver: None
        };

        // Create the new timer instance
        pomodoro_timer
    }

    // A run being an "Idle -> Working -> Break -> Idle" iteration
    pub fn start_run(&mut self){
        // Create new Timer runner
        let (tx, rx) = mpsc::channel();
        let (time_tx, time_rx) = mpsc::channel();

        let mut timer_runner = TimerRunner::new(rx, time_tx);

        // Create the command injector
        let timer_commander = TimerCommander::new(tx);

        self.commander = Some(timer_commander);
        self.receiver = Some(time_rx);

        // Save the times in separate variables
        let working_duration = self.work_duration;
        let break_duration = self.break_duration;
        let current_state = Arc::clone(&self.current_state);


        thread::spawn(move || {
            // Run through the phases
            // Start in working phase
            PomodoroTimer::update_state(&current_state, Working);
            let exit_condition = timer_runner.run_timer(working_duration);

            if exit_condition == ExitCondition::Terminated {
                PomodoroTimer::update_state(&current_state, Idle);
                return;
            }

            // Then breaking phase
            PomodoroTimer::update_state(&current_state, Breaking);
            let exit_condition = timer_runner.run_timer(break_duration);

            if exit_condition == ExitCondition::Terminated {
                PomodoroTimer::update_state(&current_state, Idle);
                return;
            }

            // Then return to idle
            PomodoroTimer::update_state(&current_state, Idle);
        });

    }

    fn update_state(state: &Arc<Mutex<TimerState>>, new_state: TimerState){
        let mut current_state = state.lock().expect("Failed to lock current state");
        *current_state = new_state;
    }

    pub fn get_state(&self) -> TimerState {
        let current_state = self.current_state.lock().expect("Failed to lock current state");
        *current_state
    }

    pub fn start_timer(&mut self){
        let current_state = self.current_state.lock().expect("Failed to lock current state");
        if *current_state == Idle {
            drop(current_state);
            self.start_run();
        } else {
            drop(current_state);
            self.resume_timer()
        }
    }

    pub fn pause_timer(&mut self){
        if self.get_state() == Idle {
            return;
        }

        match &mut self.commander {
            None => println!("Have to start a sessions to give commands"),
            Some(c) => c.pause_timer()
        }
    }

    pub fn stop_timer(&mut self){
        if self.get_state() == Idle {
            return;
        }

        match &mut self.commander {
            None => println!("Have to start a sessions to give commands"),
            Some(c) => {
                c.stop_timer();
            }
        }
    }

    pub fn resume_timer(&mut self) {
        match &mut self.commander {
            None => println!("Have to start a sessions to give commands"),
            Some(c) => c.resume_timer()
        }
    }

    pub fn get_work_duration(&self) -> Duration {
        self.work_duration
    }

    pub fn get_break_duration(&self) -> Duration {
        self.break_duration
    }

    pub fn set_work_duration(&mut self, duration: Duration) {
        self.work_duration = duration
    }

    pub fn set_break_duration(&mut self, duration: Duration) {
        self.break_duration = duration
    }

    pub fn get_remaining_time(&mut self) -> Duration {
        if self.receiver.is_some() && self.commander.is_some() {
            let r = self.receiver.as_ref().unwrap();
            let c = self.commander.as_ref().unwrap();
            let success = c.get_time_remaining();
            if !success {
                // We are now in "Idle" since execution stopped
                self.receiver = None;
                self.commander = None;
                return Duration::from_secs(3599)
            }
            r.recv().unwrap()
        } else {
            Duration::from_secs(3599)
        }
    }

    pub fn set_state_time_period(&mut self, period: Duration, state: TimerState) {
        // Stop timer when user updates duration
        self.stop_timer();

        match state {
            Idle => {}
            Working => {self.work_duration = period;},
            Breaking => {self.break_duration = period},
        }
    }
}


