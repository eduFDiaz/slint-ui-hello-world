slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_format_json(move |string| {
    let ui = ui_handle.unwrap();
    let unformatted_json = string.as_str();

    // Parse the string of data into serde_json::Value.
    let v: serde_json::Value = serde_json::from_str(&unformatted_json).unwrap();

    // Convert the serde_json::Value back to a string with pretty formatting.
    let pretty_json = serde_json::to_string_pretty(&v).unwrap();

    // Print the prettily formatted JSON string.
    println!("{}", pretty_json);
        
    ui.set_formattedJson(pretty_json.into());
    });

    ui.run()
}
