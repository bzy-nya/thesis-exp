mod utility;
use utility::*;


mod sat;
mod dep;
mod lll;
mod r#match;
mod random_space;
mod moser_tardos_algorithm;

mod exp;

#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let dataset = exp::load_dataset( &args[1] );

    match args[2].as_str() {
        "bench" => { exp::bench(&dataset, args[3].parse().unwrap() ); },
        _ => {}
    };
}
