use std::fs;
use std::collections::HashMap;

/* maybe could be done with polymorphism but I'm lazy*/
#[derive(Debug, Clone, PartialEq)]
struct ModuleData {
    m_type: char,
    state: bool,
    inputs: HashMap<String, u8>,
    outputs: Vec<String>
}

fn check_states(modules : &HashMap<String , ModuleData>) -> bool {
    for module in modules {
        if module.1.m_type == '%' {
            if module.1.state == true {
                return false;
            }
        } else if module.1.m_type == '&' {
            for input in &module.1.inputs {
                if *input.1 != 0 {
                    return false;
                }
            }
        }
    }
    return true;
}

fn main() {

    let mut modules : HashMap<String , ModuleData> = HashMap::new();
    

    for raw_module in fs::read_to_string("input").expect("No file\n").split("\n") {
        let module = raw_module.split(" -> ").collect::<Vec<_>>();

        let m_type :char;
        // let mut inputs :  HashMap<String, bool> = HashMap::new();
        let name :String;

        if module[0].chars().nth(0).unwrap() == '&' {
            name = module[0][1..].to_string();
            m_type = '&';
        }else if module[0].chars().nth(0).unwrap() == '%' {
            name = module[0][1..].to_string();
            m_type = '%';
        } else {
            name = module[0].to_string();
            m_type = 'b';
        }
        modules.insert(name, ModuleData{
            m_type: m_type,
            inputs: HashMap::new(),
            state: false,
            outputs: module[1].split(", ").map(|x| x.to_string()).collect::<Vec<String>>(),
        });
    }

    let mut new_modules = modules.clone();
    for el in &modules {
        for m in &el.1.outputs {
            if let Some( m_data ) = new_modules.get_mut(m) {
                if m_data.m_type == '&' {
                    m_data.inputs.insert(el.0.clone(), 0);
                }
            }
        }
    }
    modules = new_modules;

    // for el in &modules {
    //     println!("{:?}", el);
    // }
    
    let mut num_high_pulses = 0;
    let mut num_low_pulses = 0;
    let mut num_button_pushes = 0;

    let mut stop = false;
    while !stop {
        /* button push */
        
        num_low_pulses+= 1; 
        num_button_pushes+= 1; 
        //println!("num button pushes: {:?}", num_button_pushes);

        let mut current_modules_outputs : Vec<(String, u8)> = vec![("broadcaster".to_string(),0)];
        //println!("{:?}", current_modules_outputs);

        while !current_modules_outputs.is_empty() {
            let mut new_modules = modules.clone();
            let mut next_modules_outputs : Vec<(String, u8)> = vec![];
            for current_module in current_modules_outputs{
                if let Some( outputs ) = modules.get(&current_module.0) {
                    
                //println!("{:?} - {:?}", current_module.1, outputs.outputs);

                    if current_module.1 == 0 {
                        num_low_pulses+= outputs.outputs.len();
                    }else{
                        num_high_pulses+= outputs.outputs.len();
                    }
                        
                    for output in &outputs.outputs {                        
                        if let Some( next_module ) = new_modules.get_mut(output) {
                            if next_module.m_type == '%' {
                                if current_module.1 == 0 {
                                    next_modules_outputs.push( (output.clone(), if next_module.state { 0 } else { 1 }) );
                                    next_module.state = !next_module.state;
                                }
                            } else if next_module.m_type == '&' {
                                next_module.inputs.insert(current_module.0.clone(), current_module.1);
                                let mut all_high_pulses = true;
                                for input in &next_module.inputs{
                                    if *input.1 == 0 {
                                        all_high_pulses = false;
                                        break;
                                    }
                                }
                                next_modules_outputs.push( (output.clone(), if all_high_pulses { 0 } else { 1 }) );
                            }
                        }
        
                    }
                }
            }
            current_modules_outputs=next_modules_outputs;
            modules = new_modules;
            //println!();
            // for el in &modules {
            //     println!("{:?}", el);
            // }
            
            //println!("{:?}", current_modules_outputs);
        }
        
        
       stop = check_states(&modules) || num_button_pushes == 1000;


    }
    println!("num high: {:?}", num_high_pulses);
    println!("num low {:?}", num_low_pulses);
    println!("num button pushes {:?}", num_button_pushes);

    
    println!("num high total: {:?}", num_high_pulses*1000/num_button_pushes);
    println!("num low total {:?}", num_low_pulses*1000/num_button_pushes);
    println!("\nresult {:?}", (num_high_pulses*1000/num_button_pushes)*(num_low_pulses*1000/num_button_pushes) );
}