use std::io::{Read, stdin};
use rand::{Rng, thread_rng};

#[derive(Debug, PartialEq)]
enum CellState {
    X,
    O,
}

#[derive(Debug)]
struct GameState<'a> {
    who_moves: CellState,
    field: Vec<Vec<Option<&'a CellState>>>,
    step_count: u8,
    is_won: bool,
}

impl GameState<'_> {
    fn build() -> GameState<'static> {
        let field = vec![vec![None; 3]; 3];
        // dbg!(&field);

        let step_count = 0;
        let who_moves = match thread_rng().gen_bool(0.5) {
            true => CellState::X,
            false => CellState::O,
        };

        GameState {
            who_moves,
            field: field.clone(),
            step_count: step_count.clone(),
            is_won: false,
        }
    }

    fn draw(&self) {
        println!();
        for (i, x) in self.field.iter().enumerate() {
            for y in x.iter() {
                match y {
                    None => print!(" - "),
                    Some(value) => print!(" {:?} ", value),
                };
            }
            println!("    {} 1   {} 2   {} 3   ", i+1, i+1, i+1);
        }
        println!();
    }

    fn update(&mut self) {
        println!("It's {:?} move", self.who_moves);
        self.draw();

        let mut inp = "".to_string();
        std::io::stdin().read_line(&mut inp).expect("Failed to read line");

        if self.make_move(&inp) {
            self.win();

            self.who_moves = match self.who_moves {
                CellState::O => CellState::X,
                CellState::X => CellState::O,
            };
        }
    }

    fn make_move(&mut self, coords: &String) -> bool {
        if !are_coords_correct(coords) {
            println!("Enter a correct coordinates");
            return false;
        }

        let coords: Vec<usize> = coords.split_whitespace().map(|x| {
            x.parse().unwrap()
        }).collect();
        let (y, x) = (coords[0], coords[1]);

        if self.field[y-1][x-1] != None {
            println!("This cell is already occupied");
            return false;
        }

        self.field[y-1][x-1] = match self.who_moves {
            CellState::O => Some(&CellState::O),
            CellState::X => Some(&CellState::X),
        };

        true
    }

    fn win(&mut self) {
        for i in 0..3 {
            let mut hor = true;
            let mut ver = true;
            let mut dia1 = true;
            let mut dia2 = true;
            for j in 0..3 {
                if self.field[i][j] != Some(&self.who_moves) {hor = false}
                if self.field[j][i] != Some(&self.who_moves) {ver = false}
                if self.field[j][j] != Some(&self.who_moves) {dia1 = false}
                if self.field[j][2-j] != Some(&self.who_moves) {dia2 = false}
            }
            if hor || ver || dia1 || dia2 {self.is_won = true}
        }
    }
}

fn are_coords_correct(coords: &String) -> bool {
    let coords: Vec<u8> = coords.split_whitespace().map(|x| {
        match x.parse::<u8>() {
            Ok(i) => i,
            Err(_) => 4, // Return false
        }
    }).collect();

    if coords.len() != 2 {return false;}
    if 3 < coords[0] || coords[0] < 1 {return false;}
    if 3 < coords[1] || coords[1] < 1 {return false;}

    true
}

fn main() {
    println!("Welcome to CLI tic tac toe!");
    println!("Type a row and column (both between 1 and 3)");
    let mut game = GameState::build();
    let mut draw = false;

    while !game.is_won {
        game.update();

        draw = true;
        for i in 0..3 {
            for j in 0..3 {
                if game.field[i][j] == None {draw = false}
            }
        }

        if draw {break}
    }

    if !draw {
        match game.who_moves {
            CellState::O => println!("X has won!"),
            CellState::X => println!("O has won!"),
        }
    } else {
        println!("Draw!");
    }


    let mut exit = "".to_string();
    stdin().read_line(&mut exit).expect("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_coordinates() {
        let mut coords = "1        2         ".to_string();
        assert_eq!(true, are_coords_correct(&coords));

        coords = "2 3".to_string();
        assert_eq!(true, are_coords_correct(&coords));

        coords = "1 4".to_string();
        assert_eq!(false, are_coords_correct(&coords));

        coords = "1a2".to_string();
        assert_eq!(false, are_coords_correct(&coords));

        coords = "-2 2     ".to_string();
        assert_eq!(false, are_coords_correct(&coords));

        coords = "1 2 3".to_string();
        assert_eq!(false, are_coords_correct(&coords));

        coords = "how it works?".to_string();
        assert_eq!(false, are_coords_correct(&coords));
    }
}