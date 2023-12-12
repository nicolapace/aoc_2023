use std::fs;
use std::cmp;
use std::cmp::Ordering;

fn hand1_beats_hand2(hand1 : Vec<u32> , hand2 : Vec<u32>) -> Ordering {
    let mut cards1 = vec![0; 15];
    let mut cards2 = vec![0; 15];

    let mut points1 : Vec<u32>  = vec![];
    let mut points2 : Vec<u32>  = vec![];

    
    for i in 0..=4 {
        cards1[hand1[i] as usize]+=1;
        cards2[hand2[i] as usize]+=1;
    }

    for i in 2..=14 {
        if cards1[i]>0 {
            points1.push(cards1[i]);
        }
        if cards2[i]>0 {
            points2.push(cards2[i]);
        }
    }
    points1.sort_by(|a, b| b.cmp(a));
    points2.sort_by(|a, b| b.cmp(a));
    if points1.len()>0 {
        points1[0]+=cards1[1];
    }else{
        points1.push(cards1[1]);
    }
    if points2.len()>0 {
        points2[0]+=cards2[1];
    }else{
        points2.push(cards2[1]);
    }
    for i in 0..cmp::min(points1.len(),points2.len()) {
        if points1[i] > points2[i]{
            return Ordering::Greater;
        } else if points1[i] < points2[i]{
            return Ordering::Less;
        }
    }
    
    //draw, see order of cards
    for i in 0..=4 {
        if hand1[i]>hand2[i] {
            return Ordering::Greater;
        }
        else if hand2[i]>hand1[i] {
            return Ordering::Less;
        }
    }

    println!("ERROR YOU SHOULDN'T BE HERE!!!!!!");
    return Ordering::Equal;
}

fn main() {

    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n");
    let mut elements : Vec<_>  = vec![];

    for line in lines{
        let el = line.split_whitespace().collect::<Vec<_>>();
        let mut hand : Vec<u32>  = vec![];
        let mut bet :u32 = 0;
        for c in el[0].bytes(){
            match c  {
                b'J' => hand.push(1),
                b'T' => hand.push(10),
                b'Q' => hand.push(12),
                b'K' => hand.push(13),
                b'A' => hand.push(14),
                _    => hand.push((c-b'0').into()),
            }
        }

        let mut pow = 1;
        for i  in 1..=el[1].len(){
            bet+=pow*(el[1].bytes().nth(el[1].len()-i as usize).unwrap()-b'0') as u32;
            pow*=10;
        }
        hand.push(bet);
        elements.push(hand.clone());
        // println!("{:?}",hand);
    }

    let mut i = 0;
    while i < elements.len()-1{        
        println!("cmp: {:?} vs {:?} : {:?}", elements[i], elements[i+1], hand1_beats_hand2(elements[i].clone(),elements[i+1].clone())); 
        i+=2;
    }
    
    elements.sort_by(|a,b| hand1_beats_hand2(a.to_vec(),b.to_vec()));
    let mut sum = 0;
    for i in 1..=elements.len(){
        sum += (i as u32)*elements[i-1][5];
    }
    println!("elements: {:?}", elements); 
   
    println!("result: {}", sum); 
}
