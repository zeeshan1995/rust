
use std::env;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashMap;


#[derive(Debug)]
struct Result {
    sum: f32,
    n: i32,
    min: f32,
    max: f32,
    mean: Option<f32>
}

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{args:?}");
    let input_file = &args[1];
    /*
    let lines = fs::read_to_string(input_file).expect("Reading input file error");
    let lines = lines.trim_end();
    //println!("{lines:?}");
    //println!("{lines}");
    let data = lines.split("\n");
    //println!("{data:?}");
    */

    let mut results: HashMap<String, Result> = HashMap::new();

    //let file = match File::open("/Users/zeeshan/project/rust/onebrc/data_1e8.txt") {
    let file = match File::open(input_file) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
    //for line in data {
        //println!("{line:?}");
        let xx = line.unwrap();
        //let mut info = line.split(";");
        let mut info = xx.split(";");
        let station = info.next().unwrap();
        let val = info.next().unwrap().parse::<f32>().unwrap();
        //println!("{station} => {val}");
        match results.get_mut(station) {
            Some(result) => {
                result.sum+=val;
                result.n+=1;
                result.min = result.min.min(val);
                result.max = result.max.max(val);
                //println!("{result:?}");
            },
            _ => {
                results.insert(station.to_string(), Result{sum:val, n:1, min:val, max:val, mean:None});
            }
        }

    }

    for (_, result) in results.iter_mut() {
        result.mean = Some(result.sum/(result.n as f32));
    }

    println!("===================");
    println!("{results:?}");
}
