use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "YouTube URL")]
    pub url: String,

    #[arg(short='o', long = "output", help = "Output file")]
    pub output: String,

    #[arg(short='q', long = "quality", help = "Quality (percentage)")]
    pub quality: Option<String>,
    
    #[arg(short = 't', long = "trim", help = "Trim (ex: 0:00,1:00)", value_parser = trim_parser)]
    pub trim: (String, String),
}

pub fn trim_parser(arg: &str) -> Result<(String, String), String> {
    if let Some(split) = arg.split_once(',') {
        return Ok((split.0.to_string(), split.1.to_string()));
    }
    Err("Invalid formatting. For example: 0 to 10 seconds is \"0:00,0:10\"".to_string())
}

fn main() {
    let args = Args::parse();

    let out_name = Path::new(&args.output).with_extension("").with_extension("").to_string_lossy().to_string();
    let out_ext = Path::new(&args.output).extension().unwrap_or(OsStr::new("webm")).to_string_lossy();

    let dlp_out = out_name.to_string() + ".webm";

    println!("## Downloading video");
    Command::new("yt-dlp")
        .args([
            &args.url,
            "--download-sections", &format!("*{start}-{end}", start=args.trim.0, end=args.trim.1),
            "--output", &dlp_out
        ])
        .spawn().unwrap()
        .wait_with_output().unwrap();
    
    if out_ext != "webm" || args.quality.is_some() {
        let quality = args.quality.unwrap_or("100%".to_string());
        let quality = quality.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u8>().unwrap();
        
        println!("## Converting '{dlp_out}' to {} (quality={quality}%)", &args.output);
        let v_bitrate = 12.0 * (quality as f32 / 100.0);  // MB per second
        Command::new("ffmpeg")
            .args([
                "-i", &dlp_out,
                &args.output,
                &if quality < 100 { format!("-b:v={}M", v_bitrate as u8) } else { String::new() },
                if quality >= 100 { "-c=copy" } else { "" },
                "-y"
            ])
            .spawn().unwrap()
            .wait_with_output().unwrap();
        
        println!("## Removing original yt-dlp file");
        std::fs::remove_file(&dlp_out).unwrap();
    }
}
