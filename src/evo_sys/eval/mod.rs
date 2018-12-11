pub mod eval;
pub mod evaluator;
pub mod registers;

use core::{Message, EvalResult};
use std::sync::mpsc;
use core::config::ThreadDefaults;
use core::config::config::process_thread_defaults;
use std::thread;


pub struct Evaluator {
    job_sender: mpsc::Sender<Message>,
    result_receiver: mpsc::Receiver<EvalResult>,
    handles: Vec<Option<thread::JoinHandle<()>>>,
    current_job_count: u32,
    config: ThreadDefaults,
}

