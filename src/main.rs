use std::{env, ffi::OsStr, io::Write, os::windows::ffi::OsStrExt, path::Path, process::{Command, Stdio}};
use chrono::Local;
use windows_sys::Win32::Media::Audio::{PlaySoundW, SND_FILENAME, SND_MEMORY, SND_SYNC};

static DEFAULT_SOUND: &[u8] = include_bytes!("../Aria_task_finished.wav");

fn wide(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(std::iter::once(0)).collect()
}

fn main() {
    let mut args = env::args_os();
    args.next();

    unsafe {
        match args.next() {
            Some(path) => {
                let wide_path = wide(Path::new(&path).as_os_str());
                if PlaySoundW(wide_path.as_ptr(), std::ptr::null_mut(), SND_FILENAME | SND_SYNC) == 0 {
                    std::process::exit(1);
                }
            }
            None => {
                if PlaySoundW(DEFAULT_SOUND.as_ptr().cast(), std::ptr::null_mut(), SND_MEMORY | SND_SYNC) == 0 {
                    std::process::exit(1);
                }
            }
        }
    }

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{timestamp}");

    let mut child = Command::new("clip")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start clip.exe");
    child.stdin.as_mut().unwrap().write_all(timestamp.as_bytes()).expect("Failed to write timestamp to clipboard");
    if !child.wait().expect("Failed to wait for clip.exe").success() {
        std::process::exit(1);
    }
}
