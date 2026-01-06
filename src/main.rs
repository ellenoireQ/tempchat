use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Alignment, widgets::Paragraph};

// User login struct
#[derive(Debug, Default)]
struct User {
    id: String,
    username: String,
}

#[derive(Debug, Default)]
struct AppState {
    user: User,
}

fn main() -> color_eyre::Result<()> {
    let mut state = AppState::default();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        //
        terminal.draw(|f| render(f, app_state))?;
        // Input handling
        //
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Right => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let area = frame.size();
    let paragraph = Paragraph::new("Hello from application").alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}
