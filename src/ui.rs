use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

pub struct UI {
  terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl UI {
  pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    Ok(UI { terminal })
  }

  pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    ratatui::crossterm::terminal::enable_raw_mode()?;
    let _ = self.clear_screen();
    Ok(())
  }

  pub fn restore(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    let _ = self.clear_screen();
    ratatui::crossterm::terminal::disable_raw_mode()?;
    Ok(())
  }

  pub fn clear_screen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    self.terminal.clear()?;
    Ok(())
  }

  pub fn draw<F>(&mut self, frame_fn: F) -> Result<(), Box<dyn std::error::Error>>
  where
    F: FnOnce(&mut ratatui::Frame),
  {
    self.terminal.draw(frame_fn)?;
    Ok(())
  }

  pub fn size(&self) -> ratatui::layout::Rect {
    // Return a default size since we can't get the actual size without a mutable reference
    ratatui::layout::Rect::new(0, 0, 80, 24)
  }

  pub fn handle_events(&self) -> Result<Option<UserAction>, Box<dyn std::error::Error>> {
    if ratatui::crossterm::event::poll(std::time::Duration::from_millis(100))? {
      if let Event::Key(key) = ratatui::crossterm::event::read()? {
        if key.kind == KeyEventKind::Press {
          match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(Some(UserAction::Quit)),
            KeyCode::Char('j') | KeyCode::Down => {
              return Ok(Some(UserAction::ScrollDown));
            }
            KeyCode::Char('k') | KeyCode::Up => return Ok(Some(UserAction::ScrollUp)),
            KeyCode::Char(' ') => return Ok(Some(UserAction::PageDown)),
            KeyCode::Char('b') => return Ok(Some(UserAction::PageUp)),
            KeyCode::Char('l') | KeyCode::Right => {
              return Ok(Some(UserAction::NextChapter));
            }
            KeyCode::Char('h') | KeyCode::Left => {
              return Ok(Some(UserAction::PreviousChapter));
            }
            KeyCode::Char('i') => {
              return Ok(Some(UserAction::ViewImage));
            }
            _ => {}
          }
        }
      }
    }
    Ok(None)
  }
}

pub enum UserAction {
  Quit,
  NextChapter,
  PreviousChapter,
  ScrollDown,
  ScrollUp,
  PageDown,
  PageUp,
  ViewImage,
}
