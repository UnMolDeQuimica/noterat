use core::str;
use std::{env, fs, io, process::Command};

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen}};
use ratatui::{
    buffer::Buffer, layout::{Constraint, Flex, Layout, Rect}, style::{
        palette::tailwind::SLATE, Modifier, Style, Stylize,
    }, symbols::border, text::{Line, Text}, widgets::{Block, Wrap, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget, Widget, Clear}, DefaultTerminal, Frame
};

use std::borrow::Cow;

use clap::Parser;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

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
    exit: bool,
    notes_entry_list: Vec<String>,
    list_state: ratatui::widgets::ListState,
    enter_note: bool,
    scroll: u16,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {

        self.notes_entry_list = NoteEntry::note_entries_in_directory();
        self.list_state = ListState::default();
        self.list_state.select(Some(0));

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
            KeyCode::Char('g') => self.first(),
            KeyCode::Char('G') => self.last(),
            KeyCode::Char('j') => self.previous(),
            KeyCode::Char('k') => self.next(),
            KeyCode::Up => self.previous(),
            KeyCode::Down => self.next(),
            KeyCode::Right => {
                self.scroll = self.scroll.saturating_add(1); // Scroll down
            },
            KeyCode::Left => {
                self.scroll = self.scroll.saturating_sub(1); // Scroll up
            },
            KeyCode::Char('q') => {
                self.exit();
            },
            KeyCode::Char('l') => {
                let entries = NoteEntry::note_entries_in_directory();
                println!("{:?}", entries)
            },
            KeyCode::Enter => {
                self.enter_note = true;
            },
            KeyCode::Esc => {
                self.enter_note = false;
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }


    fn next(&mut self) {
        let i = self.list_state.selected().unwrap();
        if i < self.notes_entry_list.len() - 1 {
            self.list_state.select(Some(i + 1));
        }
    }

    fn previous(&mut self) {
        let i = self.list_state.selected().unwrap();
        if i > 0 {
            self.list_state.select(Some(i - 1));
        }
    }

    fn first(&mut self) {
        self.list_state.select_first();
    }

    fn last(&mut self) {
        self.list_state.select_last();
    }

    fn render_note_content(&mut self, area: Rect, buf: &mut Buffer) {
        if !self.enter_note {
            return
        }
        Clear.render(area, buf);
        let index = self.list_state.selected().unwrap();
        let entries = NoteEntry::note_entries_in_directory();
        // let notes_list = NoteEntry::note_entries_to_list_item(&entries);
        let note = Note::read_note(String::from(&entries[index]));
        Clear.render(area, buf);
        let paragraph = Paragraph::new(Text::raw(note.content)).block(Block::bordered().title(Line::from(String::from(note.title)).centered())).wrap(Wrap { trim: true }).scroll((self.scroll, 0));
        paragraph.render(area, buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" COMRAD ".bold());

        let instructions = Line::from(vec![
            " Quit ".into(),
            "<q> ".blue().bold(),
            ]);

        let entries = NoteEntry::note_entries_in_directory();
        let notes_list = NoteEntry::note_entries_to_list_item(&entries);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let list =List::new(notes_list)
            .block(block)
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(SELECTED_STYLE);

        StatefulWidget::render(list, area, buf, &mut self.list_state);
        self.render_note_content(area, buf);
    }
}

#[derive(Debug)]

struct Note {
    title: String,
    content: String
}

impl Note {
    fn new(title: String, content: String) -> Self {
        Self {
            title: title,
            content: content
        }
    }

    fn save_note(note: Note) {
        let mut note_path: String = String::from(note.title);
        note_path.push_str(".md");
        fs::write(note_path, note.content).expect("Unable to write file");
    }

    fn read_note(path: String) -> Note {
        let note_data = fs::read_to_string(&path).expect("Unable to read file");
        Note::new(path, note_data)
    }
}

#[derive(Debug)]

struct NoteEntry {
    title: String,
}

impl NoteEntry {
    fn new(title: String) -> Self {
        Self {
            title: title,
        }
    }

    fn note_entries_in_directory() -> Vec<String> {
        let paths = fs::read_dir("./").unwrap(); // TODO: Change it to custom path later
        let files = paths
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path().display().to_string())
            .filter(|entry| entry.ends_with(".md"));
        Vec::from_iter(files)
    }

    fn note_entries_to_list_item(entries: &Vec<String>) -> Vec<ListItem> {
        entries
            .into_iter()
            .map(|entry| ListItem::new(String::from(entry)))
            .collect()
    }

}
