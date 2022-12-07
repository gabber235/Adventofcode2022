use std::collections::HashMap;
use std::fs;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

// --- Day 7: No Space Left On Device ---
// You can hear birds chirping and raindrops hitting leaves as the expedition proceeds. Occasionally, you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?
// The device the Elves gave you has problems with more than just its communication system. You try to run a system update:
// $ system-update --please --pretty-please-with-sugar-on-top
// Error: No space left on device
// Perhaps you can delete some files to make space for the update?
//
// You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input). For example:
//
// $ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k
// The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.
//
// Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:
//
// cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
// cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
// cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
// cd / switches the current directory to the outermost directory, /.
// ls means list. It prints out all of the files and directories immediately contained by the current directory:
// 123 abc means that the current directory contains a file named abc with size 123.
// dir xyz means that the current directory contains a directory named xyz.
// Given the commands and output in the example above, you can determine that the filesystem looks visually like this:
//
// - / (dir)
//   - a (dir)
//     - e (dir)
//       - i (file, size=584)
//     - f (file, size=29116)
//     - g (file, size=2557)
//     - h.lst (file, size=62596)
//   - b.txt (file, size=14848514)
//   - c.dat (file, size=8504156)
//   - d (dir)
//     - j (file, size=4060174)
//     - d.log (file, size=8033020)
//     - d.ext (file, size=5626152)
//     - k (file, size=7214296)
// Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a). These directories also contain files of various sizes.
// Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)
// The total sizes of the directories above can be found as follows:
//
// The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
// The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
// Directory d has total size 24933642.
// As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.
// To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)
//
// Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?

#[derive(Debug)]
struct Directory {
    name: String,
    directories: HashMap<String, Directory>,
    files: Vec<File>,
}

impl Directory {
    fn file_size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
    }

    fn total_size(&self) -> u64 {
        self.file_size()
            + self
                .directories
                .values()
                .map(|d| d.total_size())
                .sum::<u64>()
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

/// Finds the new path for the current directory
/// based on the current path and the command
/// If the name is ".." then it moves up one level
/// If the name is "/" then it moves to the root
/// If the name is a directory then it moves into that directory
fn new_path(path: &str, name: &str) -> String {
    if name == ".." {
        let mut path = path.split('/').collect::<Vec<&str>>();
        path.pop();
        path.join("/")
    } else if name == "/" {
        "/".to_string()
    } else {
        format!("{}/{}", path, name)
    }
}

fn find_directory<'a>(path: &str, root: &'a mut Directory) -> &'a mut Directory {
    let mut current = root;
    for name in path.split('/').skip(1) {
        if name.is_empty() {
            continue;
        }
        // Get the directory from the current directory or insert a new one
        current = current
            .directories
            .entry(name.to_string())
            .or_insert(Directory {
                name: name.to_string(),
                directories: HashMap::new(),
                files: Vec::new(),
            });
    }
    current
}

#[test]
fn when_given_a_root_directory_and_the_base_path_expect_to_find_the_root_directory() {
    let mut root = Directory {
        name: "/".to_string(),
        directories: HashMap::new(),
        files: Vec::new(),
    };
    let path = "/";
    println!("Path Split: {:?}", path.split('/').collect::<Vec<&str>>());
    let directory = find_directory(path, &mut root);
    assert_eq!(directory.name, "/");
}

fn parse_commands(text: String) -> Directory {
    let mut root = Directory {
        name: String::from("/"),
        directories: HashMap::new(),
        files: Vec::new(),
    };
    let mut current_dir = &mut root;
    let mut path = "/".to_string();

    for line in text.lines() {
        if line.starts_with("$ cd") {
            path = new_path(&path, &line[5..]);
            current_dir = find_directory(&path, &mut root);
        }

        // If the line starts with a number then it is a file
        if line.chars().next().unwrap().is_numeric() {
            let mut parts = line.split(' ');
            let size = parts.next().unwrap().parse::<u64>().unwrap();
            let name = parts.next().unwrap().to_string();
            current_dir.files.push(File { name, size });
        }
    }

    root
}

fn find_directories_with_size(directory: &Directory, size: u64) -> Vec<&Directory> {
    let mut directories = Vec::new();
    if directory.total_size() <= size {
        directories.push(directory);
    }
    for (_, dir) in &directory.directories {
        directories.append(&mut find_directories_with_size(dir, size));
    }
    directories
}

fn process_part1(text: String) -> i32 {
    let root = parse_commands(text);

    find_directories_with_size(&root, 100000)
        .iter()
        .map(|d| d.total_size() as i32)
        .sum()
}

//--- Part Two ---
// Now, you're ready to choose a directory to delete.
// The total disk space available to the filesystem is 70000000. To run the update, you need unused space of at least 30000000. You need to find a directory you can delete that will free up enough space to run the update.
// In the example above, the total size of the outermost directory (and thus the total amount of used space) is 48381165; this means that the size of the unused space must currently be 21618835, which isn't quite the 30000000 required by the update. Therefore, the update still requires a directory with total size of at least 8381165 to be deleted before it can run.
// To achieve this, you have the following options:
//
// Delete directory e, which would increase unused space by 584.
// Delete directory a, which would increase unused space by 94853.
// Delete directory d, which would increase unused space by 24933642.
// Delete directory /, which would increase unused space by 48381165.
// Directories e and a are both too small; deleting them would not free up enough space. However, directories d and / are both big enough! Between these, choose the smallest: d, increasing unused space by 24933642.
//
// Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory?

fn all_directories(directory: &Directory) -> Vec<&Directory> {
    let mut directories = Vec::new();
    directories.push(directory);
    for (_, dir) in &directory.directories {
        directories.append(&mut all_directories(dir));
    }
    directories
}

fn process_part2(text: String) -> i32 {
    let root = parse_commands(text);

    let max_size = 70000000;
    let min_free = 30000000;
    let free = max_size - root.total_size();
    let needed = min_free - free;

    let mut directories = all_directories(&root);
    directories.sort_by(|a, b| a.total_size().cmp(&b.total_size()));

    directories
        .iter()
        .filter(|d| d.total_size() >= needed)
        .map(|d| d.total_size() as i32)
        .next()
        .unwrap()
}
