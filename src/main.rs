mod textinput;

use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize, palette::tailwind},
    symbols,
    text::{self, Line},
    widgets::{Block, Borders, Padding, Paragraph, Tabs, Widget, Wrap},
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use crate::textinput::textinput::TextInput;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Default)]
struct App {
    state: AppState,
    selected_tab: SelectedTab,
    input: TextInput,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "Channel")]
    Tab1,
    #[strum(to_string = "Account")]
    Tab2,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if self.input.focused {
                    match key.code {
                        KeyCode::Esc => self.input.focused = false,
                        _ => self.input.handle_event(key),
                    }
                } else {
                    match key.code {
                        KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                        KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                        KeyCode::Char('q') => self.quit(),
                        KeyCode::Char('i') => self.input.focused = true,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl SelectedTab {
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.selected_tab.render(inner_area, buf, &self.input);
        render_footer(footer_area, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Tempchat v1".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer, app: &TextInput) {
        match self {
            Self::Tab1 => self.render_tab0(area, buf),
            Self::Tab2 => self.render_tab1(area, buf, app),
        }
    }
}

impl SelectedTab {
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    fn render_tab0(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Channel content here")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab1(&self, area: Rect, buf: &mut Buffer, input: &TextInput) {
        use Constraint::{Length, Min};
        let block = Block::default()
            .title(" Account ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        block.render(area, buf);

        let layout = Layout::vertical([
            Length(1),
            Length(1),
            Length(1),
            Length(1),
            Length(3),
            Min(0),
        ]);

        let [
            id_label_area,
            id_value_area,
            _,
            name_label_area,
            input_area,
            _,
        ] = layout.areas(inner);

        Paragraph::new("User ID")
            .style(Style::default().fg(Color::DarkGray))
            .render(id_label_area, buf);

        Paragraph::new("usr_a1b2c3d4e5f6")
            .style(Style::default().fg(Color::Yellow).bold())
            .render(id_value_area, buf);

        Paragraph::new("Name")
            .style(Style::default().fg(Color::DarkGray))
            .render(name_label_area, buf);

        input.render(input_area, buf);
    }

    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::ROUNDED)
            .padding(Padding::horizontal(1))
            .border_style(Color::White)
    }

    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Tab1 => tailwind::BLUE,
            Self::Tab2 => tailwind::EMERALD,
        }
    }
}
