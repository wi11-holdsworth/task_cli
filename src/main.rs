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
    // todo: is panicking here bad?
    let task = tasks.get_mut(*id - 1).expect("task not found");
    task.description = description;
}

fn delete_task(id: &usize, tasks: &mut Vec<Task>) {
    if 0 < *id && *id < tasks.len() + 1 {
        tasks.remove(*id - 1);
    } else {
        let min_val = if tasks.len() != 0 { 1 } else { 0 };
        println!(
            "invalid task selected. the available range is currently [{}, {}]",
            min_val,
            tasks.len()
        );
    }
}

fn mark_task(id: &usize, status: TaskStatus, tasks: &mut Vec<Task>) {
    // todo: is panicking here bad?
    let task: &mut Task = tasks.get_mut(*id - 1).expect("task not found");
    task.status = status;
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
