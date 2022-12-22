mod data;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod utils;

use utils::ansi_colors;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!(
        r"
        {}*{}
       /.\
      /{}..\
      /..{}\
     /.{}..{}\
     /...{}.\
    /..{}....\
    ^^^{}[_]{}^^^{}
    ",
        ansi_colors::BRIGHT_YELLOW,
        ansi_colors::GREEN,
        format!("{}o{}", ansi_colors::BRIGHT_RED, ansi_colors::GREEN),
        format!("{}o{}", ansi_colors::BRIGHT_MAGENTA, ansi_colors::GREEN),
        format!("{}o{}", ansi_colors::BRIGHT_CYAN, ansi_colors::GREEN),
        format!("{}o{}", ansi_colors::BRIGHT_YELLOW, ansi_colors::GREEN),
        format!("{}o{}", ansi_colors::BRIGHT_RED, ansi_colors::GREEN),
        format!("{}o{}", ansi_colors::BRIGHT_MAGENTA, ansi_colors::GREEN),
        ansi_colors::YELLOW,
        ansi_colors::GREEN,
        ansi_colors::DEFAULT,
    );

    if args.len() < 2 {
        println!("Usage: <day+phase>");
        println!("   <day+phase> : 1a, 1b, 2a, 2b ... 25a, 25b");
        return;
    }

    for s in args.iter().skip(1) {
        let s: &str = s;
        println!();
        println!(
            "{}----------------{}",
            utils::ansi_colors::BRIGHT_MAGENTA,
            utils::ansi_colors::DEFAULT
        );
        println!(
            "{}Executing {}...{}",
            utils::ansi_colors::BRIGHT_MAGENTA,
            s,
            utils::ansi_colors::DEFAULT
        );
        println!(
            "{}----------------{}",
            utils::ansi_colors::BRIGHT_MAGENTA,
            utils::ansi_colors::DEFAULT
        );

        let start = std::time::Instant::now();
        match s {
            "1a" => day01::test1(),
            "2a" => day02::test1(),
            "3a" => day03::test1(),
            "4a" => day04::test1(),
            "5a" => day05::test1(),
            "6a" => day06::test1(),
            "7a" => day07::test1(),
            "8a" => day08::test1(),
            "9a" => day09::test1(),
            "10a" => day10::test1(),
            "11a" => day11::test1(),
            "12a" => day12::test1(),
            "13a" => day13::test1(),
            "14a" => day14::test1(),
            "15a" => day15::test1(),
            "16a" => day16::test1(),
            "17a" => day17::test1(),
            "18a" => day18::test1(),
            "19a" => day19::test1(),
            "20a" => day20::test1(),
            "21a" => day21::test1(),
            "22a" => day22::test1(),
            "23a" => day23::test1(),
            "24a" => day24::test1(),
            "25a" => day25::test1(),
            "1b" => day01::test2(),
            "2b" => day02::test2(),
            "3b" => day03::test2(),
            "4b" => day04::test2(),
            "5b" => day05::test2(),
            "6b" => day06::test2(),
            "7b" => day07::test2(),
            "8b" => day08::test2(),
            "9b" => day09::test2(),
            "10b" => day10::test2(),
            "11b" => day11::test2(),
            "12b" => day12::test2(),
            "13b" => day13::test2(),
            "14b" => day14::test2(),
            "15b" => day15::test2(),
            "16b" => day16::test2(),
            "17b" => day17::test2(),
            "18b" => day18::test2(),
            "19b" => day19::test2(),
            "20b" => day20::test2(),
            "21b" => day21::test2(),
            "22b" => day22::test2(),
            "23b" => day23::test2(),
            "24b" => day24::test2(),
            "25b" => day25::test2(),
            _ => panic!("???"),
        }
        let elapsed = start.elapsed();
        println!(
            "{}Took: {:.2?}{}",
            utils::ansi_colors::DARK_GRAY,
            elapsed,
            utils::ansi_colors::DEFAULT
        );
    }
}
