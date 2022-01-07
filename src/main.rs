
use std::{env, primitive}; 
use std::fs;

struct parse_args
{
    query:String,
    filename:String,
}

impl parse_args
{
    fn new(args:&[String]) -> parse_args
    {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let search_string = args[1].clone();
        let filename = args[2].clone();
        parse_args {query: search_string.to_lowercase(), filename:filename}
    }
}
fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect(); //panic if args contains unicode
    println!("{:?}",args);
    let config = parse_args::new(&args);
  

    println!("String to search {:?}",config.query);
    println!("File to search {:?}",config.filename);

    let result = search(&config.query,config.filename); 
    println!("Lines containing the query {:?}",result);


}

fn search(query:&String, filename:String) -> Vec<String>
{
    let contents = fs::read_to_string(filename)
                            .expect("Something went wrong reading the file");
    let mut results: Vec<String> = Vec::new();


    for line in contents.lines()
    {
        if line.to_lowercase().contains(query)
        {
            results.push(line.to_string());
        }
    }
    results
 }