use clap::{Parser};
use dotenv::dotenv;
use reqwest::{blocking::Client, header::COOKIE};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long)]
    day: String,

    #[clap(long)]
    current_working_directory: Option<PathBuf>,
}

fn clean_day(day_str: &str) -> Result<u8, String> {
    let day_str = day_str.strip_prefix("day-").unwrap_or(day_str);
    let day_str = day_str.trim_start_matches('0');
    let day_str = if day_str.is_empty() { "0" } else { day_str };
    day_str.parse::<u8>().map_err(|_| format!("Invalid day: {}", day_str))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = Args::parse();

    let session = std::env::var("SESSION")
        .expect("SESSION environment variable not set in .env");

    let day_num = clean_day(&args.day)?;
    let url = format!("https://adventofcode.com/2024/day/{}/input", day_num);

    let client = Client::new();
    let res = client
        .get(&url)
        .header(COOKIE, format!("session={}", session))
        .send()?
        .error_for_status()?;

    let body = res.text()?;

    // Determinar directorio base
    let cwd = args.current_working_directory.unwrap_or_else(|| std::env::current_dir().unwrap());
    // Carpeta del día: ./day-XX/
    let day_folder = cwd.join(&args.day);

    // Crear carpeta si no existe
    if !day_folder.exists() {
        fs::create_dir_all(&day_folder)?;
    }

    // Guardar dos archivos con mismo contenido
    for filename in ["input1.txt", "input2.txt"] {
        let input_path = day_folder.join(filename);
        let mut file = File::create(&input_path)?;
        file.write_all(body.as_bytes())?;
        println!("Wrote {}", input_path.display());
    }

    println!("Input for day {} saved.", args.day);

    Ok(())
}
