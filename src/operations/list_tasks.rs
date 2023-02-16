use crate::task::Task;
use std::io::BufRead;
use std::io::BufReader;

use fs::File;
use std::fs;

use std::path::Path;

pub fn list_tasks() {
    let root_folder = std::env::var("HOME").unwrap();
    let file_path = Path::new(&root_folder).join(".local/share/torush/index");

    if !file_path.exists() {
        return println!("No task registered! Please, try:\ntorush add 'buy energy tomorrow'");
    };

    let tasks_file = File::open(file_path).expect("Unable to find tasks");
    let buffer_file = BufReader::new(tasks_file);

    for line in buffer_file.lines() {
        let task_line = line.unwrap();
        let task = ron::from_str::<Task>(&task_line).unwrap();
        println!(
            "{} - [ {} ] - {} - Created at {}",
            task.id,
            if task.completed { '✅' } else { '🌀' },
            task.name,
            task.created_at.format("%Y-%m-%d %H:%M:%S"),
        );
    }
}
