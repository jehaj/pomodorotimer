
mod pomodoro_timer_tests {
    use std::thread;
    use std::time::Duration;
    use pomodorotimer::pomodoro_timer::PomodoroTimer;
    use pomodorotimer::pomodoro_timer::TimerState::{Breaking, Idle, Working};

    // Timer runner tests
    #[test]
    fn should_new_timer_start_in_idle() {
        // Given a timer
        let timer = PomodoroTimer::new( 0, 0);

        // When I ask for it's state
        let got = timer.get_state();

        // Then it should be in Idle state
        assert_eq!(got, Idle)
    }

    #[test]
    fn should_timer_end_in_idle_state() {
        // Given a timer
        let mut timer = PomodoroTimer::new(0, 0);

        // When I ask it to start and wait second
        timer.start_run();

        // Then it should be in Idle state
        let got = timer.get_state();
        assert_eq!(got, Idle)
    }

    #[test]
    fn should_pause_in_working() {
        // Given a timer
        let mut timer = PomodoroTimer::new(2, 1);
        timer.start_run();
        thread::sleep(Duration::from_secs(1));

        // When I ask it to pause in working
        timer.pause_timer();

        // Then it should remain in the same state
        assert_eq!(timer.get_state(), Working);
    }

    #[test]
    fn should_pause_in_breaking() {
        // Given a timer
        let mut timer = PomodoroTimer::new(0, 2);
        timer.start_run();
        thread::sleep(Duration::from_secs(1));

        // When I ask it to pause in breaking
        timer.pause_timer();

        // Then it should remain in the same state
        assert_eq!(timer.get_state(), Breaking);
    }


    #[test]
    fn should_pause_and_resume_stop_in_idle() {
        // Given a timer
        let mut timer = PomodoroTimer::new(2, 1);
        timer.start_run();
        thread::sleep(Duration::from_secs(1));

        // When I ask it to pause and resume
        timer.pause_timer();
        timer.resume_timer();

        // Then it should remain in the same state
        assert_eq!(timer.get_state(), Working);
    }

    #[test]
    fn should_stop_end_in_idle() {
        // Given a timer
        let mut timer = PomodoroTimer::new(3, 2);
        timer.start_run();

        // When I ask it to stop
        timer.stop_timer();
        let state = timer.get_state();

        // Then it should return to "Idle"
        assert_eq!(state, Idle);
    }

}

