use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

/* maybe could be done with polymorphism */
#[derive(Debug, Clone, PartialEq)]
struct ModuleData {
    m_type: char,
    state: bool,
    inputs: HashMap<String, u8>,
    outputs: Vec<String>
}

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

    let mut modules : HashMap<String , ModuleData> = HashMap::new();
    
    /* parsing and building modules data structure */
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

    /* adding inputs to each module */
    let mut new_modules = modules.clone();
    for el in &modules {
        for m in &el.1.outputs {
            if let Some( m_data ) = new_modules.get_mut(m) {
                m_data.inputs.insert(el.0.clone(), 0);
            }
        }
    }
    modules = new_modules;

    /* divide module in groups for each branch of broadcaster */
    let groups : HashSet<String> = HashSet::from_iter(modules.get("broadcaster").unwrap().outputs.clone()); // brances of brodcaster
    
    /* build hashmap of module -> branches which is part of */
    let mut module_groups : HashMap<String, HashSet<String>> = HashMap::new();
    for group in &groups {
        let mut current_modules_outputs : HashSet<String> = vec![group.clone()].into_iter().collect();
        let mut all_inside=false;
        while !all_inside {
            let mut next_modules_outputs : HashSet<String> = HashSet::new();
            all_inside = true;
            for current_module in current_modules_outputs{
                if let Some( outputs ) = modules.get(&current_module) {
                    for output in &outputs.outputs {      
                        if let Some( nex_mod ) = module_groups.get_mut(output){
                            nex_mod.insert( group.clone() );
                        }else{
                            all_inside=false;
                            module_groups.insert(output.clone(), vec![group.clone()].into_iter().collect() );
                        }
                        next_modules_outputs.insert( output.clone() );        
                    }
                }
            }
            current_modules_outputs=next_modules_outputs;
        }
    }

    /* split modules in groups by the branch in which they are part of */
    let mut groups_map : HashMap<String, HashMap<String, ModuleData>> = HashMap::new();
    // inizialize group_map whit broadcaster with only one output for each branch
    for group in groups {
        let mut sub_modules_map : HashMap<String, ModuleData> = HashMap::new();
        sub_modules_map.insert("broadcaster".to_string(), ModuleData{
            m_type: 'b',
            inputs: HashMap::new(),
            state: false,
            outputs: vec![group.clone()],
        });
        groups_map.insert(group.clone(), sub_modules_map);
    }
    // put each module in the correct group branch
    for module in &module_groups {
        for group in module.1 {
            if let Some( subgroup ) = groups_map.get_mut(group){
                if let Some( module_data ) = modules.get(module.0){
                    // we need to filter out inputs coming from other branches
                    let mut new_inputs : HashMap<String, u8> = HashMap::new();
                    for input in &module_data.inputs{
                        if let Some( groups ) = module_groups.get(&input.0.clone()) {
                            for el in groups {
                                if el==group{
                                    new_inputs.insert(input.0.clone(), 0);
                                }
                            }
                        }
                    }
                    subgroup.insert( module.0.clone(), ModuleData{
                        m_type: module_data.m_type,
                        inputs: new_inputs,
                        state: false,
                        outputs: module_data.outputs.clone(),
                    } );
                }
            }
        }
    }

    /* now we calculate ne number of button pushes for each branch. since before "rx" there's a '&',
       all inputs to the node before "rx" must be 1, so when can use "superposition" principle,
       and the calculate the LCM of the numbers of presses */

    let mut num_pushes_vec :Vec<u64> = vec![];
    for submodules in groups_map {
        modules = submodules.1;
        let mut num_button_pushes :u64 = 0;
        let mut found = false;
        while !found {
            /* button push */
            num_button_pushes+= 1; 
            let mut current_modules_outputs : Vec<(String, u8)> = vec![("broadcaster".to_string(),0)];
            while !current_modules_outputs.is_empty() {
                let mut new_modules = modules.clone();
                let mut next_modules_outputs : Vec<(String, u8)> = vec![];
                for current_module in current_modules_outputs{
                    if let Some( outputs ) = modules.get(&current_module.0) {
                            
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

                                    if output == "kh" /*node before "rx" */ && all_high_pulses {
                                        println!("num button pushes: {:?}", num_button_pushes);
                                        found = true;
                                        num_pushes_vec.push(num_button_pushes);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if found {
                        break;
                    }
                }
                if found {
                    break;
                }
                current_modules_outputs=next_modules_outputs;
                modules = new_modules;
            }

        }
    }

    
    println!("result: {:?}", lcm(num_pushes_vec));
    
}