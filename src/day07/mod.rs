#[test]
fn day07() {
    let input = include_str!("input.txt");
    // let input = include_str!("test_input.txt");

    let mut dirs = parse_directories(input);

    let tot_size = dirs
        .iter_mut()
        .map(|d| d.size())
        .filter(|&v| v <= 100_000)
        .sum::<usize>();
    println!("{}", tot_size);

    let size_to_delete = 30_000_000 - (70_000_000 - dirs[0].size()); // root happen to be at index 0
    let min_possible_size = dirs
        .iter_mut()
        .map(|d| d.size())
        .filter(|&v| v >= size_to_delete)
        .min()
        .unwrap();
    println!("{}", min_possible_size);
}

fn parse_directories(commands: &str) -> Vec<Directory> {
    let mut all_dirs = Vec::new();

    all_dirs.push(Directory::new("root".to_string()));
    let mut cwd = all_dirs[0].clone();

    for l in commands.lines() {
        if let Some(dir) = l.strip_prefix("$ cd ") {
            match dir {
                "/" => { cwd = all_dirs[0].clone(); }
                ".." => { cwd = cwd.get_parent(); }
                dir => { cwd = cwd.get_subdir(dir); }
            }
        } else if l == "$ ls" {
            // do noting
        } else if let Some(dir) = l.strip_prefix("dir ") {
            let dir = Directory::new(dir.to_string());

            all_dirs.push(dir.clone());
            cwd.add_subdir(dir.clone());
        } else {
            let mut parts = l.split(' ');
            let size = parts.next().unwrap().parse::<usize>().unwrap();
            let name = parts.next().unwrap().to_string();
            let file = File { name, size };

            cwd.add_file(file);
        }
    }

    return all_dirs;
}


use std::rc::{Rc,Weak};
use std::cell::{RefCell,Ref,RefMut};
use std::collections::HashMap;

#[derive(Debug,Clone)]
struct Directory(Rc<RefCell<DirData>>);

#[derive(Debug)]
struct DirectoryWeak(Weak<RefCell<DirData>>);

#[derive(Debug)]
struct DirData {
    name: String,
    size: Option<usize>,
    parent: DirectoryWeak,
    files: HashMap<String,File>,
    subdirs: HashMap<String,Directory>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl Directory {
    fn new(name: String) -> Self {
        let data = DirData {
            name,
            size: None,
            parent: DirectoryWeak::new(),
            files: HashMap::new(),
            subdirs: HashMap::new(),
        };
        Directory(Rc::new(RefCell::new(data)))
    }

    fn get_data(&self) -> Ref<DirData> {
        self.0.borrow()
    }

    fn get_data_mut(&mut self) -> RefMut<DirData> {
        self.0.borrow_mut()
    }

    fn to_weak(&self) -> DirectoryWeak {
        DirectoryWeak(Rc::downgrade(&self.0))
    }

    fn add_subdir(&mut self, mut dir: Directory) {
        let key = dir.get_data().name.to_string();
        dir.get_data_mut().parent = self.to_weak();
        self.get_data_mut().subdirs.entry(key).or_insert(dir);
    }

    fn get_subdir(&self, name: &str) -> Directory {
        self.get_data().subdirs.get(name).unwrap().clone()
    }

    fn get_parent(&self) -> Directory {
        self.get_data().parent.to_strong()
    }

    fn add_file(&mut self, file: File) {
        let key = file.name.to_string();
        self.get_data_mut().files.entry(key).or_insert(file);
    }

    fn size(&mut self) -> usize {
        let mut data = self.get_data_mut();

        match data.size {
            Some(size) => size,
            None => {
                let size_files = data.files.values().map(|f| f.size).sum::<usize>();
                let size_dirs = data.subdirs.values_mut().map(|d| d.size()).sum::<usize>();
                let size = size_files + size_dirs;
                data.size = Some(size);

                size
            }
        }

    }
}

impl DirectoryWeak {
    fn to_strong(&self) -> Directory {
        Directory(self.0.upgrade().unwrap().clone())
    }

    fn new() -> Self {
        DirectoryWeak(Weak::new())
    }
}

