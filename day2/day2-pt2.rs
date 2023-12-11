use std::fs;

fn main() {

    let contents = fs::read_to_string("test").expect("No file\n");
    let lines = contents.split("\n");

    let mut sum : u32 = 0;
    
    for line in lines{

            let  game = line.split(": ").collect::<Vec<_>>()[1];
            let mut max_red : u32  = 0;
            let mut max_green : u32 = 0;
            let mut max_blue : u32 = 0;
            for ex in game.split("; ") {
                for el in ex.split(", ") {
                    let div : Vec<_> = el.split_whitespace().collect();
                    
                    let mut num : u32 = (div[0].bytes().nth(0).unwrap() - b'0') as u32;
                    if div[0].bytes().count() == 2 {
                        num = 10*num+(div[0].bytes().nth(1).unwrap() - b'0') as u32;
                    }         
                    match div[1].bytes().nth(0).unwrap()  {
                        b'r' => if num > max_red {max_red=num},
                        b'g' => if num > max_green {max_green=num},
                        b'b' => if num > max_blue {max_blue=num},
                        _ => print!("ziocan"),
                    }
                }
            }
            sum+= max_red*max_green*max_blue ;   
    }
    println!("result: {}", sum);
}
