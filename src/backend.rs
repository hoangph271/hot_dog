use dioxus::prelude::*;

#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    _ = file.write_fmt(format_args!("{image}\n"));

    Ok(())
}
