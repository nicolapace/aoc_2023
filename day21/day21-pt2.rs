use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {

    /* PARSING */
    let mut map : Vec<Vec<char>> = vec![];
    let mut start :(i32,i32) = (0,0);
    let mut y = 0;
    for line in fs::read_to_string("input").expect("No file\n").split("\n") {
        let mut line_vec = line.chars().collect::<Vec<char>>();
        for x in 0..line_vec.len(){
            if line_vec[x] == 'S' {
                start = (x as i32,y as i32);
                line_vec[x] = '.';
            }
        }
        map.push(line_vec);
        y+=1;
    }

    let mut out : HashMap<(i32,i32), HashSet<(i32,i32)>>;
    let mut starting_positions : HashMap<(i32,i32), HashSet<(i32,i32)>> = HashMap::new();
    let mut starting_world : HashSet<(i32,i32)> = HashSet::new();
    starting_world.insert((0,0));
    starting_positions.insert(start, starting_world);
    
    let y_len = map.len() as i32;
    let x_len = map[0].len() as i32; 
    
    // calculate steps
    let mut depth = 0;
    while depth < 131*4+65 {

        out = HashMap::new();

        for start_map in &starting_positions {
            let start = start_map.0;
            // down
            if map[((start.1+1) % (y_len)) as usize][start.0 as usize] == '.'{
                if let Some (new) = out.get_mut(&(start.0, (start.1+1) % (y_len))) {
                    if start.1+1 == y_len {
                        for world in start_map.1 {
                            new.insert((world.0, world.1+1));
                        }
                    }else{
                        new.extend(start_map.1);
                    }
                }
                else { 
                    let mut new_world_out : HashSet<(i32,i32)>;
                    if start.1+1 == y_len {
                        new_world_out = HashSet::new();
                        for world in start_map.1 {
                            new_world_out.insert((world.0, world.1+1));
                        }
                    }else{
                        new_world_out = start_map.1.clone();
                    }
                    out.insert((start.0,  (start.1+1) % (y_len)), new_world_out);
                }
            }
            // right
            if map[start.1 as usize][((start.0+1) % (x_len)) as usize] == '.'{
                if let Some (new) = out.get_mut(&((start.0+1) % (x_len),start.1)) {
                    if start.0+1 == x_len {
                        for world in start_map.1 {
                            new.insert((world.0+1, world.1));
                        }
                    }else{
                        new.extend(start_map.1);
                    }
                }else{
                    let mut new_world_out : HashSet<(i32,i32)>;
                    if start.0+1 == x_len {
                        new_world_out = HashSet::new();
                        for world in start_map.1 {
                            new_world_out.insert((world.0+1, world.1));
                        }
                    }else{
                        new_world_out = start_map.1.clone();
                    }
                    out.insert(((start.0+1) % (x_len), start.1), new_world_out);
                }
            }
            // up
            if map[(start.1-1).rem_euclid(y_len) as usize][start.0 as usize] == '.'{
                if let Some (new) = out.get_mut(&(start.0,(start.1-1).rem_euclid(y_len))) {
                    if start.1 == 0 {
                        for world in start_map.1 {
                            new.insert((world.0, world.1-1));
                        }
                    }else{
                        new.extend(start_map.1);
                    }
                } else {
                    let mut new_world_out : HashSet<(i32,i32)>;
                    if start.1 == 0 {
                        new_world_out = HashSet::new();
                        for world in start_map.1 {
                            new_world_out.insert((world.0, world.1-1));
                        }
                    }else{
                        new_world_out = start_map.1.clone();
                    }
                    out.insert((start.0, (start.1-1).rem_euclid(y_len)), new_world_out);
                }
            }
            // left
            if map[start.1 as usize][(start.0-1).rem_euclid(x_len) as usize] == '.'{
                if let Some (new) =  out.get_mut(&((start.0-1).rem_euclid(x_len),start.1)) {
                    if start.0 == 0 {
                        for world in start_map.1 {
                            new.insert((world.0-1, world.1));
                        }
                    }else{
                        new.extend(start_map.1);
                    }
                }else{
                    let mut new_world_out : HashSet<(i32,i32)> ;
                    if start.0 == 0 {
                        new_world_out = HashSet::new();
                        for world in start_map.1 {
                            new_world_out.insert((world.0-1, world.1));
                        }
                    }else{
                        new_world_out = start_map.1.clone();
                    }
                    out.insert(((start.0-1).rem_euclid(x_len), start.1), new_world_out);
                }
            }  
            // 
        }
        depth+=1;
        starting_positions = out;
    }
   
    // extract the explored worlds
    let mut worlds_explored : HashMap<(i32,i32), HashSet<(i32,i32)>> = HashMap::new();
    for el in &starting_positions{
        for world in el.1{

            if let Some ( positions ) = worlds_explored.get_mut(&world){
                positions.insert(*el.0);
            }else{
                let mut positions : HashSet<(i32,i32)> = HashSet::new();
                positions.insert(*el.0);
                worlds_explored.insert(*world,positions);
            }
          
        }
        
    }
    let mut sum = 0;
    for el in &starting_positions{
        sum+=el.1.len();
    }
    println!("result: {:?}\n",sum);
    
    // add empty world to produce a squared plot easier
    let mut max_world_y = -99999;
    let mut max_world_x = -99999;
    
    let mut min_world_y = 99999;
    let mut min_world_x = 99999;

    for world in &worlds_explored{
        if world.0.0 > max_world_x {
            max_world_x = world.0.0;
        }
        if world.0.0 < min_world_x {
            min_world_x = world.0.0;
        }

        if world.0.1 > max_world_y {
            max_world_y = world.0.1;
        }
        if world.0.1 < min_world_y {
            min_world_y = world.0.1;
        }
    }
    
    for world_y in min_world_y..=max_world_y{
        for world_x in min_world_x..=max_world_x{
            if worlds_explored.get(&(world_x, world_y))==None{
                let empty_pos : HashSet<(i32,i32)> = HashSet::new();
                worlds_explored.insert((world_x, world_y),empty_pos);
            }
        }
    }

    /* print map */
    // for world_y in min_world_y..=max_world_y{
    //     for j in 0..y_len{
    //         for world_x in min_world_x..=max_world_x{
    //             for i in 0..x_len{
    //                 if i == 5 && j == 5 {
    //                     print!("X");
    //                     continue;
    //                 }
    //                 if let Some ( world ) = worlds_explored.get_mut(&(world_x, world_y)){
    //                     if world.get(&(i,j))!=None{
    //                         print!("O");
    //                     }else{
    //                         print!(".");
    //                     }
    //                 }
    //             }
    //             print!(" ");
    //         }
    //         println!();
    //     }
    //     println!();        
    // }

    /* print map occupancy */
    for world_y in min_world_y..=max_world_y{
        for world_x in min_world_x..=max_world_x{
            if let Some( world )  = worlds_explored.get(&(world_x, world_y)) {
                sum=0;
                for j in 0..y_len{
                    for i in 0..x_len{
                        if world.get(&(i,j))!=None{
                            sum+=1;
                        }
                    }
                }
                print!("{}\t", sum);
            }
        }
        
        print!("\n");
    }

}


/*

world_size = 131
number_of_steps = 26501365

number_of_steps % world_size = 65

so each map made by number of steps % 65 will be "similar" to the solution,
the above code prints the total world divided in subworlds and then prints number of steps in each world
for example: map from 65+131*4 

0       0       0       911     5357    932     0       0       0
0       0       911     6224    7193    6215    932     0       0
0       911     6224    7193    7082    7193    6215    932     0
911     6224    7193    7082    7193    7082    7193    6215    932
5348    7193    7082    7193    7082    7193    7082    7193    5340
917     6206    7193    7082    7193    7082    7193    6207    929
0       917     6206    7193    7082    7193    6207    929     0
0       0       917     6206    7193    6207    929     0       0
0       0       0       917     5331    929     0       0       0
total: 289514

and for map from 65+131*5


0       0       0       0       911     5357    932     0       0       0       0
0       0       0       911     6224    7193    6215    932     0       0       0
0       0       911     6224    7193    7082    7193    6215    932     0       0
0       911     6224    7193    7082    7193    7082    7193    6215    932     0
911     6224    7193    7082    7193    7082    7193    7082    7193    6215    932
5348    7193    7082    7193    7082    7193    7082    7193    7082    7193    5340
917     6206    7193    7082    7193    7082    7193    7082    7193    6207    929
0       917     6206    7193    7082    7193    7082    7193    6207    929     0
0       0       917     6206    7193    7082    7193    6207    929     0       0
0       0       0       917     6206    7193    6207    929     0       0       0
0       0       0       0       917     5331    929     0       0       0       0
total: 432366

Let's write a formula to calculate the total for this configuration:


7082 + 7193*2*(n-1) + (7193+7082)*( 2*((n)*(n-1)//2) + n-1) + (911+6224 + 6215+932 + 917+6206 + 6207+929)*n + (911+5357+932)+(917+5331+929)+5348+5340+7193+7193

7082 + 7193*2*(n+1) + (7193+7082)*(n+1)*(n-1) + (911+6224 + 6215+932 + 917+6206 + 6207+929)*n + (911+5357+932)+(917+5331+929)+5348+5340

where n = number_of_steps//world_size -1

solution: 584211423220706
*/