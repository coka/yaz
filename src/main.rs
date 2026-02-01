use std::io::Write;

fn main() {
    print!("What's on your mind?\n> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let dir = std::path::PathBuf::from(std::env::var("HOME").unwrap()).join(".yaz");
    std::fs::create_dir_all(&dir).unwrap();
    writeln!(
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join("notes.txt"))
            .unwrap(),
        "[{}] {}",
        chrono::Local::now().to_rfc2822(),
        input.trim()
    )
    .unwrap();
    println!("Thought persisted.");
}
