use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: u8,
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
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a new task
    Add { description: String },
    /// Udpate a task's description
    Update { id: u8, description: String },
    /// Delete a task
    Delete { id: u8 },
    /// Mark a task as in progress
    MarkInProgress { id: u8 },
    /// Mark a task as done
    MarkDone { id: u8 },
    /// List all tasks
    List {
        #[command(subcommand)]
        status: Option<TaskStatus>,
    },
}

fn main() {
    let _cli = Cli::parse();
}
