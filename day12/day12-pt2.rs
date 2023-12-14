
use std::fs;
use std::collections::HashMap;

fn find_solutions( springs: &Vec<char>, sizes: &Vec<u64>,  current_group_size : u64,  cache: &mut HashMap<(Vec<char>, Vec<u64>, u64), u64> ) -> u64 {
    
    if springs.len() == 0{
        if sizes.len()==0{
            return 1;
        }else{
            return 0;
        }
    }
    
    if sizes.len() == 0 {
        for i in 0..springs.len() {
            if springs[i]== '#'{
                return 0
            }
        }
        return 1;
    }
    if springs.len()==1 && sizes.len() == 1 {
        if current_group_size == sizes[0] {
            if springs[0]=='#'{
                return 0;
            }
            return 1;
        }
        if current_group_size == sizes[0]-1 {
            if springs[0]!='.'{
                return 1;
            }
            return 0;
        }
        return 0;
    }
    
    if current_group_size > sizes[0] {
        return 0;
    }
    if let Some(&result) = cache.get(&(springs.clone(), sizes.clone(), current_group_size)) {
        return result;
    }
    let mut num_solutions = 0;

    let symbols = if springs[0] == '?' {
        vec!['.', '#']
    } else {
        vec![springs[0] ]
    };

    for sym in symbols {
        if sym == '#' {
            num_solutions += find_solutions(&springs[1..].to_vec(), sizes, current_group_size+1, cache);
        } else /*if  sym = '.' */ {
            if current_group_size != 0 && sizes[0] == current_group_size {
                num_solutions += find_solutions(&springs[1..].to_vec(), &sizes[1..].to_vec(), 0, cache);
            } else if current_group_size == 0 {
                num_solutions += find_solutions(&springs[1..].to_vec(), sizes, 0, cache);
            }
        }
    }
    cache.insert((springs.to_vec(), sizes.to_vec(), current_group_size), num_solutions);

    return num_solutions;
}

fn main() {
    /* PARSING */
    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n").collect::<Vec<_>>();

    let mut total_combinations = 0;

    let mut memo: HashMap<(Vec<char>, Vec<u64>, u64), u64> = HashMap::new();

    for line in lines {
        let data = line.split_whitespace().collect::<Vec<_>>();
        
        let _row = data[0].chars().collect::<Vec<_>>();
        let mut row :Vec<char> = vec![];
        for i in 1..=5 {
            for el in &_row {
                row.push(*el);
            }
            if i != 5 {
                row.push('?');
            }
        }
        let mut _groups_sizes: Vec<u64> = vec![];
        let mut _tot_sizes :usize = 0;
        
        for num in data[1].split(",") {
            let mut k = 1;
            let mut i = 1;
            let mut num_int = 0;
            let count = num.chars().count();
            while i <= count {
                num_int += k*(num.bytes().nth(count-i).unwrap() -b'0');
                k*=10;
                i+=1;
            }
            _groups_sizes.push(num_int as u64);
            _tot_sizes+=num_int as usize;
        }

        let mut groups_sizes: Vec<u64> = vec![];
        let tot_sizes = _tot_sizes*5;
        for _ in 0..5 {
            for s in &_groups_sizes{
                groups_sizes.push(*s);
            }
        }

        if row.len()+1 == tot_sizes+groups_sizes.len() {
            total_combinations += 1;
            continue;
        }

        let mut optimized_row : Vec<_> = vec![];
        let mut value_before = row[0];
        optimized_row.push(value_before);
        for i in 1..row.len(){
            if value_before == '.' && row[i] == '.' {
                continue;
            }else {
                value_before = row[i];
                optimized_row.push(row[i]);
            }
        }
        if optimized_row[0] == '.'{
            optimized_row.remove(0);
        }
        if optimized_row[optimized_row.len()-1] == '.'{
            optimized_row.pop();
        }

        // let mut remove_group = true;
        // while optimized_row.len() > 1 && remove_group && optimized_row[0] == '?' &&  optimized_row[1] == '#' {
        //     for i in 1..=groups_sizes[0]+1 {
        //         if optimized_row[i as usize] != '#'{
        //             remove_group = false;
        //             break;
        //         }
        //     }
        //     if remove_group {
        //         for _ in 0..=groups_sizes[0]+1 {
        //             optimized_row.remove(0);
        //         }
        //         groups_sizes.remove(0);
        //         if optimized_row[0] == '.'  {
        //             optimized_row.remove(0);
        //         } 
        //     } 
        // }

        // let mut tot_sizes = 0;
        // for el in &groups_sizes {
        //     tot_sizes+=el;
        // }
        // if optimized_row.len() +1 == tot_sizes as usize + groups_sizes.len()  {
        //     total_combinations += 1;
        //     continue;
        // }

        // let mut remove_group = true;
        // while remove_group && optimized_row.len()>0 && optimized_row[0] == '#' {
        //     for i in 0..groups_sizes[0] {
        //         if optimized_row[i as usize] != '#'{
        //             remove_group = false;
        //             break;
        //         }
        //     }
        //     if remove_group {
        //         for _ in 0..=groups_sizes[0] {
        //             optimized_row.remove(0);
        //         }
        //         groups_sizes.remove(0);
        //         if optimized_row[0] == '.'{
        //             optimized_row.remove(0);
        //         }
        //     } 
        // }
        
        // let mut tot_sizes = 0;
        // for el in &groups_sizes {
        //     tot_sizes+=el;
        // }
        // if optimized_row.len() +1 == tot_sizes as usize + groups_sizes.len()  {
        //     total_combinations += 1;
        //     continue;
        // }

        // let mut remove_group = true;
        // while optimized_row.len()>2 && remove_group && optimized_row[optimized_row.len()-1] == '?' &&  optimized_row[optimized_row.len()-2] == '#' {
        //     for i in 1..=groups_sizes[groups_sizes.len()-1]+1 {
        //         if optimized_row[optimized_row.len()-1-i as usize] != '#'{
        //             remove_group = false;
        //             break;
        //         }
        //     }
        //     if remove_group {
        //         for _ in 0..=groups_sizes[groups_sizes.len()-1]+1 {
        //             optimized_row.pop();
        //         }
        //         groups_sizes.remove(groups_sizes.len()-1);
        //         if optimized_row[optimized_row.len()-1] == '.'{
        //             optimized_row.pop();
        //         }
        //     } 
        // }

        // let mut tot_sizes = 0;
        // for el in &groups_sizes {
        //     tot_sizes+=el;
        // }
        // if optimized_row.len() +1 == tot_sizes as usize + groups_sizes.len()  {
        //     total_combinations += 1;
        //     continue;
        // }

        // let mut remove_group = true;
        // while remove_group && optimized_row.len()>0 && optimized_row[optimized_row.len()-1] == '#' {
        //     for i in 0..groups_sizes[groups_sizes.len()-1] {
        //         if optimized_row[optimized_row.len()-1-i as usize] != '#'{
        //             remove_group = false;
        //             break;
        //         }
        //     }
        //     if remove_group {
        //         for _ in 0..=groups_sizes[groups_sizes.len()-1] {
        //             optimized_row.pop();
        //         }
        //         groups_sizes.remove(groups_sizes.len()-1);
        //         if optimized_row.len() > 0 && optimized_row[optimized_row.len()-1] == '.'{
        //             optimized_row.pop();
        //         }
        //     } 
        // }       

        let mut tot_sizes = 0;
        for el in &groups_sizes {
            tot_sizes+=el;
        }
        if optimized_row.len() +1 == tot_sizes as usize + groups_sizes.len()  {
            total_combinations += 1;
            continue;
        }

        total_combinations += find_solutions(& row , & groups_sizes ,0 , &mut memo) ;  
        
    }

    println!("result: {:?}", total_combinations);
}
