use camino::Utf8PathBuf;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

enum Command {
    Cd,
    Ls,
}

#[derive(PartialEq, Eq, Debug)]
struct File {
    size: i64,
    path: String,
}

#[derive(Debug)]
struct Dir {
    files: Vec<File>,
    path: String,
}

impl Hash for Dir {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl PartialEq for Dir {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Dir {}

// impl Hash for File<'_> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.path.hash(state);
//     }
// }

fn get_command(line: &str) -> Command {
    if line.contains("ls") {
        Command::Ls
    } else {
        Command::Cd
    }
}

trait Directories {
    // fn get_dir(&mut self, path: &str) -> Option<&mut &Dir>;

    fn has_dir(&self, path: &str) -> bool;
}

impl Directories for HashSet<Dir> {
    // fn get_dir(&mut self, path: &str) -> Option<&mut &Dir> {
    //     self.iter().find(|&dir| dir.path == path).as_mut()
    // }

    fn has_dir(&self, path: &str) -> bool {
        self.iter().any(|dir| dir.path == path)
    }
}

pub fn run() {
    let lines = include_str!("input_files/day7.txt").lines();
    let mut current_path = Utf8PathBuf::new();
    let mut directories: HashSet<Dir> = HashSet::new();

    for line in lines {
        let controlling_character = line.chars().next().unwrap();
        match controlling_character {
            '$' => match get_command(line) {
                Command::Ls => {}
                Command::Cd => {
                    let path = line.split_whitespace().last().unwrap();
                    // Path mutations
                    match path {
                        ".." => {
                            current_path.pop();
                            continue;
                        }
                        _ => {
                            current_path.push(path);
                        }
                    };

                    if !directories.has_dir(current_path.as_str()) {
                        let dir = Dir {
                            path: current_path.clone().to_string(),
                            files: vec![],
                        };
                        directories.insert(dir);
                    };

                    println!("Current path after `{line}`  =>  {current_path}");
                }
            },
            '1'..='9' => {
                let mut dir = directories
                    .iter()
                    .find(|&dir| dir.path == current_path.as_str())
                    .as_mut()
                    .unwrap();
                dir.files.push(File {
                    size: 10,
                    path: "".to_string(),
                });
                // let mut t = dir.as_mut().unwrap();
                // dir.files.push(File { size: 10, path: "" });
            }
            'd' => {
                println!("Found directory `{line}`")
            }
            _ => panic!("Wrong Command"),
        };
    }

    dbg!(directories);
}
