use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum File {
    Dir(HashMap<String, File>),
    File(u32),
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

#[inline(always)]
pub fn get_parent_map<'a>(
    path_stack: &Vec<String>,
    root_file: &'a mut File,
) -> &'a mut HashMap<String, File> {
    let mut curr_file = root_file;
    for path in path_stack.iter() {
        match curr_file {
            File::Dir(map) => curr_file = map.get_mut(path.as_str()).unwrap(),
            File::File(_) => panic!("Cannot find parent file in FILE"),
        }
    }

    match curr_file {
        File::Dir(map) => map,
        File::File(_) => panic!("Parent file is not a directory?"),
    }
}

fn find_dir_sizes(
    file: &mut File,
    dir_size_map: &mut HashMap<String, u32>,
    curr_file_path: String,
) -> u32 {
    match file {
        File::Dir(map) => {
            let size = map
                .into_iter()
                .map(|(dir_name, dir)| {
                    find_dir_sizes(dir, dir_size_map, format!("{curr_file_path}/{dir_name}"))
                })
                .sum();
            dir_size_map.insert(curr_file_path, size);
            size
        }
        File::File(size) => size.clone(),
    }
}

fn find_root_sizes(root_file: &mut File) -> HashMap<String, u32> {
    let mut dir_size_map: HashMap<String, u32> = HashMap::new();
    match root_file {
        File::Dir(root_map) => {
            for (_, file) in root_map {
                find_dir_sizes(file, &mut dir_size_map, String::new());
            }
        }
        File::File(_) => panic!("Root is a file not a dir!??!?!"),
    }
    dir_size_map
}

fn create_root_file<'a>(input: impl Iterator<Item = Input>) -> File {
    let mut root_file = File::Dir(HashMap::new());

    let mut path_stack: Vec<String> = vec![];
    let mut curr_path = "/".to_string();
    let mut curr_map: HashMap<String, File> = HashMap::new();

    for i in input {
        match i {
            Input::Cd(arg) => {
                let parent_map = get_parent_map(&path_stack, &mut root_file);
                parent_map.insert(curr_path.to_string(), File::Dir(curr_map));

                match arg.as_str() {
                    ".." => {
                        let prev_path = path_stack.pop().unwrap();
                        curr_path = prev_path;
                    }
                    "/" => {
                        path_stack = vec![];
                        curr_path = "/".to_string();
                    }
                    _ => {
                        path_stack.push(curr_path.clone());
                        curr_path = arg;
                    }
                }

                let parent_map = get_parent_map(&path_stack, &mut root_file);
                curr_map = match parent_map.remove(&curr_path).unwrap() {
                    File::Dir(map) => map,
                    File::File(_) => panic!("Expected dir, got file"),
                }
            }
            Input::Ls => { /* Don't do anything? */ }
            Input::DirPrint(dir_name) => {
                curr_map.insert(dir_name, File::Dir(HashMap::new()));
            }
            Input::FilePrint(file_size, file_name) => {
                curr_map.insert(file_name, File::File(file_size));
            }
        };
    }
    let parent_map = get_parent_map(&path_stack, &mut root_file);
    parent_map.insert(curr_path.to_string(), File::Dir(curr_map));

    root_file
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = Input>) -> String {
    let mut root_file = create_root_file(input);
    let dir_size_map = find_root_sizes(&mut root_file);

    let solution: u32 = dir_size_map
        .into_iter()
        .map(|(_, v)| v)
        .filter(|&v| v <= (100000 as u32))
        .sum();

    format!("{solution}")
}

const TOTAL_SPACE: u32 = 70000000;
const REQUIRED_DISK_SPACE: u32 = 30000000;

pub fn solve_part_two<'a>(input: impl Iterator<Item = Input>) -> String {
    let mut root_file = create_root_file(input);

    let dir_size_map = find_root_sizes(&mut root_file);

    // println!("DIR SIZE MAP {dir_size_map:#?}");

    let total_used_space = dir_size_map.get("").unwrap().clone();
    // println!("total_used_space {total_used_space}");
    // println!("total_space      {TOTAL_SPACE}");
    let free_space = TOTAL_SPACE - total_used_space;
    // println!("free_space       {free_space}");
    let space_to_free = REQUIRED_DISK_SPACE - free_space;
    // println!("space_to_free    {space_to_free}");

    let mut closest_val = u32::MAX;
    for (_, size) in dir_size_map {
        if size >= space_to_free && size < closest_val {
            closest_val = size;
        }
    }

    format!("{closest_val}")
}
