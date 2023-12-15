use std::fs;

fn hash_func( s : &str ) -> usize {
    let mut current = 0;
    for el in s.bytes() {
        current+=el as usize;
        current*=17;
        current%=256;
    }
    current
}

fn main() {
    
    let mut boxes : Vec<Vec<(&str, usize)>> = vec![vec![];256];

    let binding = fs::read_to_string("test").expect("No file\n");
    for string in binding.split(","){

        let mut command = string.split("=").collect::<Vec<_>>();
        if command.len() == 1 {
            command = string.split("-").collect::<Vec<_>>();
            let hash = hash_func(command[0]);

            for i in 0..boxes[hash].len(){
                if command[0] == boxes[hash][i].0 {
                    boxes[hash].remove(i);
                    break;
                }
            }

        }else {
            let hash = hash_func(command[0]);
            let mut found = false;
            for i in 0..boxes[hash].len(){
                if boxes[hash][i].0 == command[0] {
                    boxes[hash][i].1 = command[1].parse().unwrap();
                    found = true;
                    break;
                }
            }
            if !found {
                boxes[hash].push( (command[0],command[1].parse().unwrap()) );
            }

        }
    }
    
    let mut sum = 0;
    for i in 0..boxes.len(){
        for j in 0..boxes[i].len(){
            sum += (i+1)*(j+1)*boxes[i][j].1;
        }
    }
    println!("result: {:?}", sum);   
}