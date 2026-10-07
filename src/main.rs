use std::io;

fn main() {
    println!("welcome to tic tac toe!");

    let mut cells = [' '; 9];

    println!("| | | |\n| | | |\n| | | |");

    let mut active_player = 'X';
    let mut buf = String::new();
    'game_loop: loop {
        buf.clear();
        println!("player {active_player}, which cell do you wish to play in? (1-9)");

        let Ok(_) = io::stdin().read_line(&mut buf) else {
            println!("couldn't read input. please try again.");
            continue;
        };

        let Ok(mut input_field) = buf.trim().parse::<usize>() else {
            println!("invalid cell. please enter a number from 1 - 9.");
            continue;
        };

        if input_field > 9 {
            println!("invalid cell. please enter a number from 1 - 9.");
            continue;
        }
        input_field -= 1;

        if cells[input_field] != ' ' {
            println!("there's already a piece in that cell.");
            continue;
        }
        cells[input_field] = active_player;

        println!();
        for ca in cells.chunks(3) {
            for c in ca {
                print!("|{c}");
            }
            println!("|");
        }

        if has_won(&cells, active_player) {
            println!("congratulations, player {active_player}! you have won!");
            println!("new round? (y/n)");
            
            loop {
                buf.clear();
                let Ok(_) = io::stdin().read_line(&mut buf) else {
                    println!("couldn't read input. please try again.");
                    continue;
                };
                if buf.trim() == "y" {
                    cells.fill(' ');
                    println!("\n| | | |\n| | | |\n| | | |");
                    active_player = if active_player == 'X' { 'O' } else { 'X' };
                    continue 'game_loop;
                } else if buf.trim() == "n" {
                    println!("bye");
                    break 'game_loop;
                } else {
                    println!("dpmo. yes or no?")
                }
            }
        }

        active_player = if active_player == 'X' { 'O' } else { 'X' };
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
