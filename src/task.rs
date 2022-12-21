use crate::echo;
use crate::package::Package;
use crate::utils::Spinner;
use crate::Error;

use crossbeam_channel::{bounded, select, Receiver};
use crossbeam_utils::sync::WaitGroup;
use signal_hook::iterator::Signals;
use std::fs;
use std::io;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use termion::{color, terminal_size};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("No plugins to sync")]
    NoPlugins,
    #[error("Terminal error")]
    TerminalError(#[from] std::io::Error),
    #[error("Fail to get terminal size.")]
    TerminalToSmall,
}

pub enum TaskType {
    Install,
    Update,
}

pub struct TaskManager {
    task_type: TaskType,
    packs: Vec<Package>,
    thread_num: usize,
}

impl TaskManager {
    pub fn new(task_type: TaskType, thread_num: usize) -> TaskManager {
        TaskManager {
            task_type,
            packs: Vec::new(),
            thread_num,
        }
    }

    pub fn add(&mut self, pack: Package) {
        self.packs.push(pack);
    }

    /// returns true on success otherwise false
    fn update<F>(pack: &Package, line: u16, func: F) -> bool
    where
        F: Fn(&Package) -> (Result<(), Error>, bool),
    {
        let msg = format!(" [{}]", &pack.name);
        let pos = msg.len() as u16;
        echo::message(line, 0, &format!("    {} syncing", &msg));

        const MSG_MARGIN: u16 = 5;
        const SIGN_MARGIN: u16 = 3;

        macro_rules! print_err {
            ($err:expr) => {
                let msg = format!("{}", $err);
                echo::character(line, SIGN_MARGIN, '✗', color::Red);
                echo::inline_message(line, MSG_MARGIN + pos, &msg);
            };
        }

        let mut successful = true;
        let spinner = Spinner::spin(line, SIGN_MARGIN);
        if let (Err(e), status) = func(pack) {
            spinner.stop();
            print_err!(e);
            successful = status;
        } else {
            if pack.build_command.is_some() {
                echo::inline_message(line, MSG_MARGIN + pos, "building");
                if let Err(e) = pack.try_build().map_err(|e| Error::build(format!("{}", e))) {
                    print_err!(e);
                }
            }

            spinner.stop();
            if successful {
                echo::character(line, SIGN_MARGIN, '✓', color::Green);
                echo::inline_message(line, MSG_MARGIN + pos, "done");
            }
        }
        successful
    }

    pub fn run<F>(self, func: F) -> Result<Vec<String>, TaskError>
    where
        F: Fn(&Package) -> (Result<(), Error>, bool) + Send + 'static + Copy,
    {
        if self.packs.is_empty() {
            return Err(TaskError::NoPlugins);
        }

        let (y, _x) = terminal_size()?;

        if y <= 2 {
            return Err(TaskError::TerminalToSmall);
        }

        let quit_notifier = setup_signal()?;

        let threads = self.thread_num;

        let wg = WaitGroup::new();
        let (tx, rx) = bounded::<Option<Package>>(threads);

        let failures = Arc::new(Mutex::new(vec![]));
        let pending = Arc::new(Mutex::new(vec![]));

        for _ in 0..threads {
            let rx = rx.clone();
            let failures = failures.clone();
            let pending = pending.clone();
            let wg = wg.clone();
            let quit_notifier = quit_notifier.clone();
            thread::spawn(move || {
                while let Ok(Some(pack)) = rx.recv() {
                    log::info!("pack {}", &pack.name);
                    let _wg = wg.clone();
                    {
                        let mut p = pending.lock().unwrap();
                        log::info!("add to pending:{}", &pack.name);
                        p.push(pack.clone());
                    }

                    let name = pack.name.clone();
                    let failures = failures.clone();

                    let (wtx, wrx) = bounded(0);
                    thread::spawn(move || {
                        let index = echo::line();
                        if !Self::update(&pack, index, func) {
                            let mut f = failures.lock().unwrap();
                            f.push(pack.name);
                        }
                        let _ = wtx.send(());
                    });
                    select! {
                        recv(wrx) -> _ => {},
                        recv(quit_notifier) -> _ => {
                            log::info!("quit received {}", &name);
                            return;
                        }
                    }
                    {
                        let mut p = pending.lock().unwrap();
                        log::info!("remove from pending: {}", &name);
                        p.retain(|x| x.name != name);
                    }
                }
            });
        }
        if !self.packs.is_empty() {
            println!();
        }

        for pack in self.packs.iter() {
            let _ = tx.send(Some(pack.clone()));
        }

        for _ in 0..threads {
            let _ = tx.send(None);
        }
        wg.wait();

        if !self.packs.is_empty() {
            println!();
        }

        log::info!("quit");

        helptags();

        if let TaskType::Install = self.task_type {
            for p in pending.lock().unwrap().iter() {
                log::info!("delete {:?}", p.path());
                let _ = fs::remove_dir_all(p.path());
            }
        }

        let failures = failures.lock().unwrap();
        Ok(failures.clone())
    }
}

fn helptags() {
    match process::Command::new("vim")
        .arg("--not-a-term")
        .arg("-c")
        .arg("silent! helptags ALL")
        .stdout(process::Stdio::null())
        .spawn()
    {
        Ok(_) => (),
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                process::Command::new("nvim")
                    .arg("--headless")
                    .arg("-c")
                    .arg("silent! helptags ALL")
                    .stdout(process::Stdio::null())
                    .spawn()
                    .expect("Error opening nvim");
            } else {
                panic!("Somthing happened when calling vim!")
            }
        }
    }
}

fn setup_signal() -> io::Result<Receiver<()>> {
    let (s, r) = bounded(10);
    let mut signals = Signals::new([signal_hook::consts::SIGTERM, signal_hook::consts::SIGINT])?;

    thread::spawn(move || {
        if signals.forever().next().is_some() {
            drop(s);
        }
    });
    Ok(r)
}
