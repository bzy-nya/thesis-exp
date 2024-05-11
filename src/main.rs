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

    let (turn_id, data_filter) = 
    if args.len() > 3 && args[3].starts_with("--filter=") 
      { (4, &args[3][9..]) } 
    else { (3, "") };

    match args[2].as_str() {
        "bench" => { exp::bench     (&dataset, args[turn_id].parse().unwrap(), data_filter); },
        "enum"  => { exp::enum_step (&dataset, args[turn_id].parse().unwrap(), data_filter); }
        "run"  => { exp::run(&dataset); }
        _ => {}
    };
}
