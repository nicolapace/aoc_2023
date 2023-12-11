use std::fs;
use std::collections::HashSet;

fn main() {

    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n");

    let mut sum : u32 = 0;
    
    let mut vec = vec![1; lines.clone().count()];

    let mut l = 0;
    for line in lines {
        
        let mut winners : HashSet<u8> = HashSet::new();
        let card = line.split(": ").collect::<Vec<_>>()[1].split(" | ").collect::<Vec<_>>();

        let mut i=0;
        
        // reading winning numbers
        while i<card[0].chars().count(){
            if card[0].bytes().nth(i).unwrap()==b' ' {
                winners.insert(card[0].bytes().nth(i+1).unwrap()-b'0');
            }else{
                winners.insert((card[0].bytes().nth(i).unwrap()-b'0')*10 + card[0].bytes().nth(i+1).unwrap()-b'0');
            }
            i+=3;
        }

        i=0;
        let mut num_winners=0;
        while i<card[1].chars().count() {
            let mut num;
            if card[1].bytes().nth(i).unwrap()==b' ' {
                num = card[1].bytes().nth(i+1).unwrap()-b'0';
            }else{
                num = (card[1].bytes().nth(i).unwrap()-b'0')*10 + card[1].bytes().nth(i+1).unwrap()-b'0';
            }
            if winners.get(&num)!=None {
                num_winners+=1;
            }
            i+=3;
        }
        let mut k = 1;
        while k <= num_winners {
            vec[l+k]+= vec[l];
            k+=1;
        }
        println!("vec[{:?}]: {:?}", l, vec[l]); 
       l+=1;
    }
    println!("result: {:?}", vec.iter().sum::<u32>()); 
}
