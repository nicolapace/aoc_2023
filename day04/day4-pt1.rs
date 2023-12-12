use std::fs;
use std::collections::HashSet;

fn main() {

    let contents = fs::read_to_string("test").expect("No file\n");
    let lines = contents.split("\n");

    let mut sum : u32 = 0;
    
    for line in lines {
        
        let mut winners : HashSet<u8> = HashSet::new();
        let card = line.split(": ").collect::<Vec<_>>()[1].split(" | ").collect::<Vec<_>>();

        let mut i=0;
        while i<card[0].chars().count(){
            if card[0].bytes().nth(i).unwrap()==b' ' {
                winners.insert(card[0].bytes().nth(i+1).unwrap()-b'0');
            }else{
                winners.insert((card[0].bytes().nth(i).unwrap()-b'0')*10 + card[0].bytes().nth(i+1).unwrap()-b'0');
            }
            i+=3;
        }
        println!("winners: {:?}", winners); 

        let mut i=0;
        let mut score=0;
        while i<card[1].chars().count(){
            let mut num;
            if card[1].bytes().nth(i).unwrap()==b' ' {
                num = card[1].bytes().nth(i+1).unwrap()-b'0';
            }else{
                num = (card[1].bytes().nth(i).unwrap()-b'0')*10 + card[1].bytes().nth(i+1).unwrap()-b'0';
            }
            if winners.get(&num)!=None {
                if score==0 {
                    score = 1;
                }else{
                    score*=2;
                }
            }
            i+=3;
        }
        sum+=score;
       
    }
    println!("result: {:?}", sum); 
}
