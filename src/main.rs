use pomodorotimer::pomodoro_timer::PomodoroTimer;
use pomodorotimer::tui_app::App;

/// Personal pomodoro timer main function
fn main() {
    // Create the timer
    let timer = PomodoroTimer::new(10, 5);

    // Run the TUI
    let terminal = ratatui::init();
    App::new(timer).run(terminal);
    ratatui::restore();
}
