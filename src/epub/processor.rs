// src/epub/processor.rs

use super::content::{RenderableChapter, RenderableBlock};
use xml::reader::{EventReader, XmlEvent};

pub fn process_chapter_html(html_content: &str) -> RenderableChapter {
    let mut blocks = Vec::new();
    let mut current_text = String::new();
    let mut heading_level = 0;
    
    // Preprocess the HTML to make it more parseable
    let processed_html = preprocess_html(html_content);
    
    // Wrap the HTML content in a root element to make it valid XML
    // let wrapped_content = format!("<root>{}</root>", processed_html);
    
    let parser = EventReader::from_str(&processed_html);
    
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) => {
                match name.local_name.as_str() {
                    "h1" => {
                        heading_level = 1;
                        current_text.clear();
                    }
                    "h2" => {
                        heading_level = 2;
                        current_text.clear();
                    }
                    "h3" => {
                        heading_level = 3;
                        current_text.clear();
                    }
                    "h4" => {
                        heading_level = 4;
                        current_text.clear();
                    }
                    "h5" => {
                        heading_level = 5;
                        current_text.clear();
                    }
                    "h6" => {
                        heading_level = 6;
                        current_text.clear();
                    }
                    "p" => {
                        // Start of a paragraph
                        current_text.clear();
                    }
                    _ => {
                        // For other elements, we don't need special handling
                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                        if !current_text.trim().is_empty() {
                            blocks.push(RenderableBlock::Heading(heading_level, current_text.trim().to_string()));
                        }
                        current_text.clear();
                        heading_level = 0;
                    }
                    "p" => {
                        if !current_text.trim().is_empty() {
                            blocks.push(RenderableBlock::Paragraph(current_text.trim().to_string()));
                        }
                        current_text.clear();
                    }
                    _ => {
                        // For other elements, we don't need special handling
                    }
                }
            }
            Ok(XmlEvent::Characters(text)) => {
                current_text.push_str(&text);
            }
            Err(e) => {
                // If we encounter an error, fall back to the simple approach
                eprintln!("XML parsing error: {:?}", e);
                return fallback_processing(html_content);
            }
            _ => {}
        }
    }
    
    // If we didn't find any blocks, use the fallback
    if blocks.is_empty() {
        return fallback_processing(html_content);
    }
    
    RenderableChapter { blocks }
}

fn preprocess_html(html_content: &str) -> String {
    let mut content = html_content.to_string();
    
    // Remove DOCTYPE declaration
    if let Some(pos) = content.find("<!DOCTYPE") {
        if let Some(end_pos) = content[pos..].find('>') {
            content.replace_range(pos..pos+end_pos+1, "");
        }
    }
    
    // Remove XML declaration
    // if let Some(pos) = content.find("<?xml") {
    //     if let Some(end_pos) = content[pos..].find('>') {
    //         content.replace_range(pos..pos+end_pos+1, "");
    //     }
    // }
    
    // Remove namespace declarations from html tag
    content = content.replace(r#"xmlns="http://www.w3.org/1999/xhtml""#, "");
    
    // Handle self-closing tags that might cause issues
    // content = content.replace("/>", ">");
    
    content
}

fn fallback_processing(html_content: &str) -> RenderableChapter {
    // Preprocess the HTML content
    let processed_content = preprocess_html(html_content);
    
    // Simple fallback that treats the entire content as a paragraph
    RenderableChapter {
        blocks: vec![RenderableBlock::Paragraph(processed_content.to_string())],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epub::content::RenderableBlock;

    #[test]
    fn test_simple_html_processing() {
        let html = "<h1>Chapter 1</h1><p>This is a paragraph.</p><h2>Section 1</h2><p>Another paragraph.</p>";
        let chapter = process_chapter_html(html);
        
        assert_eq!(chapter.blocks.len(), 4);
        
        match &chapter.blocks[0] {
            RenderableBlock::Heading(_level, text) => {
                assert_eq!(text, "Chapter 1");
            }
            _ => panic!("Expected heading"),
        }
        
        match &chapter.blocks[1] {
            RenderableBlock::Paragraph(text) => {
                assert_eq!(text, "This is a paragraph.");
            }
            _ => panic!("Expected paragraph"),
        }
        
        match &chapter.blocks[2] {
            RenderableBlock::Heading(_level, text) => {
                assert_eq!(text, "Section 1");
            }
            _ => panic!("Expected heading"),
        }
        
        match &chapter.blocks[3] {
            RenderableBlock::Paragraph(text) => {
                assert_eq!(text, "Another paragraph.");
            }
            _ => panic!("Expected paragraph"),
        }
    }

    #[test]
    fn test_fallback_processing() {
        let html = "This is plain text without HTML tags.";
        let chapter = process_chapter_html(html);
        
        assert_eq!(chapter.blocks.len(), 1);
        
        match &chapter.blocks[0] {
            RenderableBlock::Paragraph(text) => {
                assert_eq!(text, "This is plain text without HTML tags.");
            }
            _ => panic!("Expected paragraph"),
        }
    }

    #[test]
    fn test_realistic_html_processing() {
        let html = r#"<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">
<head><title>Chapter 1</title></head>
<body>
<h1>Chapter 1: The Beginning</h1>
<p>This is the first paragraph of the chapter. It contains some text that should be displayed properly.</p>
<p>This is the second paragraph with more content to test the text wrapping functionality.</p>
<h2>Section 1.1</h2>
<p>This is a paragraph under the first section heading.</p>
</body>
</html>"#;
        
        let chapter = process_chapter_html(html);
        
        // Print debug info
        println!("Number of blocks: {}", chapter.blocks.len());
        for (i, block) in chapter.blocks.iter().enumerate() {
            println!("Block {}: {:?}", i, block);
        }
        
        // We should have at least 5 blocks (3 paragraphs, 2 headings)
        assert!(chapter.blocks.len() >= 5);
        
        // Check that we have the expected headings
        let mut heading_count = 0;
        let mut paragraph_count = 0;
        
        for block in &chapter.blocks {
            match block {
                RenderableBlock::Heading(_level, text) => {
                    heading_count += 1;
                    // Check that the text doesn't contain HTML tags
                    assert!(!text.contains("<"));
                    assert!(!text.contains(">"));
                }
                RenderableBlock::Paragraph(text) => {
                    paragraph_count += 1;
                    // Check that the text doesn't contain HTML tags
                    assert!(!text.contains("<"));
                    assert!(!text.contains(">"));
                }
            }
        }
        
        assert!(heading_count >= 2);
        assert!(paragraph_count >= 3);
    }
    
    #[test]
    fn test_preprocess_html() {
        let html = r#"<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">
<head><title>Test</title></head>
<body>
<p>Test paragraph.</p>
</body>
</html>"#;
        
        let processed = preprocess_html(html);
        assert!(!processed.contains("<?xml"));
        assert!(!processed.contains("<!DOCTYPE"));
        assert!(!processed.contains(r#"xmlns="http://www.w3.org/1999/xhtml""#));
    }
}
