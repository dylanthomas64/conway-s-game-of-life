use std::{cell::Cell, fmt::Display, io::{Stdout, Write}, thread, time::Duration};
use crossterm::{style::{self, SetBackgroundColor, Color}, terminal, QueueableCommand, cursor::MoveTo};
use std::io::stdout;
use rand::Rng;

///Any live cell with two or three live neighbours survives.
///Any dead cell with three live neighbours becomes a live cell.
///All other live cells die in the next generation. Similarly, all other dead cells stay dead.


const NUM_ROWS: usize = 50; // (y_MAX)
const NUM_COLS: usize = 100; // (x_MAX)
const STARTING_LIVES: usize = 500;

fn main() {
    //startup
    let mut stdout = stdout();
    stdout.queue(terminal::EnterAlternateScreen);


    let mut frame = vec!(vec!(CellState::Dead; NUM_COLS); NUM_ROWS);

    //create random life
    let mut rng = rand::thread_rng();
    for _n in 0..STARTING_LIVES {
        let x = rng.gen_range(0, NUM_COLS);
        let y = rng.gen_range(0, NUM_ROWS);
        create_life((x, y), &mut frame)
    }

    loop {
        frame = step(&frame);
        thread::sleep(Duration::from_millis(100));
        create_screen(&mut stdout, &frame);
    }

}


fn create_screen(stdout: &mut Stdout, frame: &Vec<Vec<CellState>>) {
    for x in 0..NUM_COLS {
        for y in 0..NUM_ROWS {
            let cell = frame[y][x];
            match cell {
                CellState::Alive => {
                    stdout.queue(SetBackgroundColor(Color::Red));
                    stdout.queue(MoveTo(x as u16, y as u16));
                    print!(" ");
                },
                CellState::Dead => {
                    stdout.queue(SetBackgroundColor(Color::White));
                    stdout.queue(MoveTo(x as u16, y as u16));
                    print!(" ");
                }
            }

        }
    }
    stdout.flush().unwrap();
}

fn step(frame: &Vec<Vec<CellState>>) -> Vec<Vec<CellState>> {
    let mut next_frame = vec!(vec!(CellState::Dead; NUM_COLS); NUM_ROWS);
    for (y, row) in &mut frame.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let n = neighbors((x as i16, y as i16), &frame);
            match cell {
                CellState::Alive if (n == 2) | (n == 3) => {
                    //println!("({},{}) survives!", x, y);
                    next_frame[y][x] = CellState::Alive;
                },
                CellState::Dead if n == 3 => {
                    //println!("({},{}) begins life!", x, y);
                    next_frame[y][x] = CellState::Alive;
                },
                _ => {
                    //println!("({},{}) dies", x, y);
                }
            }

        }

    }
    next_frame
}

#[derive(Debug)]
#[derive(Clone,Copy)]
#[derive(PartialEq)]
enum CellState {
    Alive,
    Dead
}

fn create_life((x, y): (usize, usize), frame: &mut Vec<Vec<CellState>>) {
    frame[y][x] = CellState::Alive
}

fn neighbors((col, row): (i16, i16), frame: &Vec<Vec<CellState>>) -> u8 {
    let mut total = 0i16;
    // make sure self wont be included in total
    if frame[row as usize][col as usize] == CellState::Alive {
        total -= 1;
    }
    for y in row-1..=row+1 {
        for x in col-1..=col+1 {
            if (x < 0) | (y < 0) | (x >= NUM_COLS as i16) | (y >= NUM_ROWS as i16) {
                //println!("nope @ ({},{})", x, y);
            } else {
                let cell = frame[y as usize][x as usize];
                match cell {
                    CellState::Alive => {
                        total += 1;
                        //println!("yep @ ({},{})", x, y)
                    },
                    CellState::Dead => {
                        //println!("dead cell @ ({},{})", x, y)
                    }
                }
                
            }
        }
    }
    total as u8

}