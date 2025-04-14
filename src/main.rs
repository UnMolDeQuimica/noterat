use core::str;
use std::{env, fs, io, process::Command};

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen}};
use ratatui::{
    buffer::Buffer, layout::{Constraint, Flex, Layout, Rect}, style::{
        palette::tailwind::SLATE, Modifier, Style, Stylize,
    }, symbols::border, text::{Line, Text}, widgets::{Block, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget, Widget, Clear}, DefaultTerminal, Frame
};

use clap::Parser;

#[derive(Parser)]
#[command(name = "noterat", version, about, long_about = r#"
A simple TUI app to create and manage notes.
"#
)]
struct Cli {
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _cli = Cli::parse();

    run_ui()?;

    Ok(())
}

fn run_ui() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}


#[derive(Debug, Default)]
pub struct App {
    exit: bool
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(q) => {
                self.exit();
            },
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" COMRAD ".bold());

        let instructions = Line::from(vec![
            " Quit ".into(),
            "<q> ".blue().bold(),
            ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        // let list =List::new(vec!::new[0, 1, 2])
            // .block(block)
            // .highlight_symbol(">> ")
            // .highlight_spacing(HighlightSpacing::Always)
            // .highlight_style(SELECTED_STYLE);

        // StatefulWidget::render(list, area, buf, &mut self.list_state);
    }
}


