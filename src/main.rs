use clap::Parser;
use std::io;

mod app;
mod epub;
mod parser;
mod reader;
mod ui;

use crate::app::AppState;
use crate::epub::handler::EpubHandler;
use crate::parser::CliArgs;
use crate::reader::renderer::Renderer;
use crate::ui::{UI, UserAction};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    // Initialize the EPUB handler
    let epub_handler =
        EpubHandler::new(args.filename).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Initialize application state
    let mut app_state = AppState::new(epub_handler, args.chapter.unwrap_or(0))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Initialize UI
    let mut ui = UI::new()?;
    ui.init()?;

    // Main application loop
    loop {
        // Render the UI
        ui.draw(|frame| {
            Renderer::render_chapter(
                frame,
                &app_state.renderable_chapter,
                &app_state.get_chapter_title(),
                app_state.get_chapter_progress(),
                app_state.scroll_position,
            );
        })?;

        // Check if we should quit
        if app_state.should_quit {
            break;
        }

        // Handle user input
        if let Some(action) = ui.handle_events()? {
            match action {
                UserAction::Quit => {
                    app_state.should_quit = true;
                }
                UserAction::NextChapter => {
                    app_state.next_chapter()?;
                }
                UserAction::PreviousChapter => {
                    app_state.previous_chapter()?;
                }
                UserAction::ScrollDown => {
                    app_state.scroll_down();
                }
                UserAction::ScrollUp => {
                    app_state.scroll_up();
                }
                UserAction::PageDown => {
                    let page_size = (ui.size().height / 2) as usize;
                    app_state.page_down(page_size);
                }
                UserAction::PageUp => {
                    let page_size = (ui.size().height / 2) as usize;
                    app_state.page_up(page_size);
                }
            }
        }
    }

    // Restore terminal
    ui.restore()?;

    Ok(())
}
