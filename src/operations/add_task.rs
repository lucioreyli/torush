use crate::task::Task;

use fs::File;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

use std::path::Path;

use chrono::prelude::*;
use ron;

pub fn add_task(args: Vec<String>) {
    if args.len() > 3 {
        let unknown_operation = args.last().unwrap();
        panic!("Unknown action: {:?} | try torush help", unknown_operation);
    }

    let root_folder = std::env::var("HOME").unwrap();
    let folder_path = Path::new(&root_folder).join(".local/share/torush");
    let file_path = folder_path.join("index");

    fs::create_dir_all(&folder_path).expect("Could not create a dir");

    let does_file_exists: bool = file_path.exists();

    let mut file: File = {
        if !does_file_exists {
            match File::create(&file_path) {
                Err(why) => panic!("couldn't create {}: {}", file_path.display(), why),
                Ok(file) => file,
            }
        } else {
            File::options()
                .append(true)
                .write(true)
                .read(true)
                .open(&file_path)
                .unwrap()
        }
    };

    let file_to_get_tasks = File::open(file_path).expect("unable to read previous tasks");

    let buffer_file = BufReader::new(&file_to_get_tasks);
    let tasks: Vec<_> = buffer_file.lines().collect();
    let total_tasks = tasks.len();

    let task_id = match total_tasks {
        0 => 1,
        _ => {
            let last_line = tasks.last().unwrap().as_ref().expect("unable to read task");
            let last_task: Task = ron::from_str::<Task>(&last_line).unwrap();
            last_task.id + 1
        }
    };

    let task: Task = Task {
        id: task_id,
        name: args[2].to_string(),
        completed: false,
        created_at: Utc::now(),
    };

    let mut task_to_save = ron::to_string(&task).unwrap() as String;
    task_to_save.push_str("\n");

    file.write(task_to_save.as_bytes())
        .expect("couldn't write file");

    println!(
        "Added task: {} | at {}",
        task.name,
        task.created_at.format("%Y-%m-%d %H:%M:%S")
    );
}
