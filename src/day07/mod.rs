use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

#[test]
fn day07() {
    let input = include_str!("input.txt");
    // let input = include_str!("test_input.txt");

    let dirs = parse_directories(input);

    let all_sizes: Vec<usize> = dirs
        .keys()
        .map(|p| dir_size(p, &dirs))
        .collect();

    let tot_size_filtered = all_sizes.iter().filter(|&&v| v <= 100_000).sum::<usize>();
    println!("{}", tot_size_filtered);

    let size_to_delete = 30_000_000 - (70_000_000 - dir_size(&PathBuf::from("/"), &dirs));
    let min_size_to_delete = all_sizes.iter().filter(|&&v| v >= size_to_delete).min().unwrap();
    println!("{}", min_size_to_delete);
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Directory {
    files: HashSet<File>,
    dirs: HashSet<PathBuf>,
}

impl Directory {
    fn new() -> Self {
        Directory {
            files: HashSet::new(),
            dirs: HashSet::new(),
        }
    }
}

fn parse_directories(input: &str) -> HashMap<PathBuf, Directory> {
    let mut cwd_path = PathBuf::from("/");

    let mut dirs: HashMap<PathBuf, Directory> = HashMap::new();
    dirs.insert(cwd_path.clone(), Directory::new());

    for l in input.lines() {
        if let Some(dir) = l.strip_prefix("$ cd ") {
            match dir {
                "/" => { cwd_path = PathBuf::from("/"); },
                ".." => { cwd_path.pop(); },
                dir => { cwd_path.push(dir); },
            }
        } else if l == "$ ls" {
            // do noting
        } else if let Some(dir) = l.strip_prefix("dir ") {
            let mut new_path = cwd_path.clone();
            new_path.push(dir);

            let cwd = dirs.get_mut(&cwd_path).unwrap();
            cwd.dirs.insert(new_path.clone());

            dirs.entry(new_path.clone()).or_insert(Directory::new());
        } else {
            let mut parts = l.split(' ');
            let size = parts.next().unwrap().parse::<usize>().unwrap();
            let name = parts.next().unwrap().to_string();
            let file = File { name, size };

            let cwd = dirs.get_mut(&cwd_path).unwrap();
            cwd.files.insert(file);
        }
    }

    dirs
}

fn dir_size(path: &PathBuf, dirs: &HashMap<PathBuf, Directory>) -> usize {
    let dir = match dirs.get(path) {
        Some(dir) => dir,
        None => panic!("can't find directory"),
    };

    let size_files = dir.files.iter().map(|f| f.size).sum::<usize>();
    let size_dirs = dir.dirs.iter().map(|p| dir_size(p, dirs)).sum::<usize>();

    size_files + size_dirs
}
