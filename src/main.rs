use clap::Parser;
use mp4;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::{fs, os::unix::thread, path::PathBuf};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    episode_path: std::path::PathBuf,
    bumper_path: std::path::PathBuf,
}

#[derive(Debug)]
struct Program<'a> {
    path: std::path::PathBuf,
    length: Duration,
    p_type: &'a str,
}

fn list_files(path: std::path::PathBuf) -> Vec<PathBuf> {
    let file_itr = fs::read_dir(path).unwrap();
    let names = file_itr
        .filter_map(|entry| entry.ok().and_then(|e| Some(e.path())))
        .collect::<Vec<PathBuf>>();
    names
}

fn get_duration(path: &PathBuf) -> Duration {
    let mut f = File::open(path).unwrap();
    let size = f.metadata().unwrap().len();
    let reader = BufReader::new(f);

    let mp4 = mp4::Mp4Reader::read_header(reader, size).unwrap();

    mp4.duration()
}

fn make_schedule(
    episode_path: std::path::PathBuf,
    bumper_path: std::path::PathBuf,
) {
    let mut ep_names = list_files(episode_path);
    let mut bump_names = list_files(bumper_path);

    let schedule_length: usize = if ep_names.len() <= bump_names.len() {
        ep_names.len() * 2
    } else {
        bump_names.len() * 2
    };

    let mut schedule: Vec<Program> = Vec::with_capacity(schedule_length);

    let i = 0;
    ep_names.shuffle(&mut thread_rng());
    bump_names.shuffle(&mut thread_rng());

    while i < schedule_length && !ep_names.is_empty() && !bump_names.is_empty()
    {
        match ep_names.pop() {
            Some(item) => {
                let ep_time = get_duration(&item);
                schedule.push(Program {
                    path: item,
                    p_type: "episode",
                    length: ep_time,
                })
            }
            None => break,
        }
        match bump_names.pop() {
            Some(item) => {
                let bump_time = get_duration(&item);
                schedule.push(Program {
                    path: item,
                    p_type: "bumper",
                    length: bump_time,
                })
            }
            None => break,
        }
        // match bump_names.pop() {
        //     Some(item) => schedule.push(item),
        //     None => break,
        // }
    }
    println!("{:?}", schedule);
}
fn main() {
    let args = Cli::parse();
    make_schedule(args.episode_path, args.bumper_path);
}
