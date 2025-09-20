use clap::{Parser, Subcommand};
use std::collections::HashMap;
use clap::ValueEnum;
use std::time::SystemTime;
use colored::*;
use serde_json;
use serde_derive::{self, Deserialize, Serialize};
use std::fs;
use std::path::Path;

const TODOS_FILE: &str= "todos.json";

fn load_todos() -> HashMap<u32, Todo> {
    if Path::new(TODOS_FILE).exists() {
        let content = fs::read_to_string(TODOS_FILE).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_todos(todos: &HashMap<u32, Todo>) {
    let json = serde_json::to_string_pretty(todos).expect("Failed to serialize todos");
    fs::write(TODOS_FILE, json).expect("Failed to write todos to file");
}
fn get_next_id(todos: &HashMap<u32, Todo>) -> u32 {
    todos.keys().max().unwrap_or(&0)+1
}

#[derive(Parser, Debug)]
#[command(name = "MyApp", version = "1.0", author = "anonymous", about = "An example todo CLI application")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
    created_at: SystemTime,
    complete_at: SystemTime,
    priority: Priority,
}


#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { title: String, description: String, priority: Priority },
    Remove { id: u32 },
    MarkCompleted { id: u32 },
    List
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Add { title, description, priority } => {
            println!("{}  {}", 
                "✨ Adding new todo:".bright_green().bold(),
                title.bright_white().underline()
            );
            println!("   {} {}", "📝".cyan(), description.italic());
            println!(
                    "   {} {}",
                    "🚨 Priority:".yellow(),
                    match priority {
                        Priority::High => format!("{:?}", priority).red().bold(),
                        Priority::Medium => format!("{:?}", priority).yellow().bold(),
                        Priority::Low => format!("{:?}", priority).green().bold(),
                    }
                );
            let mut todos= load_todos();
            let next_id = get_next_id(&todos);
                let new_todo = Todo {
                id: next_id,
                title,
                description,
                completed: false,
                created_at: SystemTime::now(),
                complete_at: SystemTime::now(),
                priority,
            };
            todos.insert(next_id, new_todo);
            save_todos(&todos);
            
            println!("   {} Todo added with ID: {}\n", 
                "✅".green(), 
                next_id.to_string().bright_cyan().bold()
            );
        }
        Commands::MarkCompleted { id } => {
            let mut todos = load_todos();
            if let Some(todo) = todos.get_mut(&id) {
                todo.completed = true;
                println!("{} Todo with ID {} marked as completed.", "✅".green(), id.to_string().bright_cyan().bold());
                save_todos(&todos);
            } else {
                println!("{}  {}", 
                    "❌ Error:".bright_red().bold(),
                    format!("Todo with ID {} not found!", id).red()
                );
            }
        }
        Commands::Remove { id } => {
            let mut todos = load_todos();
            match todos.remove(&id) {
                Some(removed_todo) => {
                    println!("{}  {}", 
                    "🗑️  Successfully removed:".bright_red().bold(),
                    removed_todo.title.strikethrough().bright_black()
                    );
                    save_todos(&todos);
                    println!("   {} ID: {}", 
                        "🆔".blue(), 
                        id.to_string().bright_cyan()
                    );
                }
                None => {
                    println!("{}  {}", 
                        "❌ Error:".bright_red().bold(),
                        format!("Todo with ID {} not found!", id).red()
                    );
                }
            }
        }
        Commands::List => {
            let todos = &load_todos();
            if todos.is_empty() {
                println!("{}  {}", 
                    "📭".bright_blue(),
                    "No todos found! Add some tasks to get started.".italic().bright_black()
                );
                return;
            }

            println!();
            println!("{}", "📋 ═══════════════════════════════════════".bright_blue().bold());
            println!("{}           {}", 
                "📋".bright_blue(),
                "MY AWESOME TODO LIST".bright_white().bold().underline()
        );
            println!("{}", "═══════════════════════════════════════".bright_blue().bold());
            println!();

            for (_id, todo) in todos {
                let status_icon = if todo.completed { "✅" } else { "⭕" };
                let priority_icon = match todo.priority {
                    Priority::High => "🔥",
                    Priority::Medium => "⚡",
                    Priority::Low => "🌱",
                };
                
                let title_display = if todo.completed {
                    todo.title.strikethrough().bright_black()
                } else {
                    todo.title.bright_white().bold()
                };

                println!("{}  {} {} {}", 
                    format!("[{}]", todo.id).bright_cyan().bold(),
                    status_icon,
                    title_display,
                    priority_icon
                );
                
                let description_display = if todo.completed {
                    todo.description.strikethrough().bright_black().italic()
                } else {
                    todo.description.bright_white().italic()
                };
                
                println!("     {} {}", "📝".dimmed(), description_display);
                
                let priority_color = match todo.priority {
                    Priority::High => "red",
                    Priority::Medium => "yellow",
                    Priority::Low => "green",
                };
                
                println!("     {} {} • {} {}", 
                    "🚨".dimmed(),
                    format!("{:?}", todo.priority).color(priority_color).bold(),
                    "📅".dimmed(),
                    "Created just now".bright_black().italic()
                );
                println!();
            }
            
            let total = todos.len();
            let completed = todos.values().filter(|t| t.completed).count();
            let pending = total - completed;
            
            println!("{}", "═══════════════════════════════════════".bright_blue().bold());
            println!("{}  {} {} • {} {} • {} {}", 
                "📊".bright_blue(),
                "Total:".bright_white().bold(), total.to_string().bright_cyan(),
                "Completed:".green().bold(), completed.to_string().bright_green(),
                "Pending:".yellow().bold(), pending.to_string().bright_yellow()
            );
            println!("{}", "═══════════════════════════════════════".bright_blue().bold());
        }
    }
}               
