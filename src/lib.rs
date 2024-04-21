pub mod handlers;

use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use entity::prelude::*;
use futures::executor::block_on;
use migration::Migrator;
use sea_orm::{Database, DatabaseBackend, DatabaseConnection, DbErr};

#[derive(Debug, Clone, Copy)]
pub struct PoolCreationError;

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

async fn run() -> Result<DatabaseConnection, DbErr> {
    let database_url = dotenvy::var("DATABASE_URL").unwrap();

    let db = Database::connect(&database_url).await?;
    // Migrator::up(&db, None).await?;

    // let user = Users::find_by_id(1).one(&db).await?;
    // println!("User is {:#?}", user);
    // dbg!(&user);

    Ok(db)
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            let db = block_on(run()).unwrap();
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job(&db);
                    }

                    Err(_) => {
                        println!("No senders to receive from");
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

type Job = Box<dyn FnOnce(&DatabaseConnection) + Send + 'static>;

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            0 => Err(PoolCreationError),
            _ => Ok(ThreadPool::new(size)),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(&DatabaseConnection) + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
