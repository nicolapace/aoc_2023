use std::fs;
use std::collections::HashMap;

pub fn lcm(nums: Vec<u64>) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm((&nums[1..]).to_vec());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

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
    let mut currents : Vec<&str> = vec![];
    for i in 2..lines.len(){
        let key = lines[i].split(" = (").nth(0).unwrap();
        let values = lines[i].split(" = (").nth(1).unwrap().split(")").nth(0).unwrap().split(", ").collect::<Vec<_>>();
        network.insert(key, values);
        if key.chars().nth(2).unwrap()=='A'{
            currents.push(key);
        }
    }
    
    println!("starts: {:?}", currents);
    let mut steps :Vec<u64> = vec![];

    /* cycle each then do MCM */
    for i in 0..currents.len(){
        let mut step = 0;
        while currents[i].chars().nth(2).unwrap()!='Z' {
            currents[i] = network.get(currents[i]).unwrap()[indications[step % indications.len()]];
            step +=1
        }
        steps.push(step as u64);
    }
    println!("steps: {:?}", steps);
    println!("Least common multiple: {}", lcm(steps) );
}
