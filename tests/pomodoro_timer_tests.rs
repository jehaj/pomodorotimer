
mod pomodoro_timer_tests {
    use pomodorotimer::pomodoro_timer::{PomodoroTimer, TimerState};

    #[test]
    fn should_new_timer_start_in_idle() {
        // Given a timer
        let timer = PomodoroTimer::new(0, 0);

        // When I ask for it's state
        let got = timer.get_state();

        // Then it should be in Idle state
        assert_eq!(got, TimerState::Idle)
    }

    #[test]
    fn should_timer_end_in_idle_state() {
        // Given a timer
        let mut timer = PomodoroTimer::new(0,0);

        // When I ask it to start and wait second
        timer.start_run();

        // Then it should be in Idle state
        let got = timer.get_state();
        assert_eq!(got, TimerState::Idle)
    }
}

