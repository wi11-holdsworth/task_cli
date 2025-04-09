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

fn print_tasks(tasks: &mut Vec<Task>, status: Option<TaskStatus>) {
    for task in tasks {
        // todo: pretty print tasks
        match status {
            Some(status) => if task.status == status {},
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
    let task = tasks.get_mut(*id).expect("task not found");
    task.description = description;
}

fn delete_task(id: &usize, tasks: &mut Vec<Task>) {
    tasks.remove(*id);
}

fn mark_task(id: &usize, status: TaskStatus, tasks: &mut Vec<Task>) {
    // todo: is panicking here bad?
    let task: &mut Task = tasks.get_mut(*id).expect("task not found");
    task.status = status;
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
        Commands::MarkInProgress { id } => mark_task(id, TaskStatus::InProgress, &mut tasks),
        Commands::MarkDone { id } => mark_task(id, TaskStatus::InProgress, &mut tasks),
        Commands::List { status } => match &status {
            Some(status) => match status {
                TaskStatus::Todo => print_tasks(&mut tasks, Some(TaskStatus::Todo)),
                TaskStatus::InProgress => print_tasks(&mut tasks, Some(TaskStatus::InProgress)),
                TaskStatus::Done => print_tasks(&mut tasks, Some(TaskStatus::Done)),
            },
            None => print_tasks(&mut tasks, None),
        },
    }
}
