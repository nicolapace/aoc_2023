use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn do_step( map: &Vec<Vec<char>>, starting_positions : &HashSet<(usize,usize)>, depth: &mut u8, cache : &mut HashMap<(usize,usize), HashSet<(usize,usize)>>, output: &mut HashSet<(usize,usize)> ) {

    let mut out : HashSet<(usize,usize)> = HashSet::new();
    
    for start in starting_positions {
        
        if let Some( res ) = cache.get(&start){
            out.extend(res.clone());
            continue;
        }        
        
        let mut partial_out : HashSet<(usize,usize)> = HashSet::new();
        if start.1+1 < map.len() && map[start.1+1][start.0] == '.'{
            partial_out.insert((start.0, start.1+1));
        }
        if start.0+1 < map[0].len() && map[start.1][start.0+1] == '.'{
            partial_out.insert((start.0+1, start.1));            
        }
        if start.1 > 0 && map[start.1-1][start.0] == '.'{
            partial_out.insert((start.0, start.1-1));            
        }
        if start.0 > 0 && map[start.1][start.0-1] == '.'{
            partial_out.insert((start.0-1, start.1));            
        }
        
        cache.insert(*start, partial_out.clone());
        out.extend(partial_out);
    }
    
    *depth+=1;
    if *depth<64{
        do_step(map,&out,depth,cache,output);
    }else{
        *output = out;
    }
}

fn main() {
    /* PARSING */
    let mut map : Vec<Vec<char>> = vec![];
    let mut start :(usize,usize) = (0,0);
    let mut y = 0;
    for line in fs::read_to_string("input").expect("No file\n").split("\n") {
        let mut line_vec = line.chars().collect::<Vec<char>>();
        for x in 0..line_vec.len(){
            if line_vec[x] == 'S' {
                start = (x,y);
                line_vec[x] = '.';
            }
        }
        map.push(line_vec);
        y+=1;
    }

    let mut cache : HashMap<(usize,usize), HashSet<(usize,usize)>> = HashMap::new();
    let mut output : HashSet<(usize,usize)> = HashSet::new();
    let mut starting_positions : HashSet<(usize,usize)> = HashSet::new();
    starting_positions.insert(start);

    do_step(&map, &starting_positions, &mut 0, &mut cache, &mut output );
    
    println!("result: {:?}",output.len());
}