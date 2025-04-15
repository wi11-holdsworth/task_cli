use std::{
    fs::{self, File},
    io::BufWriter,
};

use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    description: String,
    status: TaskStatus,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize, Subcommand, PartialEq, Eq, Clone, Copy)]
enum TaskStatus {
    /// Show unstarted tasks
    Todo,
    /// Show in progress tasks
    InProgress,
    /// Show compelted tasks
    Done,
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a new task
    Add { description: String },
    /// Udpate a task's description
    Update { id: usize, description: String },
    /// Delete a task
    Delete { id: usize },
    /// Mark a task as in progress
    MarkInProgress { id: usize },
    /// Mark a task as done
    MarkDone { id: usize },
    /// List all tasks
    List {
        #[command(subcommand)]
        status: Option<TaskStatus>,
    },
}

fn offset_index(id: &usize) -> usize {
    id - 1
}

fn index_in_range(id: &usize, tasks: &mut Vec<Task>) -> bool {
    0 < *id && *id < tasks.len() + 1
}

fn get_task<'a>(id: &usize, tasks: &'a mut Vec<Task>) -> Option<&'a mut Task> {
    if index_in_range(id, tasks) {
        tasks.get_mut(offset_index(id))
    } else {
        println!("Task index {} is out of bounds.", id);
        None
    }
}

fn print_tasks(status: Option<TaskStatus>, tasks: &mut Vec<Task>) {
    for task in tasks {
        // todo: pretty print tasks
        match status {
            Some(status) => {
                if task.status == status {
                    println!("{:?}", task)
                }
            }
            None => println!("{:?}", task),
        }
    }
}

fn add_task(description: String, tasks: &mut Vec<Task>) {
    let task = Task {
        description,
        status: TaskStatus::Todo,
        created_at: Local::now(),
        updated_at: Local::now(),
    };
    tasks.push(task);
}

fn update_task(id: &usize, description: String, tasks: &mut Vec<Task>) {
    if let Some(task) = get_task(id, tasks) {
        task.description = description;
    };
}

fn delete_task(id: &usize, tasks: &mut Vec<Task>) {
    if index_in_range(id, tasks) {
        tasks.remove(offset_index(id));
    } else {
        println!("Task index {} is out of bounds.", id);
    }
}

fn mark_task(id: &usize, status: TaskStatus, tasks: &mut Vec<Task>) {
    if let Some(task) = get_task(id, tasks) {
        task.status = status;
    };
}

fn read_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let s = fs::read_to_string(PATH)?;
    let tasks = serde_json::from_str(&s)?;
    Ok(tasks)
}

fn write_tasks(tasks: Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &tasks)?;
    Ok(())
}

const PATH: &str = "tasks.json";

fn main() {
    let cli = Cli::parse();

    let mut tasks = read_tasks().unwrap_or_else(|_| Vec::new());

    match &cli.command {
        Commands::Add { description } => add_task(description.to_string(), &mut tasks),
        Commands::Update { id, description } => {
            update_task(id, description.to_string(), &mut tasks)
        }
        Commands::Delete { id } => delete_task(id, &mut tasks),
        Commands::MarkInProgress { id } => mark_task(id, TaskStatus::InProgress, &mut tasks),
        Commands::MarkDone { id } => mark_task(id, TaskStatus::Done, &mut tasks),
        Commands::List { status } => match &status {
            Some(status) => match status {
                TaskStatus::Todo => print_tasks(Some(TaskStatus::Todo), &mut tasks),
                TaskStatus::InProgress => print_tasks(Some(TaskStatus::InProgress), &mut tasks),
                TaskStatus::Done => print_tasks(Some(TaskStatus::Done), &mut tasks),
            },
            None => print_tasks(None, &mut tasks),
        },
    }

    write_tasks(tasks).expect("failed to write");
}
