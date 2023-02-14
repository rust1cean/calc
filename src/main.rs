mod utils;

use std::io;
use utils::{App, Data};

fn main() {
    Calc::run();
}

pub enum Storage {
    FirstValue = 0,
    Operation = 1,
    SecondValue = 2,
    Result = 3,
}

pub struct Calc;

impl Calc {
    pub fn run() {
        App::new()
            .mut_system(<Calc as Greetings>::main)
            .mut_system(<Calc as Body>::main)
            .run();
    }
}

impl Io for Calc {}
impl Greetings for Calc {}
impl Body for Calc {}

pub trait Io {
    fn menu(app: &mut App) {
        match input().as_str() {
            "r" => {
                app.mut_task(<Calc as Body>::run);
            }
            "q" => app.stop(),
            _ => {
                app.mut_task(Self::menu);
            }
        }
    }
}

pub trait Greetings {
    fn main(app: &mut App) {
        app.immut_task(Self::msg);
        app.mut_task(<Calc as Io>::menu);
    }

    fn msg() {
        println!("Hello!");
        println!("r - run");
        println!("q - quit");
    }
}

pub trait Body {
    fn main(app: &mut App) {
        app.mut_task(Self::first_value);
        app.mut_task(Self::operation);
        app.mut_task(Self::second_value);
        app.mut_task(Self::result);
        app.mut_task(<Calc as Io>::menu);
    }

    fn run(app: &mut App) {
        app.clear_systems();
        app.clear_tasks();
        app.mut_task(<Calc as Body>::main);
    }

    fn first_value(app: &mut App) {
        println!("First value:");
        let value = input().parse::<i32>().unwrap();
        let id = Storage::FirstValue as usize;
        let data = Data::I32(value);
        app.push_data(id, data);
    }

    fn operation(app: &mut App) {
        println!("Operation(+ - * /):");
        let value = input();
        match value.as_str() {
            "+" | "-" | "*" | "/" => {
                let id = Storage::Operation as usize;
                let data = Data::String(value);
                app.push_data(id, data);
            }
            _ => (),
        }
    }

    fn second_value(app: &mut App) {
        println!("Second value:");
        let value = input().parse::<i32>().unwrap();
        let id = Storage::SecondValue as usize;
        let data = Data::I32(value);
        app.push_data(id, data);
    }

    fn result(app: &mut App) {
        let (_, first) = app.rm_data(Storage::FirstValue as usize).unwrap();
        let (_, operation) = app.rm_data(Storage::Operation as usize).unwrap();
        let (_, second) = app.rm_data(Storage::SecondValue as usize).unwrap();
        let result = match operation {
            Data::String(s) if s == "+" => first + second,
            Data::String(s) if s == "-" => first - second,
            Data::String(s) if s == "*" => first * second,
            Data::String(s) if s == "/" => first / second,
            _ => panic!("Unexpected error!"),
        }
        .unwrap();

        if let Data::I32(result) = result {
            println!("Result: {result}");
        }
    }
}

fn input() -> String {
    let mut result = String::new();
    io::stdin().read_line(&mut result).unwrap();
    result.trim().to_owned()
}

// fn input() -> io::Result<String> {
//     let mut result = String::new();
//     io::stdin().read_line(&mut result)?;
//     Ok(result)
// }
