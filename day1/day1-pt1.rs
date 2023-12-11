use std::fs;

fn main() {

    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n");

    let mut sum : u32 = 0;
    let mut found_l;
    let mut found_r;
    let mut ssize;

    let mut i;
    let mut j;
    for line in lines{

        ssize =  line.bytes().count();
        found_l = false;
        found_r = false;

        if ssize>0 {
            ssize = ssize-1;

            i = 0;
            j = ssize-i;
            while !(found_l && found_r) {
               if line.bytes().nth(i)!=None{

                     //left
                    if !found_l && line.bytes().nth(i).unwrap() >= b'0' && line.bytes().nth(i).unwrap() <= b'9'{
                        found_l = true;
                        sum+=((line.bytes().nth(i).unwrap()-b'0')*10) as u32;
                    }else{
                        i+=1;
                    }
                    //right
                    if !found_r && line.bytes().nth(j).unwrap() >= b'0' && line.bytes().nth(j).unwrap() <= b'9'{
                        found_r = true;
                        
                        sum+=(line.bytes().nth(j).unwrap()-b'0') as u32;
                    }else{
                        j-=1;
                    }

                }
            }
        }
        
    }
    println!("result: {}", sum);
}
