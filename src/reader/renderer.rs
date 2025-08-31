// src/reader/renderer.rs

use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use crate::epub::content::{RenderableChapter, RenderableBlock};

pub struct Renderer;

impl Renderer {
    pub fn render_chapter(
        frame: &mut ratatui::Frame,
        chapter: &RenderableChapter,
        title: &str,
        progress: f64,
        scroll_position: usize,
    ) {
        let size = frame.area();
        
        // Create the layout sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // Content
                Constraint::Length(3),  // Footer
            ])
            .split(size);

        // Header with title
        let title_block = Block::default()
            .borders(Borders::ALL)
            .title(title);
        
        let title_paragraph = Paragraph::new("")
            .block(title_block);
        
        frame.render_widget(title_paragraph, chunks[0]);

        // Content area
        let content_block = Block::default()
            .borders(Borders::NONE);

        // Build the content text with proper formatting
        let mut content_lines = Vec::new();
        
        for block in &chapter.blocks {
            match block {
                RenderableBlock::Paragraph(text) => {
                    // For paragraphs, we'll wrap the text and add it as multiple lines
                    let wrapped_lines = wrap_text(text, size.width as usize - 2); // -2 for borders/padding
                    content_lines.extend(wrapped_lines);
                    // Add an empty line after each paragraph
                    content_lines.push(String::new());
                }
                RenderableBlock::Heading(level, text) => {
                    // Add an empty line before headings
                    content_lines.push(String::new());
                    
                    // For headings, we'll add the text with appropriate markers
                    let heading_text = match level {
                        1 => format!("# {}", text),
                        2 => format!("## {}", text),
                        3 => format!("### {}", text),
                        4 => format!("#### {}", text),
                        5 => format!("##### {}", text),
                        _ => format!("###### {}", text),
                    };
                    
                    content_lines.push(heading_text);
                    // Add an empty line after headings
                    content_lines.push(String::new());
                }
            }
        }

        // Convert to a single string with newlines
        let content_text = content_lines.join("\n");
        
        // Create the content paragraph with scrolling
        let content_paragraph = Paragraph::new(content_text)
            .block(content_block)
            .wrap(Wrap { trim: false })
            .scroll((scroll_position as u16, 0));

        frame.render_widget(content_paragraph, chunks[1]);

        // Footer with progress
        let progress_text = format!("Progress: {:.1}% | Scroll: {}", progress * 100.0, scroll_position);
        let footer_block = Block::default()
            .borders(Borders::ALL)
            .title(progress_text);
        
        let footer_paragraph = Paragraph::new("")
            .block(footer_block);
        
        frame.render_widget(footer_paragraph, chunks[2]);
    }
}

// Helper function to wrap text to fit within a specified width
fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in text.split_whitespace() {
        // Check if adding this word would exceed the width
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        
        if test_line.len() <= width {
            current_line = test_line;
        } else {
            // If the current line is not empty, add it to lines
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = word.to_string();
            } else {
                // If the word itself is longer than width, we need to split it
                if word.len() > width {
                    // Add as much as we can to the current line
                    let (first_part, rest) = word.split_at(width);
                    lines.push(first_part.to_string());
                    
                    // Handle the rest of the word
                    let mut remaining = rest;
                    while remaining.len() > width {
                        let (part, rest) = remaining.split_at(width);
                        lines.push(part.to_string());
                        remaining = rest;
                    }
                    if !remaining.is_empty() {
                        current_line = remaining.to_string();
                    }
                } else {
                    current_line = word.to_string();
                }
            }
        }
    }
    
    // Add the last line if it's not empty
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    // If no lines were added (empty text), add an empty line
    if lines.is_empty() {
        lines.push(String::new());
    }
    
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text() {
        let text = "This is a test paragraph with several words that should be wrapped appropriately.";
        let wrapped = wrap_text(text, 20);
        
        // Check that we have multiple lines
        assert!(wrapped.len() > 1);
        
        // Check that each line is within the width limit
        for line in wrapped.iter() {
            assert!(line.len() <= 20);
        }
    }

    #[test]
    fn test_wrap_text_short_width() {
        let text = "This is a test";
        let wrapped = wrap_text(text, 5);
        
        // Check that we have multiple lines
        assert!(wrapped.len() >= 2);
        
        // Check that each line is within the width limit
        for line in wrapped.iter() {
            assert!(line.len() <= 5);
        }
    }

    #[test]
    fn test_wrap_text_empty() {
        let text = "";
        let wrapped = wrap_text(text, 10);
        
        // Should have at least one line (empty line)
        assert!(!wrapped.is_empty());
    }
}