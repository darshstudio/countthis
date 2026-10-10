use std::{env, process};
mod counter;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() < 2 {
        eprintln!("This command excpect 2 arguments countthis [OPTIONS] [FILE_PATH]");
        println!("\t[OPTIONS]\t<words | -words | --words>\t||\t<lines | -lines | --lines>");
        println!("\t[FILE_PATH]\t<path/to/file");
        process::exit(1)
    }

    let options = if args[0] == "words" || args[0] == "-words" || args[0] == "--words" {
        counter::CountingMode::Words
    } else if args[0] == "lines" || args[0] == "-lines" || args[0] == "--lines" {
        counter::CountingMode::Lines
    } else {
        counter::CountingMode::Words
    };

    let count_er = counter::Counter::set_counter(&options, &args[1]);

    match count_er.get_count() {
        Ok(count) => if &options == &counter::CountingMode::Words.clone() {
            if count < 11 && count > 1 {
                println!("This file contains {} words", count);
            } else {
                println!("This file contains {} word", count);
            }
        } else if &options == &counter::CountingMode::Lines.clone() {
            if count < 11 && count > 1 {
                println!("This file contains {} lines", count);
            } else {
                println!("This file contains {} line", count);
            }
        },
        Err(error) => eprintln!("Faild to count file records {}", error),
    }
}
