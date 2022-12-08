use std::{
    fs::File,
    io::{BufReader, Read},
};

const IS_PART_1: bool = false;
const WINDOW_SIZE: usize = if IS_PART_1 { 4 } else { 14 };

fn shift_into<T, const N: usize>(arr: &mut [T; N], item: T) {
    //Shift everything over
    arr.rotate_left(1);
    //Replace the last item (which was the first before rotating)
    arr[N - 1] = item;
}

fn is_same<T>(a: *const T, b: *const T) -> bool {
    std::ptr::eq(a, b)
}

fn has_only_unique<T: PartialEq, const N: usize>(arr: &[T; N]) -> bool {
    //TODO: O(n^2) :(

    for elem in arr {
        for other in arr {
            if !is_same(elem, other) & (other == elem) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut file = BufReader::new(File::open("task.txt").expect("Can open input file"));

    let input = {
        let mut buf = "".into();
        file.read_to_string(&mut buf)
            .expect("Can read input from file");

        buf.trim().to_owned()
    };

    println!("Parsing input '{input}'");

    let mut last_chars = ['\0'; WINDOW_SIZE];

    for (index, c) in input.trim().chars().enumerate() {
        shift_into(&mut last_chars, c);

        if (index >= WINDOW_SIZE) && has_only_unique(&last_chars) {
            println!("Found marker after char {}", index + 1);
            return;
        }
    }

    println!("No Marker found! Searched {} characters", input.len());
}
