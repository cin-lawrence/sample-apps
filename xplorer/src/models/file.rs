use std::fs;

use tracing;

#[derive(Debug)]
pub struct File {
    pub is_directory: bool,
    pub name: String,
}

pub struct Files {
    path_stack: Vec<String>,
    pub path_names: Vec<File>,
    pub err: Option<String>,
}

impl Files {
    pub fn new() -> Self {
        let mut files = Self {
            path_stack: vec!["/home/".to_string()],
            path_names: vec![],
            err: None,
        };

        files.reload_path_list();
        files
    }

    fn reload_path_list(&mut self) {
        let cur_path = self.path_stack.join("/");
        tracing::info!("Reloading path list for {:?}", cur_path);

        let paths = match fs::read_dir(&cur_path) {
            Ok(e) => e,
            Err(err) => match fs::read_to_string(cur_path) {
                Ok(_) => {
                    tracing::info!("Opened file");
                    return;
                }
                Err(_) => {
                    let err = format!("An error occurred: {:?}", err);
                    self.err = Some(err);
                    self.path_stack.pop();
                    return;
                }
            },
        };

        let collected = paths.collect::<Vec<_>>();
        tracing::info!("Path list reloaded {:#?}", collected);

        self.clear_err();
        self.path_names.clear();

        for path in collected {
            let file = path.unwrap();
            self.path_names.push(File {
                name: file.file_name().to_str().unwrap().to_string(),
                is_directory: file.file_type().unwrap().is_dir(),
            });
        }
        tracing::info!("path names are {:#?}", self.path_names);
    }

    pub fn go_up(&mut self) {
        if self.path_stack.len() > 1 {
            self.path_stack.pop();
        }
        self.reload_path_list();
    }

    pub fn enter_dir(&mut self, dir_id: usize) {
        let path = &self.path_names[dir_id];
        self.path_stack.push(path.name.to_string());
        self.reload_path_list();
    }

    pub fn current(&self) -> String {
        self.path_stack.join("/")
    }

    pub fn clear_err(&mut self) {
        self.err = None;
    }
}
