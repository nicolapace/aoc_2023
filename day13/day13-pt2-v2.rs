use std::fs;
// use std::collections::HashMap;
use std::cmp;
use std::collections::HashSet;

fn is_even_palindrome( vec : &[u32]/*,  cache: &mut HashMap<Vec<u32>, bool> */) -> bool{
    if vec.len() < 2 || vec.len() % 2 == 1 {
        return false;
    }
    // if let Some(result) = cache.get(&vec.to_vec()) {
    //     //println!("using cache");
    //     return *result;
    // }
    for i in 0..vec.len()/2 {
        if vec[i] != vec[vec.len()-1-i] {
            // cache.insert(vec.to_vec(), false);
            return false;
        }
    }
    // cache.insert(vec.to_vec(), true);
    return true;


}

fn main() {
    /* PARSING */
    let contents = fs::read_to_string("test").expect("No file\n");

    let mut two_powers :HashSet<u32> = HashSet::new();
    let mut pow = 1;
    for _ in 0..17{
        two_powers.insert(pow);
        pow*=2;
    }

    // let mut cache_palindorme : HashMap<Vec<u32>, bool> = HashMap::new();
    let mut sum =0;

    for block in contents.split("\n\n"){
        let mut horizontal_vec : Vec<u32> = vec![];
        
        let mut matrix : Vec<_> = vec![];

        for line in block.split("\n") {
            let mut num = 0;
            let mut array : Vec<u32> = vec![];
            for el in line.chars() {
                if el == '#' {
                    num+=1;
                    array.push(1);
                }else{
                    array.push(0);
                }
                num = num<<1;
            }
            horizontal_vec.push(num>>1);
            matrix.push(array);
        }

        let mut vertical_vec : Vec<u32> = vec![0;matrix[0].len()];
        for i in 0..matrix[0].len(){
            for row in &matrix {
                if row[i] == 1 {
                    vertical_vec[i]+=1;
                }
                vertical_vec[i]= vertical_vec[i]<<1;
            }
            vertical_vec[i]= vertical_vec[i]>>1;
        }

        let mut result = 0;

        let mut found_flip = false;
        for i in 0..horizontal_vec.len() {
            for k in i..horizontal_vec.len() {
                if two_powers.get(&(horizontal_vec[i]^horizontal_vec[k])) != None {
                    let old_val = horizontal_vec[i];
                    horizontal_vec[i]=horizontal_vec[k];
                    
                    let mut ok = true;
                    let mut d = cmp::min(horizontal_vec.len()-1-k, i);
                    
                    while i >= d && k+d < horizontal_vec.len() {
                       // println!("dentro d: {}",d );
                        if !is_even_palindrome( &horizontal_vec[(i-d)..=(k+d)] /*, &mut cache_palindorme*/) {
                            horizontal_vec[i]=old_val;
                            ok=false;
                            break;
                        }

                        d+=1;
                    }
                    if ok {
                        found_flip = true;
                        
                        if k+d==horizontal_vec.len() {
                            result+=100*( i-d+1 + (horizontal_vec.len() - (i-d))/2);
                        }else{
                            result+=100*(k+d)/2;
                        }
                        break;
                    }
                }
            }
            if found_flip {
                break;
            }
        } 
        if !found_flip {

            found_flip = false;
            
            for i in 0..vertical_vec.len() {
                for k in i..vertical_vec.len() {
                    if two_powers.get(&(vertical_vec[i]^vertical_vec[k])) != None  {
                        let old_val = vertical_vec[i];
                        vertical_vec[i]=vertical_vec[k];
                        
                        let mut ok = true;
                        let mut d = cmp::min(vertical_vec.len()-1-k, i);

                        while i >= d && k+d < vertical_vec.len() {
                            //println!("dentro d: {}",d );
                            if !is_even_palindrome( &vertical_vec[(i-d)..=(k+d)] /*, &mut cache_palindorme*/) {
                                vertical_vec[i]=old_val;
                                ok=false;
                                break;
                            }
                           
                            d+=1;
                        }
                        if ok {
                            found_flip = true;
                            
                            if k+d==vertical_vec.len() {
                                result+= i-d+1 + (vertical_vec.len() - (i-d))/2 ;
                            }else{
                                result+=(k+d)/2;
                            }
                            break;
                        }
                    }
                }
                if found_flip {
                    break;
                }
            }
        }

        sum+=result;       
    }
    println!("total: {}", sum);
    
}
