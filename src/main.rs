use clap::Parser;
use std::io;

mod app;
mod epub;
mod image_handler;
mod parser;
mod reader;
mod ui;

use crate::app::AppState;
use crate::epub::handler::EpubHandler;
use crate::image_handler::create_image_widget;
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
                UserAction::ViewImage => {
                    // Display the current image if there is one
                    if let Some(image_path) = app_state.get_current_image_path() {
                        if !image_path.as_os_str().is_empty() {
                            // Convert PathBuf to string for create_image_widget function
                            if let Some(path_str) = image_path.to_str() {
                                // Try to create the image widget
                                match create_image_widget(path_str) {
                                    Ok(_image_widget) => {
                                        // In a full implementation, we would render the image widget
                                        // For now, we'll just show a message
                                        // ui.clear_screen()?;
                                        ui.draw(|frame| {
                                            Renderer::render_image(
                                                frame,
                                                path_str,
                                                &app_state.get_chapter_title(),
                                                app_state.get_chapter_progress(),
                                                app_state.scroll_position,
                                            );
                                        })?;
                                        let _ = ratatui::crossterm::event::read();
                                        // Reinitialize the terminal
                                        // ui.init()?;
                                    }
                                    Err(e) => {
                                        ui.clear_screen()?;
                                        println!("Error creating image widget: {}", e);
                                        println!("Press any key to continue...");
                                        let _ = ratatui::crossterm::event::read();
                                        ui.init()?;
                                    }
                                }
                            } else {
                                ui.clear_screen()?;
                                println!("Error: Invalid image path");
                                println!("Press any key to continue...");
                                let _ = ratatui::crossterm::event::read();
                                ui.init()?;
                            }
                        }
                    }
                }
            }
        }
    }

    // Restore terminal
    ui.restore()?;

    Ok(())
}
