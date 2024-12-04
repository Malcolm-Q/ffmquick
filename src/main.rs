use std::env;
use std::process::Command;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage:\n\t-half_resolution <file_path>\n\t\tRe-encode video to mp4 and half the resolution.");
        eprintln!("\t-strip_audio <file_path>\n\t\tStrip audio");
        eprintln!("\t-re_encode <file_path>\n\t\tRe encode the video to mp4");
        eprintln!("\t-trim <file_path>\n\t\tTrim the video to a start and end time");
        eprintln!("\t-specify_resolution <file_path>\n\t\tSpecify the width and height");
        eprintln!("\t-convert_to_gif <file_path>\n\t\tMake a gif specifying the scale and fps");
        eprintln!("\t-change_file_type <file_path>\n\t\tChange video type ex from .mp4 to .mov");
        std::process::exit(1);
    } else if args.len() < 3 {
        eprintln!("You need to provide a file path!");
        std::process::exit(1);
    }

    map_command(&args[1], &args[2]);
}

fn map_command(cmd: &str, file_path: &str) {
    match cmd {
        "-half_resolution" => half_resolution(file_path),
        "-strip_audio" => strip_audio(file_path),
        "-re_encode" => re_encode(file_path),
        "-trim" => trim(file_path),
        "-specify_resolution" => specify_resolution(file_path),
        "-convert_to_gif" => convert_to_gif(file_path),
        "-change_file_type" => change_file_type(file_path),
        _ => eprintln!("Unknown command: {}", cmd),
    }
}

fn change_file_type(file_path: &str) {
    let parts: Vec<&str> = get_file_parts(file_path);

    print!("Enter a valid video or audio extension (EX: .mp3): ");
    io::stdout().flush().unwrap();
    let mut extension = String::new();
    io::stdin().read_line(&mut extension).expect("Failed to read line");
    let extension = extension.trim();
    let extension = if !extension.starts_with('.') {
        format!(".{}", extension)
    } else {
        extension.to_string()
    };
    let output_file = format!("{}_{}_to_{}", parts[1], parts[0], extension);

    run_ffmpeg(vec!["-i", file_path, &output_file])
}

fn half_resolution(file_path: &str) {
    let parts: Vec<&str> = get_file_parts(file_path);
    let output_file = format!("{}_half.{}", parts[1], parts[0]);
    run_ffmpeg(vec!["-i", file_path, "-vf", "scale=iw/2:ih/2", &output_file]);
}

fn strip_audio(file_path: &str){
    let parts: Vec<&str> = get_file_parts(file_path);
    let output_file = format!("{}_no_audio.{}", parts[1], parts[0]);
    run_ffmpeg(vec!["-i", file_path, "-an", &output_file]);
}

fn re_encode(file_path: &str){
    let parts: Vec<&str> = get_file_parts(file_path);
    let output_file = format!("{}_simple.{}", parts[1], parts[0]);
    run_ffmpeg(vec!["-i", file_path, &output_file]);
}

fn specify_resolution(file_path: &str){
    let parts: Vec<&str> = get_file_parts(file_path);
    let output_file = format!("{}_res.{}", parts[1], parts[0]);

    print!("Enter a width (press enter for 1920): ");
    io::stdout().flush().unwrap();
    let mut width = String::new();
    io::stdin().read_line(&mut width).expect("Failed to read line");
    let mut width = width.trim();
    if width.is_empty() {
        width = "1920";
    }

    print!("Enter a height (press enter for 1080): ");
    io::stdout().flush().unwrap();
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read line");
    let mut height = height.trim();
    if height.is_empty() {
        height = "1080";
    }

    run_ffmpeg(vec!["-i", file_path, "-vf", &format!("scale={}:{}", width, height), &output_file]);
}

fn trim(file_path: &str){
    let parts: Vec<&str> = get_file_parts(file_path);
    let output_file = format!("{}_trimmed.{}", parts[1], parts[0]);
    let mut args = vec!["-i", file_path];

    print!("Enter start time (hh:mm:ss or seconds or press enter to skip): ");
    io::stdout().flush().unwrap();
    let mut start_time = String::new();
    io::stdin().read_line(&mut start_time).expect("Failed to read line");
    let start_time = start_time.trim();

    print!("Enter end time (hh:mm:ss or seconds or press enter to skip): ");
    io::stdout().flush().unwrap();
    let mut end_time = String::new();
    io::stdin().read_line(&mut end_time).expect("Failed to read line");
    let end_time = end_time.trim();
    
    if !start_time.is_empty(){
        args.push("-ss");
        args.push(start_time);
    }
    if !end_time.is_empty(){
        args.push("-to");
        args.push(end_time);
    }
    args.push(&output_file);
    
    run_ffmpeg(args);
}

fn convert_to_gif(file_path: &str) {
    let parts: Vec<&str> = get_file_parts(file_path);
    let output_file = format!("{}.gif", parts[1]);

    print!("Enter fps (press enter for 10): ");
    io::stdout().flush().unwrap();
    let mut fps = String::new();
    io::stdin().read_line(&mut fps).expect("Failed to read line");
    let mut fps = fps.trim();
    if fps.is_empty() {
        fps = "10";
    }

    print!("Enter scale (press enter for 320): ");
    io::stdout().flush().unwrap();
    let mut scale = String::new();
    io::stdin().read_line(&mut scale).expect("Failed to read line");
    let mut scale = scale.trim();
    if scale.is_empty() {
        scale = "320";
    }

    run_ffmpeg(vec!["-i", file_path, "-vf", &format!("fps={},scale={}:-1", fps, scale), "-gifflags", "+transdiff", "-y", &output_file]);
}

fn run_ffmpeg(args: Vec<&str>) {
    let status = Command::new("ffmpeg")
        .args(args)
        .status()
        .expect("Failed to execute ffmpeg command");
    if !status.success() {
        eprintln!("Failed to halve resolution of file!")
    }
}

fn get_file_parts(file_path: &str) -> Vec<&str> {
    let parts: Vec<&str> = file_path.rsplitn(2, '.').collect();
    if parts.len() != 2 {
        eprintln!("Invalid file path: {}", file_path);
        std::process::exit(1);
    }
    parts
}

