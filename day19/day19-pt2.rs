use std::fs;
use std::collections::HashMap;

fn main() {

    let binding = fs::read_to_string("test").expect("No file\n");
    let data = binding.split("\n\n").collect::<Vec<_>>();
    
    let mut workflows : HashMap<&str, Vec<&str>> = HashMap::new();
    for el in data[0].split("\n") {
        let string = el.split("{").collect::<Vec<_>>();
        let workflow = string[1].split("}").nth(0).unwrap().split(",").collect::<Vec<_>>();
        workflows.insert(string[0], workflow);
    } 
    
    /* vector of vector is the graph of all possible workflows, and for each possible workflow the tuple is (min,max) for each "x","m","a","s"*/
    let mut workflows_graph : Vec<(Vec<&str>, ((u64,u64),(u64,u64),(u64,u64),(u64,u64)))> = vec![(vec!["in"], ((1,4000),(1,4000),(1,4000),(1,4000)))];

    let mut all_ended = false;
    while !all_ended {
        all_ended = true;
        let mut new_workflows_graph : Vec<(Vec<&str>, ((u64,u64),(u64,u64),(u64,u64),(u64,u64)) )> = vec![];
        for workflow in &workflows_graph {
            if let Some(steps) = workflows.get(workflow.0[workflow.0.len()-1]) {
                all_ended = false;
                for i in 0..steps.len() {
                    let mut new_workflow = workflow.clone();
                    
                    for j in 0..i {
                        let instruction = steps[j].split(":").collect::<Vec<_>>();
                        if instruction.len()!=1 {
                            let minor = instruction[0].split("<").collect::<Vec<_>>();
                            if minor.len()>1 {
                                let num = minor[1].parse::<u64>().unwrap();
                                if minor[0] == "x" {
                                    if num > new_workflow.1.0.0 {
                                        new_workflow.1.0.0 = num;
                                    }
                                } else if minor[0]=="m" {
                                    if num > new_workflow.1.1.0 {
                                        new_workflow.1.1.0 = num;
                                    }                                
                                } else if minor[0]=="a" {
                                    if num > new_workflow.1.2.0 {
                                        new_workflow.1.2.0 = num;
                                    }                                                            
                                }else if minor[0]=="s" {
                                    if num > new_workflow.1.3.0 {
                                        new_workflow.1.3.0 = num;
                                    }                                                          
                                }
                            }else {
                                let major = instruction[0].split(">").collect::<Vec<_>>();
                                let num = major[1].parse::<u64>().unwrap();
                                if major[0] == "x" {
                                    if num < new_workflow.1.0.1 {
                                        new_workflow.1.0.1 = num;
                                    }  
                                } else if major[0]=="m" {
                                    if num < new_workflow.1.1.1 {
                                        new_workflow.1.1.1 = num;
                                    }
                                } else if major[0]=="a" {
                                    if num < new_workflow.1.2.1 {
                                        new_workflow.1.2.1 = num;
                                    }
                                }else if major[0]=="s" {
                                    if num < new_workflow.1.3.1 {
                                        new_workflow.1.3.1 = num;
                                    }
                                }
                            }
                        }
                    }

                    let instruction = steps[i].split(":").collect::<Vec<_>>();
                    if instruction.len()==1 {
                        new_workflow.0.push(instruction[0]);
                    }else {
                        new_workflow.0.push(instruction[1]);

                        let minor = instruction[0].split("<").collect::<Vec<_>>();
                        if minor.len()>1 {
                            let num = minor[1].parse::<u64>().unwrap()-1;

                            if minor[0] == "x" {
                                if num < new_workflow.1.0.1 {
                                    new_workflow.1.0.1 = num;
                                }  
                            } else if minor[0]=="m" {
                                if num < new_workflow.1.1.1 {
                                    new_workflow.1.1.1 = num;
                                }
                            } else if minor[0]=="a" {
                                if num < new_workflow.1.2.1 {
                                    new_workflow.1.2.1 = num;
                                }
                            }else if minor[0]=="s" {
                                if num < new_workflow.1.3.1 {
                                    new_workflow.1.3.1 = num;
                                }
                            }
                        } else {
                            let major = instruction[0].split(">").collect::<Vec<_>>();
                            let num = major[1].parse::<u64>().unwrap()+1;
                            
                            if major[0] == "x" {
                                if num > new_workflow.1.0.0 {
                                    new_workflow.1.0.0 = num;
                                }
                            } else if major[0]=="m" {
                                if num > new_workflow.1.1.0 {
                                    new_workflow.1.1.0 = num;
                                }                                
                            } else if major[0]=="a" {
                                if num > new_workflow.1.2.0 {
                                    new_workflow.1.2.0 = num;
                                }                                                            
                            }else if major[0]=="s" {
                                if num > new_workflow.1.3.0 {
                                    new_workflow.1.3.0 = num;
                                }                                                          
                            }
                        }
                    }
                    
                    if new_workflow.0[new_workflow.0.len()-1]!="R"{
                        new_workflows_graph.push(new_workflow);
                    }
                }
            }else{
                if workflow.0[workflow.0.len()-1]!="R"{
                    new_workflows_graph.push(workflow.clone());
                }
            }
        }
        if !all_ended{
            workflows_graph = new_workflows_graph;
        }
    }

    let mut sum :u64 =0;
    for el in workflows_graph {
        sum +=(1+el.1.0.1-el.1.0.0)*(1+el.1.1.1-el.1.1.0)*(1+el.1.2.1-el.1.2.0)*(1+el.1.3.1-el.1.3.0);
    }
    
    println!("\nresult: {}", sum);
}