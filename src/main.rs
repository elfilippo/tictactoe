use std::{
    io,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    println!("welcome to tic tac toe!");

    let mut cells = [' '; 9];

    println!("| | | |\n| | | |\n| | | |");

    let mut active_player = 'X';
    let mut buf = String::new();
    let mut reset = true;
    let mut multiplayer = false;

    'game_loop: loop {
        buf.clear();

        if reset {
            println!("do you wish to play against another player or a bot? (p/b)");
            let Ok(_) = io::stdin().read_line(&mut buf) else {
                println!("couldn't read input. please try again.");
                continue;
            };
            multiplayer = match buf.to_lowercase().trim() {
                "p" | "player" => true,
                "b" | "bot" | "computer" => {
                    active_player = 'X';
                    false
                }
                _ => continue,
            };
            reset = false;
        }

        println!("player {active_player}, which cell do you wish to play in? (1-9)");

        buf.clear();
        let Ok(_) = io::stdin().read_line(&mut buf) else {
            println!("couldn't read input. please try again.");
            continue;
        };

        let Some(input_field) = buf
            .trim()
            .parse::<usize>()
            .ok()
            .and_then(|n| (1..=9).contains(&n).then_some(n - 1))
        else {
            println!("invalid cell. please enter a number from 1 - 9.");
            continue;
        };

        if cells[input_field] != ' ' {
            println!("there's already a piece in that cell.");
            continue;
        }
        cells[input_field] = active_player;

        if !multiplayer && cells.contains(&' ') && !has_won(&cells, 'X') {
            cells[bot_move(&cells)] = 'O';
        }

        println!();
        for ca in cells.chunks(3) {
            for c in ca {
                print!("|{c}");
            }
            println!("|");
        }

        if has_won(&cells, active_player)
            || !cells.contains(&' ')
            || (!multiplayer && has_won(&cells, 'O'))
        {
            if !cells.contains(&' ') {
                println!("it's a tie!");
            } else if has_won(&cells, active_player) {
                println!("congratulations, player {active_player}! you have won!");
            } else {
                println!("oh no! the bot got you on that one. better luck next time!");
            }

            println!("new round? (y/n)");

            let mut piss_off_counter = 0;
            loop {
                buf.clear();
                let Ok(_) = io::stdin().read_line(&mut buf) else {
                    println!("couldn't read input. please try again.");
                    continue;
                };
                match buf.to_lowercase().trim() {
                    "y" | "yes" => {
                        cells.fill(' ');
                        println!("\n| | | |\n| | | |\n| | | |");
                        active_player = if active_player == 'X' { 'O' } else { 'X' };
                        reset = true;
                        continue 'game_loop;
                    }
                    "n" | "no" => {
                        println!("bye");
                        break 'game_loop;
                    }
                    _ => {
                        piss_off_counter += 1;
                        if piss_off_counter % 4 == 3 {
                            println!("yes or no, bitchass?");
                        } else if piss_off_counter >= 12 {
                            println!("fuck right off you twat");
                            break 'game_loop;
                        } else {
                            println!("dpmo. yes or no?");
                        }
                    }
                }
            }
        }

        active_player = if active_player == 'X' && multiplayer {
            'O'
        } else {
            'X'
        };
    }
}

fn has_won(cells: &[char], current_player: char) -> bool {
    'outer: for row in 0..3 {
        for col in 0..3 {
            if cells[col + 3 * row] != current_player {
                continue 'outer;
            }
        }
        return true;
    }

    'outer: for col in 0..3 {
        for row in 0..3 {
            if cells[col + 3 * row] != current_player {
                continue 'outer;
            }
        }
        return true;
    }

    cells[4] == current_player
        && ((cells[0] == current_player && cells[8] == current_player)
            || (cells[2] == current_player && cells[6] == current_player))
}

fn bot_move(cells: &[char]) -> usize {
    let mut possible_moves = Vec::new();

    for row in 0..3 {
        let mut in_row = 0;
        for col in 0..3 {
            let i = col + 3 * row;
            if cells[i] == 'X' {
                in_row += 1;
            } else if cells[i] == 'O' {
                in_row = -3;
            }
        }
        for col in 0..3 {
            let i = col + 3 * row;
            if cells[i] == ' ' {
                if in_row == 2 {
                    return i;
                } else if in_row == 1 {
                    possible_moves.push(i);
                }
            }
        }
    }

    for col in 0..3 {
        let mut in_col = 0;
        for row in 0..3 {
            let i = col + 3 * row;
            if cells[i] == 'X' {
                in_col += 1;
            } else if cells[i] == 'O' {
                in_col = -3;
            }
        }
        for row in 0..3 {
            let i = col + 3 * row;
            if cells[i] == ' ' {
                if in_col == 2 {
                    return i;
                } else if in_col == 1 {
                    possible_moves.push(i);
                }
            }
        }
    }

    let diags = [[0, 4, 8], [2, 4, 6]];

    for diag in diags {
        let mut x = 0;
        for i in diag {
            if cells[i] == 'X' {
                x += 1
            } else if cells[i] == 'O' {
                x = -3;
            }
        }
        for i in diag {
            if cells[i] == ' ' {
                if x == 2 {
                    return i;
                } else if x == 1 {
                    possible_moves.push(i);
                }
            }
        }
    }

    if possible_moves.is_empty() {
        return cells.iter().position(|&n| n == ' ').unwrap();
    }

    *possible_moves
        .get(random_usize(possible_moves.len()))
        .unwrap()
}

fn random_usize(range: usize) -> usize {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    (millis % range as u128) as usize
}
