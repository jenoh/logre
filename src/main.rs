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
use ratatui::widgets::{Block, Padding};


fn render_sentence(game: &Game) -> Line {
    Line::from(vec![
        Span::styled(&game.sentence[0..game.pointer], Style::default().fg(Color::Green)),
        Span::styled(&game.sentence[game.pointer ..game.pointer +1], Style::default().fg(Color::Yellow)),
        Span::raw(&game.sentence[game.pointer +1..game.sentence_size]),
    ])
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut game = Game::default();

    loop {

        terminal.draw(|frame| {
            let area = frame.size();
            let p = Paragraph::new(render_sentence(&game))
                .block(Block::new().style(Style::new().bg(Color::Black)).padding(Padding::new(
                    0, // left
                    0, // right
                    area.height / 2, // top
                    0, // bottom
                )))
                .style(Style::new().white())
                .alignment(Alignment::Center);
            frame.render_widget(p, area);
        })?;

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('²') {
                    break;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char(game.letters[game.pointer]) {
                    game.pointer = game.pointer + 1;
                    if game.pointer == game.sentence_size {
                        game = Game::default()
                    }
                }
            }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}