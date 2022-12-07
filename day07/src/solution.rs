use std::collections::HashMap;

#[derive(Debug, Clone)]
enum FileType {
    Dir,
    File(u32),
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    variant: FileType,
}

#[derive(Debug, Clone)]
pub enum Input {
    Cd(String),
    Ls,
    DirPrint(String),
    FilePrint(u32, String),
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Input> + 'a {
    input.lines().map(|l| {
        if let Some(command) = l.strip_prefix("$ ") {
            if let Some(dir) = command.strip_prefix("cd ") {
                Input::Cd(dir.to_string())
            } else {
                Input::Ls
            }
        } else {
            if let Some(dir) = l.strip_prefix("dir ") {
                Input::DirPrint(dir.to_string())
            } else {
                let (size, name) = l.split_once(" ").unwrap();
                Input::FilePrint(size.parse().unwrap(), name.to_string())
            }
        }
    })
}

fn handle_cd<'a>(
    arg: &'a str,
    dir_map: &mut HashMap<String, Vec<File>>,
    parent_map: &mut HashMap<String, Option<String>>,
    curr_dir: &'a str,
) -> String {
    let s = match arg {
        ".." => {
            if let Some(dir) = parent_map
                .get(curr_dir)
                .expect("Failed to find current dir in map!")
                .clone()
            {
                dir
            } else {
                String::from("/")
            }
        }
        "/" => "/".to_string(),
        dir => {
            if parent_map.contains_key(dir) == false {
                parent_map.insert(
                    dir.to_string(),
                    if curr_dir == "/" {
                        None
                    } else {
                        Some(curr_dir.to_string())
                    },
                );
            }

            let mut files = if let Some(files) = dir_map.remove(curr_dir) {
                files
            } else {
                vec![]
            };

            files.push(File {
                name: dir.to_string(),
                variant: FileType::Dir,
            });
            dir_map.insert(curr_dir.to_string(), files);

            dir.to_string()
        }
    };

    s
}

fn find_size_of_dir<'a>(
    name: &'a str,
    dir_map: &HashMap<String, Vec<File>>,
    dir_size_map: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(files) = dir_map.get(name) {
        files
            .iter()
            .map(|f| match f.variant {
                FileType::Dir => {
                    if let Some(size) = dir_size_map.get(&f.name) {
                        size.clone()
                    } else {
                        let size = find_size_of_dir(&f.name, dir_map, dir_size_map);
                        dir_size_map.insert(f.name.clone(), size);
                        size
                    }
                }
                FileType::File(size) => size,
            })
            .sum()
    } else {
        0
    }
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = Input>) -> String {
    let mut parent_map: HashMap<String, Option<String>> = HashMap::new();
    parent_map.insert("/".to_string(), None);
    let mut dir_map: HashMap<String, Vec<File>> = HashMap::new();
    let mut curr_dir = String::from("/");
    input.for_each(|i| match i {
        Input::Cd(arg) => curr_dir = handle_cd(&arg, &mut dir_map, &mut parent_map, &curr_dir),
        Input::Ls => {
            // Don't do anything?
        }
        Input::DirPrint(dir) => {
            if parent_map.contains_key(&dir) == false {
                parent_map.insert(
                    dir.clone(),
                    if curr_dir == "/" {
                        None
                    } else {
                        Some(curr_dir.clone())
                    },
                );
            }

            let mut files = if let Some(files) = dir_map.remove(&curr_dir) {
                files
            } else {
                vec![]
            };

            files.push(File {
                name: dir.clone(),
                variant: FileType::Dir,
            });
            dir_map.insert(curr_dir.clone(), files);
        }
        Input::FilePrint(size, name) => {
            let mut files = if let Some(files) = dir_map.remove(&curr_dir) {
                files
            } else {
                vec![]
            };

            files.push(File {
                name: name.clone(),
                variant: FileType::File(size),
            });
            dir_map.insert(curr_dir.clone(), files);
        }
    });

    let mut dir_size_map: HashMap<String, u32> = HashMap::new();
    for dir in dir_map.keys() {
        let size = find_size_of_dir(dir.as_str(), &dir_map, &mut dir_size_map);
        dir_size_map.insert(dir.clone(), size);
    }

    let solution: u32 = dir_size_map
        .into_iter()
        .map(|(_, v)| v)
        .filter(|&v| v <= (100000 as u32))
        .sum();

    format!("{solution}")
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = Input>) -> String {
    todo!("Part two is not yet implemented");
}
