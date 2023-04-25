use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::core::options::Options;
use tracing::{event, Level};

use super::{grid::Export, BaseRunner, GridElement, IntoHelper, Runner};
enum Command {
    RESET,
    STOP,
}
pub(crate) struct ThreadRunner {
    handle: JoinHandle<()>,
    tx_opts: Sender<Options>,
    tx_command: Sender<Command>,
    tx_elem: Sender<Box<dyn IntoHelper>>,
    rx_export: Receiver<Export>,
    last_expt: Export,
}
struct RunnerHandle {
    runner: BaseRunner,
    tx_export: Sender<Export>,
    rx_opts: Receiver<Options>,
    rx_command: Receiver<Command>,
    rx_elem: Receiver<Box<dyn IntoHelper>>,
}
impl RunnerHandle {
    fn run(&mut self) {
        loop {
            println!("Running! {:#?}", self.tx_export);
            let deadline = Duration::new(0, 500000);
            match self.rx_command.recv_timeout(deadline) {
                Err(_) => (),
                Ok(command) => match command {
                    Command::STOP => break,
                    Command::RESET => self.runner.reset(),
                },
            }
            match self.rx_opts.recv_timeout(deadline) {
                Err(_) => (),
                Ok(opts) => self.runner.set_opts(opts),
            }
            match self.rx_elem.recv_timeout(deadline) {
                Err(_) => (),
                Ok(elem_box) => self.runner.put_raw(elem_box.into()),
            }
            self.runner.run_dynamic();

            if self.tx_export.send(self.runner.export()).is_err() {
                break;
            };
        }
    }
}
impl Runner for ThreadRunner {
    fn set_opts(&mut self, options: Options) {
        self.tx_opts.send(options).expect("TODO: panic message");
    }
    fn export(&mut self) -> Export {
        match self.rx_export.recv() {
            Ok(export) => self.last_expt = export,
            Err(err) => event!(Level::WARN, "Error {:#?}", err),
        }
        return self.last_expt.clone();
    }
    fn reset(&mut self) {
        self.tx_command
            .send(Command::RESET)
            .expect("TODO: panic message");
    }
    fn put<T: GridElement + 'static>(&mut self, elem: T) {
        self.tx_elem
            .send(Box::new(elem))
            .expect("TODO: panic message");
    }
}
impl ThreadRunner {
    pub fn new(rows: usize, cols: usize, options: Options) -> Self {
        let (tx_opts, rx_opts) = mpsc::channel();
        let (tx_command, rx_command) = mpsc::channel();
        let (tx_elem, rx_elem) = mpsc::channel();
        let (tx_export, rx_export) = mpsc::channel();
        let handle = thread::spawn(move || {
            RunnerHandle {
                runner: BaseRunner::new(rows, cols, options),
                tx_export,
                rx_opts,
                rx_command,
                rx_elem,
            }
            .run();
        });
        let export = rx_export.recv().unwrap();
        return ThreadRunner {
            handle,
            tx_opts,
            tx_command,
            rx_export,
            tx_elem,
            last_expt: export,
        };
    }
}
