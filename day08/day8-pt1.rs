use std::fs;
use std::collections::HashMap;

fn main() {
    /* PARSING */
    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n").collect::<Vec<_>>();
    let mut indications : Vec<usize> = vec![];
    for ind in lines[0].chars(){
        if ind == 'L' {
            indications.push(0);
        }else{
            indications.push(1);
        }
    }   
    let mut network : HashMap<&str, Vec<&str>> = HashMap::new();
    for i in 2..lines.len(){
        let key = lines[i].split(" = (").nth(0).unwrap();
        let values = lines[i].split(" = (").nth(1).unwrap().split(")").nth(0).unwrap().split(", ").collect::<Vec<_>>();
        network.insert(key, values);
    }

    let mut current = "AAA";
    let mut step = 0;
    println!("map: {:?}", network);
    while current!="ZZZ" {
        println!( "step - key: {} value: {:?}",current, network.get(current).unwrap() );
        current = network.get(current).unwrap()[indications[step % indications.len()]];
        step +=1
    }
    println!("result: {}", step)
}
