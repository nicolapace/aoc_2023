use std::fs;
// use std::collections::HashMap;
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
    let contents = fs::read_to_string("input").expect("No file\n");

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


        let mut found = false;   
        let mut result :usize= 0;

        let mut i = horizontal_vec.len()%2;
        while i<horizontal_vec.len() {
            if is_even_palindrome(&horizontal_vec[..i]/*, &mut cache_palindorme*/){
                found = true;
                result = 100*(i/2);
                break;
            }
            if is_even_palindrome(&horizontal_vec[i..]/*, &mut cache_palindorme*/){
                found = true;
                result = 100*( i + (horizontal_vec.len()-i)/2 );
                break;
            }
            i+=  2-horizontal_vec.len()%2 ;
        }
        
        if !found {
            let mut i = vertical_vec.len()%2;
            while i<vertical_vec.len() {
                if is_even_palindrome( &vertical_vec[..i] /*, &mut cache_palindorme*/){
                    result = i/2;
                    break;
                }
                if is_even_palindrome( &vertical_vec[i..], /*&mut cache_palindorme*/){
                    result = i + (vertical_vec.len()-i)/2 ;
                    break;
                }
                i+=  2-vertical_vec.len()%2 ;
            }
        }

        let mut new_result = 0;

        let mut found_flip = false;
        for i in 0..horizontal_vec.len() {
            for k in i..horizontal_vec.len() {
                if two_powers.get(&(horizontal_vec[i]^horizontal_vec[k])) != None {
                    let old_val = horizontal_vec[i];
                    horizontal_vec[i]=horizontal_vec[k];
                    
                    let mut ok = true;
                    let mut d = 0;
                    while i>= d && k+d < horizontal_vec.len() {
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
                            println!("new_result: {}", 100*(1+ i-d+ (horizontal_vec.len()-(i-d))/2));
                        }else{
                            println!("new_result: {}", 100*(k+d)/2);
                        }
                        break;
                    }
                }
            }
            if found_flip {
                break;
            }
        } 
        if found_flip {    
            let mut i = horizontal_vec.len()%2;
            while i<horizontal_vec.len() {
                
                if is_even_palindrome(&horizontal_vec[..i]/* ,&mut cache_palindorme */)&& 100*(i/2)!=result && (i/2)!=0  {
                    new_result = 100*(i/2);
                    break;
                }
                
                if is_even_palindrome(&horizontal_vec[i..]/*, &mut cache_palindorme*/) && 100*( i + ((horizontal_vec.len()-i)/2) )!=result {
                    new_result = 100*(i + (horizontal_vec.len()-i)/2 );
                    break;
                }
                i+=  2-horizontal_vec.len()%2 ;
            }
            
        }
        else {

            found_flip = false;
            
            for i in 0..vertical_vec.len() {
                for k in i..vertical_vec.len() {
                    if two_powers.get(&(vertical_vec[i]^vertical_vec[k])) != None  {
                        let old_val = vertical_vec[i];
                        vertical_vec[i]=vertical_vec[k];
                        
                        let mut d = 0;
                        let mut ok = true;
                        while i >= d && k+d < vertical_vec.len() {
                            if !is_even_palindrome( &vertical_vec[(i-d)..=(k+d)] /*, &mut cache_palindorme */) {
                                vertical_vec[i]=old_val;
                                ok = false;
                                break;
                            }
                            d+=1;
                        }
                        if ok {
                            found_flip = true;                            
                            if k+d==vertical_vec.len() {
                                println!("new_result: {}", (1+ i-d+ (vertical_vec.len()-(i-d))/2));
                            }else{
                                println!("new_result: {}", (k+d)/2);
                            }
                            break
                        }
                            
                    }
                }
                if found_flip {
                    break;
                }
            }
            if found_flip{
                let mut i = vertical_vec.len()%2;
                while i<vertical_vec.len() {
                    if is_even_palindrome( &vertical_vec[..i], /*&mut cache_palindorme*/) && (i/2)!=result {
                        new_result = i/2;
                        break;
                    }
                    if is_even_palindrome( &vertical_vec[i..] /*, &mut cache_palindorme*/) && i + (vertical_vec.len()-i)/2 != result  {
                        new_result = i + (vertical_vec.len()-i)/2 ;
                        break;
                    }
                    i+=  2-vertical_vec.len()%2 ;
                }
                
            
            }
        }

        println!("new_result: {}\n", new_result);
        sum+=new_result;       
    }
    println!("total: {}", sum);
    
}
