use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn fill_gaps_memoized( group_size: usize,  element_num: usize,  memo: &mut HashMap<(usize, usize), HashSet<Vec<u32>>> ) -> HashSet<Vec<u32>> {

    if let Some(result) = memo.get(&(group_size, element_num)) {
        return result.clone();
    }
    if element_num == 1 {
        let mut result : HashSet<Vec<u32>> = HashSet::new();
        for i in 0..group_size {
            let mut v = vec![0;group_size];
            v[i]+=1;
            result.insert(v);
        }
        memo.insert((group_size, element_num), result.clone());
        return result;
    }    

    let mut result : HashSet<Vec<u32>> = HashSet::new();

    for mut el in fill_gaps_memoized(group_size, element_num-1, memo){
        for i in 0..group_size {
            el[i]+=1;
            if result.get(&el) == None { 
                result.insert(el.clone());
            }

            el[i]-=1;
        }
    }
    memo.insert((group_size, element_num), result.clone());
    return result;
}

fn main() {
    /* PARSING */
    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n").collect::<Vec<_>>();

    let mut total_combinations = 0;
    let mut memo: HashMap< (usize, usize) , HashSet<Vec<u32>>> = HashMap::new();

    for line in lines {
        let data = line.split_whitespace().collect::<Vec<_>>();
        
        let row = data[0].chars().collect::<Vec<_>>();
        let mut groups_sizes: Vec<_> = vec![];
        for num in data[1].split(",") {
            let mut k = 1;
            let mut i = 1;
            let mut num_int = 0;
            while i <= num.chars().count()  {
                num_int += k*(num.bytes().nth(num.chars().count()-i).unwrap() -b'0');
                k*=10;
                i+=1;
            }
            groups_sizes.push(num_int);
        }

        let mut tot_sizes = 0;
        for el in &groups_sizes {
            tot_sizes+=el;
        }
        if row.len()+1 == tot_sizes as usize + groups_sizes.len() {
            total_combinations += 1;
            continue;
        }
        
        /* BUILD ALL POSSIBLE COMBINATIONS (POSSIBLY DIRECTLY IMPOSING CONSTRAINTS: hard) */

        /* building all possible combinations is just a matter of putting 
           the "(row.len() - (tot_sizes + (groups_sizes.len()-1)))" missing '.'s inside blocks
           blocks beeing the groups, for example if  ["####", ".", "##", ".", "###"] are the groups
           and the missing '.'s is just 1, you can put it in 5 different positions.
           so it's just a matter of putting k objects in n slots, or assiginig a number to k objects, 
           with repetition. So there are (k - (n-1))! / (k! (n-1)!) combinations.
           with n beeing the number of groups_sizes + number of groups_sizes - 1
           and with k beeing the number of missing '.'s 
           
           From this n^k possibile combinations we need to remove the impossible ones. */

        /* one optimization can be to remove '.'s at the start or the end of blocks or reducing consecutive '.'s to just 1 '.'*/

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

        if ( tot_sizes as usize + groups_sizes.len())  > ( optimized_row.len() + 1 ){
            continue;
        }
        let num_missing_empty = ( optimized_row.len() + 1 ) - (tot_sizes as usize + groups_sizes.len())   ;

        let mut tot_groups :Vec<_> = vec![];
        for i in 0..groups_sizes.len() {
            
            let string :String = "#".to_string();
            if i != groups_sizes.len()-1 {
                tot_groups.push(string.to_string()+&String::from("."));
            }else{
                tot_groups.push(string.to_string());
                tot_groups.push(String::from(""));
            }
        }

        //println!("{:?} - n = {:?}, missing: {:?}",tot_groups, tot_groups.len(), num_missing_empty );

        let combinations = fill_gaps_memoized(tot_groups.len(), num_missing_empty, &mut memo);

        //println!("number of total combinations: {:?} ", combinations.len() );
        // let mut good_combinations_num = combinations.len();
       
        /* CHECK COMBINATIONS AGAINST CONSTRAINTS TO REMOVE IMPOSSIBLE COMBINATIONS)*/
        for el in combinations {
            let mut exploded : Vec<char> = vec![];
            
            for part in 0..el.len() {

                for _ in 0..el[part]{
                    exploded.push('.');
                }

                if part < el.len()-1 {
                    
                    for _ in 0..groups_sizes[part] {
                        exploded.push('#');
                    }
                    if part != groups_sizes.len()-1 {    
                        exploded.push('.');
                    }
                }
            }
                
            //println!("{:?} {:?}", exploded , optimized_row);
            
            let mut possibile = true;
            for i in 0..optimized_row.len(){
                if optimized_row[i]!='?' && optimized_row[i]!=exploded[i] {
                    possibile = false;
                    //good_combinations_num-=1;
                    break;
                }
            }
            if possibile {
                total_combinations+=1;
                //println!("{:?} {:?}", exploded , optimized_row);
            }
        }

       //println!("good combinations: {:?} , total: {:?}",  good_combinations_num, combinations.len() );
    }
   println!("total_combinations: {:?} ", total_combinations );
    
}
