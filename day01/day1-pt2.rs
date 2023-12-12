use std::fs;

fn main() {
    
    let numbers = ["one","two","three","four","five","six","seven","eight","nine"];

    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n");

    let mut sum : u32 = 0;
    
    let mut found_l;
    let mut found_r;
    for line in lines{

        let ssize =  line.bytes().count();

        if ssize>0 {
            
            found_l = false;
            found_r = false;

            for i in 0..ssize{
               if line.bytes().nth(i)!=None{

                    //left
                    if !found_l && line.bytes().nth(i).unwrap() >= b'0' && line.bytes().nth(i).unwrap() <= b'9'{
                        found_l = true;
                        sum+=((line.bytes().nth(i).unwrap()-b'0')*10) as u32;
                    }
                    if !found_l && line.bytes().nth(i+1).unwrap() >= b'0' && line.bytes().nth(i+1).unwrap() <= b'9'{
                        found_l = true;
                        sum+=((line.bytes().nth(i+1).unwrap()-b'0')*10) as u32;
                    }
                    if !found_l && ssize>5 {
                        let mut diff = 5;
                        if i+1+5>ssize {
                           diff=ssize-(i+1); 
                        }
                        let substring = &line[i..i+diff];
                        for j in 0..numbers.len() {
                            if substring.contains(numbers[j]) {
                                found_l = true;
                                sum+=((j+1)*10) as u32;
                                break;
                            }
                        }
                    }

                    //right
                    if !found_r && line.bytes().nth(ssize-1-i).unwrap() >= b'0' && line.bytes().nth(ssize-1-i).unwrap() <= b'9'{
                        found_r = true;
                        sum+=(line.bytes().nth(ssize-1-i).unwrap()-b'0') as u32;
                    }
                    if !found_r  && ssize>4 && i>1 {
                        let mut diff = 5;
                        if i<4 {
                           diff=i+1; 
                        }
                        let substring = &line[ssize-i-1..ssize-i-1+diff];
                        for j in 0..numbers.len() {
                            if substring.contains(numbers[j]) {
                                found_r = true;
                                sum+=(j+1) as u32;
                                break;
                            }
                        }
                    }
                }
                if found_l && found_r {
                    break;
                }
            }
        }
        
        
    }
    println!("result: {}", sum);
}
