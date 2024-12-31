use std::time::Duration;
use crossterm::event::poll;
use tui_input::{Input, InputRequest};
use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout, Position}, style::{Color, Modifier, Style, Stylize}, text, text::{Line, Span, Text}, widgets::{Block, List, ListItem, Paragraph}, DefaultTerminal, Frame};
use crate::pomodoro_timer::PomodoroTimer;
use crate::tui_app::MessageType::{InvalidCommand, ValidCommand};

pub struct App {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<(String, MessageType)>,
    /// The pomodoro timer - Application
    timer: PomodoroTimer
}

enum InputMode {
    Normal,
    Editing,
}

enum MessageType {
    ValidCommand,
    InvalidCommand,
}

impl App {
    pub fn new(timer: PomodoroTimer) -> Self {
        App {
            input: "".into(),
            input_mode: InputMode::Normal,
            messages: vec![],
            timer
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame)).expect("Could not draw");

            if let Ok(true) = poll(Duration::from_millis(75)) {
                if let Ok(Event::Key(key)) = event::read() {
                    match self.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('e') => {
                                self.input_mode = InputMode::Editing;
                            }
                            KeyCode::Char('q') => {
                                return;
                            }
                            _ => {}
                        },
                        InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                            KeyCode::Enter => self.submit_command(),
                            KeyCode::Char(to_insert) => self.enter_char(to_insert),
                            KeyCode::Backspace => self.delete_char(),
                            KeyCode::Left => self.move_cursor_left(),
                            KeyCode::Right => self.move_cursor_right(),
                            KeyCode::Esc => self.input_mode = InputMode::Normal,
                            _ => {}
                        },
                        InputMode::Editing => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [information_area, help_area, input_area, messages_area] = vertical.areas(frame.area());

        // Timer display in top half
        let time_remaining = self.timer.get_remaining_time().as_secs();
        let time_remaining_min = time_remaining / 60;
        let time_remaining_sec = time_remaining % 60;

        let work_duration = self.timer.get_work_duration().as_secs();
        let work_duration_min = work_duration / 60;
        let work_duration_sec = work_duration % 60;

        let break_duration = self.timer.get_break_duration().as_secs();
        let break_duration_min = break_duration / 60;
        let break_duration_sec = break_duration % 60;

        let timer_text = vec![
            text::Line::from(format!("Time remaining: {:?}:{:?}", time_remaining_min, time_remaining_sec )),
            text::Line::from(""),
            text::Line::from(format!("Timer state: {:?}", self.timer.get_state() )),
            text::Line::from(""),
            text::Line::from(format!("Current stats: Working {:?}:{:?} and breaking {:?}:{:?}", work_duration_min, work_duration_sec, break_duration_min, break_duration_sec)),
        ];

        let timer_widget = Paragraph::new(timer_text)
            .block(Block::bordered().title("Timer"))
            .style(Style::default().fg(Color::Green));

        frame.render_widget(timer_widget, information_area);

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to start entering commands.".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to record the message".into(),
                ],
                Style::default(),
            ),
        };

        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.to_string())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);

        match self.input_mode {
            InputMode::Normal => {}
            InputMode::Editing => frame.set_cursor_position(Position::new(
                input_area.x + self.input.cursor() as u16 + 1,
                input_area.y + 1,
            )),
        }

        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let (message, t) = m;
                let content = Line::from(Span::raw(format!("{i}: {message}")));
                let color = match t {
                    ValidCommand => Color::Green,
                    InvalidCommand => Color::Red,
                };
                ListItem::new(content).style(Style::default().fg(color))
            })
            .collect();
        let messages = List::new(messages).block(Block::bordered().title("Messages"));
        frame.render_widget(messages, messages_area);
    }

    fn submit_command(&mut self) {
        let message = self.input.to_string();

        let command_validity = match message.to_lowercase().as_str() {
            "start" => {
                self.timer.start_timer();
                ValidCommand
            },
            "stop" => {
                self.timer.stop_timer();
                ValidCommand
            },
            "pause" => {
                self.timer.pause_timer();
                ValidCommand
            },
            _ => InvalidCommand
        };

        self.messages.push((message, command_validity));

        // Clear the terminal
        self.input = "".into();
    }

    fn enter_char(&mut self, char_entered: char) {
        let input_request = InputRequest::InsertChar(char_entered);
        let input_response = self.input.handle(input_request);

        if input_response.is_some() {
            self.move_cursor_right();
        }
    }

    fn delete_char(&mut self) {
        let delete_request = InputRequest::DeletePrevChar;
        self.input.handle(delete_request);
    }

    fn move_cursor_left(&mut self) {
        let move_left_request = InputRequest::GoToPrevChar;
        self.input.handle(move_left_request);
    }

    fn move_cursor_right(&mut self) {
        let move_left_request = InputRequest::GoToNextChar;
        self.input.handle(move_left_request);
    }
}