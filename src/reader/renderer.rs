use crate::epub::content::{RenderableBlock, RenderableChapter};
use ratatui::{
  layout::{Constraint, Direction, Layout},
  style::{Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders, Paragraph, Wrap},
};
use ratatui_image::{StatefulImage, picker::Picker};

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
        Constraint::Length(3), // Header
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Footer
      ])
      .split(size);

    // Header with title
    let title_block = Block::default().borders(Borders::ALL).title(title);

    let title_paragraph = Paragraph::new("").block(title_block);

    frame.render_widget(title_paragraph, chunks[0]);

    // Content area
    let content_block = Block::default().borders(Borders::NONE);

    // Build the content with proper formatting using Lines and Spans
    let mut content_lines: Vec<Line> = Vec::new();

    for block in &chapter.blocks {
      match block {
        RenderableBlock::Paragraph(text) => {
          // Add an empty line before paragraph for spacing
          content_lines.push(Line::from(""));

          // For paragraphs, we'll wrap the text and add it as multiple lines
          let wrapped_lines = wrap_text(text, size.width as usize - 2); // -2 for borders/padding
          for line in wrapped_lines {
            content_lines.push(Line::from(line));
          }

          // Add an empty line after paragraph for spacing
          content_lines.push(Line::from(""));
        }
        RenderableBlock::Heading(level, text) => {
          // Add an empty line before heading for spacing
          content_lines.push(Line::from(""));

          // For headings, we'll add the text with appropriate styling
          let (heading_prefix, heading_suffix, style) = match level {
            1 => (
              "=".repeat(std::cmp::min(5, size.width as usize / 4)),
              "=".repeat(std::cmp::min(5, size.width as usize / 4)),
              Style::default().add_modifier(Modifier::BOLD),
            ),
            2 => (
              "-".repeat(std::cmp::min(3, size.width as usize / 6)),
              "-".repeat(std::cmp::min(3, size.width as usize / 6)),
              Style::default().add_modifier(Modifier::BOLD),
            ),
            3 => (
              "###".to_string(),
              "".to_string(),
              Style::default().add_modifier(Modifier::BOLD),
            ),
            4 => (
              "####".to_string(),
              "".to_string(),
              Style::default().add_modifier(Modifier::UNDERLINED),
            ),
            5 => (
              "#####".to_string(),
              "".to_string(),
              Style::default().add_modifier(Modifier::UNDERLINED),
            ),
            _ => ("######".to_string(), "".to_string(), Style::default()),
          };

          let heading_line = Line::from(vec![
            Span::raw(" "),
            Span::styled(heading_prefix.clone(), style),
            Span::raw(" "),
            Span::styled(text.clone(), style),
            Span::raw(" "),
            Span::styled(heading_suffix.clone(), style),
            Span::raw(" "),
          ]);

          content_lines.push(heading_line);

          // Add an empty line after heading for spacing
          content_lines.push(Line::from(""));
        }
        RenderableBlock::Image(path) => {
          // Add an empty line before image for spacing
          content_lines.push(Line::from(""));

          // Add image info with special styling
          content_lines.push(Line::from(vec![
            Span::raw("[Image: "),
            Span::styled(
              path.clone(),
              Style::default().add_modifier(Modifier::ITALIC),
            ),
            Span::raw("]"),
          ]));
          content_lines.push(Line::from(
            "(Press 'i' when this line is visible to view the image)",
          ));

          // Add an empty line after image for spacing
          content_lines.push(Line::from(""));
        }
        RenderableBlock::ImagePlaceholder(description) => {
          // Add an empty line before image for spacing
          content_lines.push(Line::from(""));

          // Add image placeholder info
          content_lines.push(Line::from(vec![
            Span::raw("[Image: "),
            Span::styled(
              description.clone(),
              Style::default().add_modifier(Modifier::ITALIC),
            ),
            Span::raw("]"),
          ]));

          // Add an empty line after image for spacing
          content_lines.push(Line::from(""));
        }
      }
    }

    // Create the content paragraph with scrolling
    let content_paragraph = Paragraph::new(content_lines)
      .block(content_block)
      .wrap(Wrap { trim: false })
      .scroll((scroll_position as u16, 0));

    frame.render_widget(content_paragraph, chunks[1]);

    // Footer with progress
    let progress_text = format!(
      "Progress: {:.1}% | Scroll: {}",
      progress * 100.0,
      scroll_position
    );
    let footer_block = Block::default().borders(Borders::ALL).title(progress_text);

    let footer_paragraph = Paragraph::new("").block(footer_block);

    frame.render_widget(footer_paragraph, chunks[2]);
  }

  pub fn render_image(
    frame: &mut ratatui::Frame,
    image_path: &str,
    title: &str,
    progress: f64,
    scroll_position: usize,
  ) {
    let size = frame.area();

    // Create the layout sections
    let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
        Constraint::Length(3), // Header
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Footer
      ])
      .split(size);

    // Header with title
    let title_block = Block::default().borders(Borders::ALL).title(title);

    let title_paragraph = Paragraph::new("").block(title_block);

    frame.render_widget(title_paragraph, chunks[0]);

    let picker = Picker::from_fontsize((8, 12));

    // Load an image with the image crate.
    let dyn_img = image::ImageReader::open(image_path)
      .unwrap()
      .decode()
      .unwrap();

    // Create the Protocol which will be used by the widget.
    let mut image = picker.new_resize_protocol(dyn_img);

    frame.render_stateful_widget(StatefulImage::default(), chunks[1], &mut image);

    // Footer with progress
    let progress_text = format!(
      "Progress: {:.1}% | Scroll: {}",
      progress * 100.0,
      scroll_position
    );
    let footer_block = Block::default().borders(Borders::ALL).title(progress_text);

    let footer_paragraph = Paragraph::new("").block(footer_block);

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
