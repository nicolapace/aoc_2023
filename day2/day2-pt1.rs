use std::fs;

fn main() {

    let contents = fs::read_to_string("input").expect("No file\n");
    let lines = contents.split("\n");

    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum : u32 = 0;
    let mut i=1;
    
    for line in lines{

            let  game = line.split(": ").collect::<Vec<_>>()[1];
            let mut possibile = true;
            for ex in game.split("; ") {
                for el in ex.split(", ") {
                    let div : Vec<_> = el.split_whitespace().collect();
                    
                    let mut num = div[0].bytes().nth(0).unwrap() - b'0' ;
                    if div[0].bytes().count() == 2 {
                        num = 10*num+(div[0].bytes().nth(1).unwrap() - b'0') ;
                    }         
                    match div[1].bytes().nth(0).unwrap()  {
                        b'r' => if num > max_red {possibile=false},
                        b'g' => if num > max_green {possibile=false},
                        b'b' => if num > max_blue {possibile=false},
                        _ => print!("ziocan"),
                    }
                    if !possibile {
                        break;
                    }
                }
                if !possibile {
                    break;
                }
            }
            if possibile {
                sum+=i;
            }
        
            i+=1;   
   
    }
    println!("result: {}", sum);
}
