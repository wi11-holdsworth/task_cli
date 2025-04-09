use std::ops::Index;

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

#[derive(Debug, Deserialize, Serialize, Subcommand)]
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
    let task = tasks.get_mut(*id).expect("task not found");
    task.description = description;
}

fn delete_task(id: &usize, tasks: &mut Vec<Task>) {
    tasks.remove(*id);
}

fn main() {
    let cli = Cli::parse();

    let mut tasks: Vec<Task> = Vec::new();

    match &cli.command {
        Commands::Add { description } => add_task(description.to_string(), &mut tasks),
        Commands::Update { id, description } => {
            update_task(id, description.to_string(), &mut tasks)
        }
        Commands::Delete { id } => delete_task(id, &mut tasks),
        Commands::MarkInProgress { id } => todo!(),
        Commands::MarkDone { id } => todo!(),
        Commands::List { status } => match &status {
            Some(status) => match status {
                TaskStatus::Todo => todo!(),
                TaskStatus::InProgress => todo!(),
                TaskStatus::Done => todo!(),
            },
            None => todo!(),
        },
    }
}
