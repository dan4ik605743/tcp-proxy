pub fn getting_data() -> String {
    tracing::info!("Data entry pending...");

    let mut data = String::new();
    std::io::stdin()
        .read_line(&mut data)
        .expect("Failed to read stdin");
    data.trim_end().to_string()
}
