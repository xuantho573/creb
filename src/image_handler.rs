pub fn create_image_widget(
  _image_path: &str,
) -> Result<ratatui_image::StatefulImage<ratatui_image::protocol::StatefulProtocol>, String> {
  // Create a StatefulImage
  let image_widget = ratatui_image::StatefulImage::new();

  Ok(image_widget)
}
