use std::collections::{HashMap, HashSet};

pub fn solve_non_parallel(board: &str) -> Vec<String> {
    let mut sudoku: HashMap<String, HashSet<char>> = initialize_sudoku(board);
    let units = get_units();
    let peers = get_peers();

    if not_consistent(&mut sudoku, &units, &peers) {
        return vec![];
    }

    non_parallel_search(sudoku, &units, &peers)
}

fn initialize_sudoku(board: &str) -> HashMap<String, HashSet<char>> {
    let mut sudoku = HashMap::new();
    let digits: HashSet<char> = "123456789".chars().collect();
    let squares = get_squares();

    for (i, ch) in board.chars().enumerate() {
        let key = &squares[i];
        if ch == '.' {
            sudoku.insert(key.clone(), digits.clone());
        } else {
            let mut set = HashSet::new();
            set.insert(ch);
            sudoku.insert(key.clone(), set);
        }
    }
    sudoku
}

fn get_squares() -> Vec<String> {
    let rows = "ABCDEFGHI".chars().collect::<Vec<_>>();
    let cols = "123456789".chars().collect::<Vec<_>>();
    cross(&rows, &cols)
}

fn cross(a: &[char], b: &[char]) -> Vec<String> {
    let mut result = Vec::new();
    for &i in a {
        for &j in b {
            result.push(format!("{}{}", i, j));
        }
    }
    result
}

fn get_units() -> HashMap<String, Vec<Vec<String>>> {
    let rows = "ABCDEFGHI".chars().collect::<Vec<_>>();
    let cols = "123456789".chars().collect::<Vec<_>>();
    let squares = get_squares();

    let mut units = HashMap::new();
    for s in &squares {
        let (row, col) = s.split_at(1);
        let row_units = cross(&[row.chars().next().unwrap()], &cols);
        let col_units = cross(&rows, &[col.chars().next().unwrap()]);

        let box_units = cross(
            match row {
                "A" | "B" | "C" => &['A', 'B', 'C'],
                "D" | "E" | "F" => &['D', 'E', 'F'],
                _ => &['G', 'H', 'I'],
            },
            match col {
                "1" | "2" | "3" => &['1', '2', '3'],
                "4" | "5" | "6" => &['4', '5', '6'],
                _ => &['7', '8', '9'],
            },
        );

        units.insert(s.clone(), vec![row_units, col_units, box_units]);
    }
    units
}

fn get_peers() -> HashMap<String, HashSet<String>> {
    let squares = get_squares();
    let units = get_units();

    let mut peers = HashMap::new();
    for s in &squares {
        let mut peer_set = HashSet::new();
        for unit in &units[s] {
            for square in unit {
                if square != s {
                    peer_set.insert(square.clone());
                }
            }
        }
        peers.insert(s.clone(), peer_set);
    }
    peers
}

fn not_consistent(
    sudoku: &mut HashMap<String, HashSet<char>>,
    _units: &HashMap<String, Vec<Vec<String>>>,
    peers: &HashMap<String, HashSet<String>>,
) -> bool {
    let mut changes = true;
    while changes {
        changes = false;
        for (s, value_set) in sudoku.clone().iter() {
            if value_set.len() == 1 {
                let value = *value_set.iter().next().unwrap();
                for peer in &peers[s] {
                    if sudoku[peer].contains(&value) {
                        sudoku.get_mut(peer).unwrap().remove(&value);
                        if sudoku[peer].is_empty() {
                            return true;
                        }
                        changes = true;
                    }
                }
            }
        }
    }
    false
}

fn non_parallel_search(
    sudoku: HashMap<String, HashSet<char>>,
    units: &HashMap<String, Vec<Vec<String>>>,
    peers: &HashMap<String, HashSet<String>>,
) -> Vec<String> {
    if sudoku.values().all(|v| v.len() == 1) {
        return vec![sudoku_to_string(&sudoku)];
    }

    let (min_square, min_set) = sudoku
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .min_by_key(|(_, v)| v.len())
        .unwrap();

    let mut solutions = Vec::new();
    for &value in min_set {
        let mut sudoku_copy = sudoku.clone();
        sudoku_copy.get_mut(min_square).unwrap().clear();
        sudoku_copy.get_mut(min_square).unwrap().insert(value);

        if not_consistent(&mut sudoku_copy, units, peers) {
            continue;
        } else {
            solutions.extend(non_parallel_search(sudoku_copy, units, peers));
        }
    }
    solutions
}

fn sudoku_to_string(sudoku: &HashMap<String, HashSet<char>>) -> String {
    let squares = get_squares();
    squares.iter().map(|s| {
        let value = sudoku[s].iter().next().unwrap();
        *value
    }).collect()
}

pub(crate) fn format_board(board: &str) -> String {
    let mut result = String::new();
    let rows = "ABCDEFGHI".chars().collect::<Vec<_>>();
    let cols = "123456789".chars().collect::<Vec<_>>();

    for (i, ch) in board.chars().enumerate() {
        let col = i % 9;
        result.push(if ch == '.' { '.' } else { ch });
        if col == 8 {
            result.push('\n');
        } else {
            result.push(' ');
            if (col + 1) % 3 == 0 {
                result.push('|');
            }
        }
        if (i + 1) % 27 == 0 && i != 80 {
            result.push_str("------+-------+------\n");
        }
    }
    result
}

pub(crate) fn print_board(board: &str) {
    let formatted_board = format_board(board);
    println!("{}", formatted_board);
}

fn main() {

    use std::time::Instant;
    let now = Instant::now();

    let board = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";

    println!("Initial board:");
    print_board(board);

    let solutions = solve_non_parallel(board);

    if solutions.is_empty() {
        println!("No solution found.");
    } else {
        for (i, solution) in solutions.iter().enumerate() {
            println!("Solution {}:\n{}", i + 1, format_board(solution));
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed 1 : {:.2?}", elapsed);
}