use std::fs;
use std::collections::HashMap;

fn main() {

    let binding = fs::read_to_string("input").expect("No file\n");
    let data = binding.split("\n\n").collect::<Vec<_>>();
    
    let mut workflows : HashMap<&str, Vec<&str>> = HashMap::new();
    for el in data[0].split("\n") {
        let string = el.split("{").collect::<Vec<_>>();
        let workflow = string[1].split("}").nth(0).unwrap().split(",").collect::<Vec<_>>();
        workflows.insert(string[0], workflow);
    }
    for el in &workflows {
        println!("{:?}",el);
    }

    let mut sum = 0;
    for el in data[1].split("\n") {
        
        let string = el.split("{").nth(1).unwrap().split("}").nth(0).unwrap().split(",").collect::<Vec<_>>();
        let part :(u32,u32,u32,u32) = (string[0].split("=").nth(1).unwrap().parse().unwrap(), string[1].split("=").nth(1).unwrap().parse().unwrap(), string[2].split("=").nth(1).unwrap().parse().unwrap(), string[3].split("=").nth(1).unwrap().parse().unwrap());
        
        let mut workflow = "in";
        let mut end_loop = false;
        
        while !end_loop {
            if let Some(steps) = workflows.get(workflow) {
                for step in steps {
                    let instruction = step.split(":").collect::<Vec<_>>();
                    if instruction.len()==1 {
                        workflow=instruction[0];
                    } else {
                        let minor = instruction[0].split("<").collect::<Vec<_>>();
                        if minor.len()>1 {
                            if minor[0] == "x" {
                                if part.0 < minor[1].parse().unwrap(){
                                    workflow=instruction[1];
                                    break;
                                }
                            } else if minor[0]=="m" {
                                if part.1 < minor[1].parse().unwrap(){
                                    workflow=instruction[1];
                                    break;
                                }
                            } else if minor[0]=="a" {
                                if part.2 < minor[1].parse().unwrap(){
                                    workflow=instruction[1];
                                    break;
                                }                                
                            }else if minor[0]=="s" {
                                if part.3 < minor[1].parse().unwrap(){
                                    workflow=instruction[1];
                                    break;
                                }                                
                            }
                        } else {
                            let major = instruction[0].split(">").collect::<Vec<_>>();
                            if major[0] == "x" {
                                if part.0 > major[1].parse().unwrap(){
                                    workflow=instruction[1];
                                    break;
                                }
                            } else if major[0]=="m" {
                                if part.1 > major[1].parse().unwrap(){
                                    workflow=instruction[1];
                                    break;
                                }
                            } else if major[0]=="a" {
                                if part.2 > major[1].parse().unwrap(){
                                    workflow=instruction[1];
                                    break;
                                }
                            }else if major[0]=="s" {
                                if part.3 > major[1].parse().unwrap(){
                                    workflow=instruction[1];
                                    break;
                                }
                            }
                        }
                    }
                }
            }else{ // "A" or "R"
                if workflow=="A" {
                    sum+=part.0+part.1+part.2+part.3;
                } 
                end_loop = true;
            }
        }
    }

    println!("\nresult: {}", sum)


}