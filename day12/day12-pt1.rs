use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn fill_gaps(vec_of_vec : &Vec<Vec<String>>,  to_fill : u32) -> Vec<Vec<String>>{

    if to_fill > 0 {
        
        let mut new_rows : Vec<Vec<String>> = vec![];
        for vec in vec_of_vec {
            for j in 0..vec.len(){
                let mut new_row : Vec<String> = vec![];
                for k in 0..j{
                    new_row.push(vec[k].clone());
                }
                new_row.push(String::from(".") + &vec[j]);
                for k in j+1..vec.len(){
                    new_row.push(vec[k].clone());
                }
                new_rows.push(new_row);
            }
        }
        
        let hashset : HashSet<Vec<String>> = HashSet::from_iter(new_rows.iter().cloned());
        if to_fill > 1 {
            return fill_gaps( &Vec::from_iter(hashset.iter().cloned()) , to_fill-1);
        }else {
            return Vec::from_iter(hashset.iter().cloned());
        }
    }else {
        return vec_of_vec.to_vec();
    }
}

fn main() {
    /* PARSING */
    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n").collect::<Vec<_>>();

    let mut total_combinations = 0;

    for line in lines {
        let data = line.split_whitespace().collect::<Vec<_>>();
        
        let row = data[0].chars().collect::<Vec<_>>();
        let mut groups_sizes: Vec<_> = vec![];
        let mut tot_sizes = 0;
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
            tot_sizes+=num_int;
        }

        if row.len() == tot_sizes as usize + (groups_sizes.len()-1) as usize {
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
        
        if optimized_row.len() == tot_sizes as usize + (groups_sizes.len()-1) as usize {
            total_combinations += 1;
            continue;
        }

        //println!("{:?} \n{:?} \n", row, optimized_row );


        let num_missing_empty = optimized_row.len() as u32 - (tot_sizes  as u32 + (groups_sizes.len() as u32 -1)) ;

        //let num_tot_groups = groups_sizes.len() + groups_sizes.len() - 1 ;

        let mut tot_groups :Vec<_> = vec![];
        for i in 0..groups_sizes.len() {
            let mut string :String = "".to_string();
            for _ in 0..groups_sizes[i]{
                string += "#"
            }
            if i != groups_sizes.len()-1 {
                tot_groups.push(string.to_string()+&String::from("."));
            }else{
                tot_groups.push(string.to_string());
                tot_groups.push(String::from(""));
            }
        }

        // println!("{:?} - n = {:?}, missing: {:?}",tot_groups, tot_groups.len(), num_missing_empty );

        let combinations = fill_gaps(&vec![tot_groups], num_missing_empty );
        // println!("number of total combinations: {:?} ", combinations.len() );
       
        /* CHECK COMBINATIONS AGAINST CONSTRAINTS TO REMOVE IMPOSSIBLE COMBINATIONS)*/
        for el in combinations {
            let mut exploded : Vec<char> = vec![];
            for part in el {
                if part != "" {
                    exploded.append(&mut part.chars().collect::<Vec<char>>());
                }
            }
            
            let mut possibile = true;
            for i in 0..optimized_row.len(){
                if optimized_row[i]!='?' && optimized_row[i]!=exploded[i] {
                    possibile = false;
                    break;
                }
            }
            if possibile {
                total_combinations+=1;
                // println!("{:?} {:?}", exploded , optimized_row);
            }
        }
    }
    println!("total_combinations: {:?} ", total_combinations );
    
}
