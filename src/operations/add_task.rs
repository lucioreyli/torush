use crate::task::Task;

use fs::File;
use std::fs;
use std::io::Write;

use std::path::Path;

use chrono::prelude::*;
use ron;

pub fn add_task(args: Vec<String>) {
    if args.len() > 3 {
        let unknown_operation = args.last().unwrap();
        panic!("Unknown action: {:?} | try torush help", unknown_operation);
    }

    let task: Task = Task {
        name: args[2].to_string(),
        completed: false,
    };

    let root_folder = std::env::var("HOME").unwrap();
    let folder_path = Path::new(&root_folder).join(".local/share/torush");
    let file_path = folder_path.join("index");

    let mut task_to_save = ron::to_string(&task).unwrap() as String;
    task_to_save.push_str("\n");

    fs::create_dir_all(&folder_path).expect("Could not create a dir");

    let does_file_exists: bool = file_path.exists();

    let mut file: File = {
        if !does_file_exists {
            match File::create(&file_path) {
                Err(why) => panic!("couldn't create {}: {}", file_path.display(), why),
                Ok(file) => file,
            }
        } else {
            File::options().append(true).open(file_path).unwrap()
        }
    };

    file.write(task_to_save.as_bytes())
        .expect("couldn't write file");

    let created_at: DateTime<Utc> = Utc::now();
    println!(
        "Added task: {} | at {}",
        task.name,
        created_at.format("%Y-%m-%d %H:%M:%S")
    );
}
