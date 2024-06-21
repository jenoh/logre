mod gameplay;
 use gameplay::Game;
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
use ratatui::layout::Alignment;
use ratatui::prelude::{Line, Span, Style};
use ratatui::style::Color;


fn render_sentence<'a>(sentence: &'a str, game:&'a Game) -> Line<'a> {
    Line::from(vec![
        Span::raw(&sentence[0..game.pointer]),
        Span::styled(&sentence[game.pointer ..game.pointer +1], Style::default().fg(Color::Green)),
        Span::raw(&sentence[game.pointer +1..game.sentence_size]),
    ])
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut sentence = "abc";
    let letters: Vec<char> = sentence.to_string().chars().collect();

    let mut index = (0, letters.len());

    let mut game = Game {
        pointer: 0,
        sentence_size: letters.len()
    };

    loop {

        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new(render_sentence(&sentence, &game))
                    .white().alignment(Alignment::Center),
                area,
            );
        })?;

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char(letters[game.pointer]) {
                    game.pointer = game.pointer + 1;
                    if game.pointer == game.sentence_size {
                        break;
                    }
                }
            }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}