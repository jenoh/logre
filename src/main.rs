use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut sentence = "abc";
    let letters: Vec<char> = sentence.to_string().chars().collect();

    let mut index = (0, letters.len());

    loop {

        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new(sentence)
                    .white()
                    .on_blue(),
                area,
            );
        })?;

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char(letters[index.0]) {
                    index.0 = index.0 + 1;
                    if (index.0 == index.1) {
                        sentence = "gagné"
                    }
                }
            }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}