use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone)]

struct BoardNumber {
    number: u32,
    marked: bool
}
type Board = Vec<Vec<BoardNumber>>;

struct BingoGame {
    calls: Vec<u32>,
    boards: Vec<Board>
}

fn show(board: &Board) {
    for line in board {
        println!();
        for num in line {
            let val = match num.marked {
                false => num.number.to_string(),
                true => "x".to_string()
            };
            print!("{:3} ", val);
        }
    }
}

fn read_board_line(line: String) -> Vec<BoardNumber> {
    line.split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .map(|x| BoardNumber{number: x, marked: false})
        .collect()
}

fn read_game(file: &File) -> BingoGame {
    let mut lines = io::BufReader::new(file).lines().map(|x| x.unwrap());
    let calls: Vec<u32> = lines.next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut boards = Vec::<Board>::new();
    for line in lines {
        match line.as_str(){
            "" => boards.push(Board::new()),
            _ => boards.last_mut().unwrap().push(read_board_line(line))
        }
    }
    BingoGame{
        calls: calls,
        boards: boards
    }
}

fn mark(board: &Board, call: u32) -> Board {
    board.iter()
        .map(|row| row.iter().map(|num| {
            BoardNumber{number: num.number, marked: num.number==call || num.marked}
        }).collect())
        .collect()
}

fn unmarked_sum(board: &Board) -> u32 {
    board.iter()
        .flat_map(|x| x.iter())
        .filter(|x| !x.marked)
        .map(|x| x.number)
        .sum()
}

fn score(board: &Board) -> Option<u32> {
    let row_win = board.iter()
        .map(|row| row.iter().all(|x| x.marked))
        .any(|x| x);

    let col_win = (0..board[0].len())
        .map(|i| board.iter().all(|row| row[i].marked))
        .any(|x| x);
    match (row_win || col_win) {
        false => None,
        true => Some(unmarked_sum(board))
    }
}

fn turn(boards: &Vec<Board>, call: u32) -> (Vec<Board>, Option<u32>)
{
    let mut boards: Vec<Board> = boards.into_iter().map(|b| mark(b, call)).collect();
    let mut last_score = None;

    boards.retain(|board| {
        match score(&board) {
            Some(s) => {
                last_score = Some(s*call);
                false
            }
            _ => true,
        }
    });
    (boards, last_score)
}

fn play(mut game: BingoGame) -> u32{
    for call in game.calls {
        match turn(&game.boards, call) {
            (_, Some(s)) => return s,
            (new_boards, None) => {game.boards = new_boards}
        }
    }
    panic!("no winners");
}

fn play_all(mut game: BingoGame) -> u32{
    let mut score = 0;
    for call in game.calls {
        match turn(&game.boards, call) {
            (new_boards, Some(s)) => {
                game.boards = new_boards;
                score = s;
            },
            (new_boards, None) => {
                game.boards = new_boards;
            }
        }
    }
    return score;
}

pub fn run(part: usize) -> io::Result<()> {
    let file = File::open("test_data/day4.txt")?;
    let game = read_game(&file);
    
    match part {
        1 => println!("score: {}", play(game)),
        2 => println!("score: {}", play_all(game)),
        _ => panic!("")
    }
    Ok(())
}