use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

const FILESYSTEM_SIZE: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;

enum DirectoryEntry {
    File {
        size: usize,
    },
    Directory {
        entries: HashMap<String, DirectoryEntry>,
    },
}

impl DirectoryEntry {
    fn new_dir() -> Self {
        use DirectoryEntry::Directory;
        Directory {
            entries: HashMap::default(),
        }
    }

    fn insert_entries<T: IntoIterator<Item = (String, DirectoryEntry)>>(&mut self, to_insert: T) {
        use DirectoryEntry::Directory;
        let Directory{entries} = self else { panic!("Tried to add entries to file!") };

        entries.extend(to_insert);
    }

    fn get_entry(&mut self, name: &str) -> &mut DirectoryEntry {
        use DirectoryEntry::*;

        let Directory{entries}= self else { panic!("Tried to get entry from file!") };

        entries.get_mut(name).expect("Can get entry from directory")
    }

    fn get_size(&self) -> usize {
        use DirectoryEntry::*;

        match self {
            File { size } => *size,
            Directory { entries } => entries.iter().map(|(_, entry)| entry.get_size()).sum(),
        }
    }

    // PART 1
    fn sum_size_below(&self, limit: usize) -> usize {
        use DirectoryEntry::*;
        match self {
            File { size } if *size <= limit => *size,
            Directory { entries } => {
                let my_size = self.get_size();
                let my_size = if my_size < limit { my_size } else { 0 };

                let subdirectory_size = entries
                    .iter()
                    .map(|(_, entry)| {
                        if let Directory { .. } = entry {
                            entry.sum_size_below(limit)
                        } else {
                            0
                        }
                    })
                    .sum::<usize>();

                my_size + subdirectory_size
            }
            _ => 0,
        }
    }

    // PART 2
    fn find_smallest_above(&self, minimum: usize) -> usize {
        self._find_smallest_above(minimum, usize::MAX)
    }

    fn _find_smallest_above(&self, minimum: usize, current_best: usize) -> usize {
        use DirectoryEntry::*;

        let my_guess = match self {
            File { size } => {
                if *size >= minimum {
                    *size
                } else {
                    current_best
                }
            }
            Directory { entries } => {
                let my_size = self.get_size();

                //bail out early if we didn't make the cut
                if my_size < minimum {
                    return current_best;
                }

                let smallest_child = entries
                    .iter()
                    .map(|(_, entry)| {
                        if let Directory { .. } = entry {
                            entry._find_smallest_above(minimum, current_best)
                        } else {
                            0
                        }
                    })
                    .filter(|&guess| guess >= minimum)
                    .min()
                    .unwrap_or(current_best);

                usize::min(my_size, smallest_child)
            }
        };

        usize::min(my_guess, current_best)
    }
}

fn parse_input(lines: &mut VecDeque<String>) -> DirectoryEntry {
    let mut current_directory = DirectoryEntry::new_dir();

    while let Some(line) = lines.pop_front() {
        let Some(cmd) = line.strip_prefix("$ ") else { panic!("Expected command!") };

        if cmd == "ls" {
            let entries = parse_ls(lines);
            current_directory.insert_entries(entries);
        } else if let Some(target) = cmd.strip_prefix("cd ") {
            //Go back up to the parent
            if target == ".." {
                return current_directory;
            }

            // Edit an existing entry
            *current_directory.get_entry(target) = parse_input(lines);
        } else {
            unimplemented!("Unknown command!");
        }
    }

    //All lines parsed :)
    current_directory
}

fn parse_ls(lines: &mut VecDeque<String>) -> Vec<(String, DirectoryEntry)> {
    use DirectoryEntry::*;

    let mut entries = vec![];

    while let Some(line) = lines.pop_front() {
        if line.starts_with('$') {
            //Command, put it back so our caller can keep parsing
            lines.push_front(line);
            break;
        }

        if let Some(name) = line.strip_prefix("dir ") {
            //Directory
            entries.push((name.into(), DirectoryEntry::new_dir()));
        } else {
            //File
            let [size, name] = line.splitn(2, ' ').collect::<Vec<_>>()[..2] else { panic!("Wrong format for file entry!") };
            let size = size.parse().expect("Can parse file size");

            entries.push((name.into(), File { size }));
        }
    }

    entries
}

fn main() {
    let file = BufReader::new(File::open("test.txt").expect("Can open input file"));

    let mut lines = file.lines().flatten().collect::<VecDeque<_>>();

    assert!(lines.pop_front().unwrap_or("".into()) == "$ cd /");

    let top_dir = parse_input(&mut lines);

    let part1_solution = top_dir.sum_size_below(100_000);
    println!("{}", part1_solution);

    let total_size = top_dir.get_size();
    let free_space = FILESYSTEM_SIZE - total_size;
    let to_free = UPDATE_SIZE - free_space;

    println!("Need {} more bytes!", to_free);

    let part2_solution = top_dir.find_smallest_above(to_free);
    println!("Smallest: {}", part2_solution);
}
