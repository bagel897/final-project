use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use tracing::{event, Level};

use super::{
    ant_grid::Options, grid::Export, grid_elements::grid_element::GridElement, BaseRunner, Runner,
};
enum Command {
    RESET,
    STOP,
}
pub(crate) struct ThreadRunner {
    handle: JoinHandle<()>,
    tx_opts: Sender<Options>,
    tx_command: Sender<Command>,
    rx_export: Receiver<Export>,
    last_expt: Export,
}
struct RunnerHandle {
    runner: BaseRunner,
    tx_export: Sender<Export>,
    rx_opts: Receiver<Options>,
    rx_command: Receiver<Command>,
}
impl RunnerHandle {
    fn new(
        runner: BaseRunner,
        tx_export: Sender<Export>,
        rx_opts: Receiver<Options>,
        rx_command: Receiver<Command>,
    ) -> Self {
        RunnerHandle {
            runner,
            tx_export,
            rx_opts,
            rx_command,
        }
    }
    fn run(&mut self) {
        loop {
            println!("Running! {:#?}", self.tx_export);
            match self.rx_command.try_recv() {
                Err(_) => (),
                Ok(command) => match command {
                    Command::STOP => break,
                    Command::RESET => self.runner.reset(),
                },
            }
            match self.rx_opts.try_recv() {
                Err(_) => (),
                Ok(opts) => self.runner.set_opts(opts),
            }
            self.runner.run_dynamic();

            if self.tx_export.send(self.runner.export()).is_err() {
                break;
            };
        }
    }
}
impl Runner for ThreadRunner {
    fn put<T: GridElement + 'static>(&mut self, elem: T) {
        todo!();
    }
    fn set_opts(&mut self, options: Options) {
        self.tx_opts.send(options);
    }
    fn export(&mut self) -> Export {
        match self.rx_export.recv() {
            Ok(export) => self.last_expt = export,
            Err(err) => event!(Level::WARN, "Error {:#?}", err),
        }
        return self.last_expt.clone();
    }
    fn reset(&mut self) {
        self.tx_command.send(Command::RESET);
    }
}
impl ThreadRunner {
    pub fn new(rows: usize, cols: usize) -> Self {
        let (tx_opts, rx_opts) = mpsc::channel();
        let (tx_command, rx_command) = mpsc::channel();
        let (tx_export, rx_export) = mpsc::channel();
        let handle = thread::spawn(move || {
            RunnerHandle::new(BaseRunner::new(rows, cols), tx_export, rx_opts, rx_command).run();
        });
        let export = rx_export.recv().unwrap();
        return ThreadRunner {
            handle,
            tx_opts,
            tx_command,
            rx_export,
            last_expt: export,
        };
    }
    pub fn stop(&mut self) {
        self.tx_command.send(Command::STOP);
        // self.handle;
    }
}
