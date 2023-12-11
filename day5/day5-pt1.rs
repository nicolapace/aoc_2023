use std::fs;
//use std::collections::HashSet;

fn main() {

    let contents = fs::read_to_string("input").expect("No file\n");
    let parts = contents.split("\n\n");
    let seeds : Vec<u64> = parts.clone().collect::<Vec<_>>()[0].split("seeds: ").nth(1).unwrap().split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect();
    println!("seeds: {:?}",seeds );

    /* PARSING */
    let mut connections = vec![];
    let mut i = 1;
    while i<parts.clone().count(){
        let rows = parts.clone().collect::<Vec<_>>()[i].split(":\n");
        
        let row =  rows.clone().collect::<Vec<_>>()[1].split("\n").collect::<Vec<_>>();

        let mut parsed_rows = vec![];
        for subrow in row {
            let temp =  subrow.split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
            parsed_rows.push(temp);
        }
        connections.push(parsed_rows);
        
        i+=1;
    }
    // for _a in &connections{
    //     println!("{:?}", _a);
    // }

    
    let mut min = 4294967295u64;

    /* SEARCH ALGORITHM */
    for seed in seeds{
        
        print!("seed: {:?} ", seed);
        let mut comp = seed as u64;
        let mut i = 0;
        while i < connections.len() {            
            for row in &connections[i]{
                if row[1]<=comp && row[1]+row[2]>=comp {
                    comp = row[0]+(comp-row[1]);
                    break;
                }
            }
            i+=1;
            print!(" -> {:?}", comp);
        }
        println!();
        if comp<min {
            min=comp;
        }
    }
    println!("result:  {:?} ", min);
}
