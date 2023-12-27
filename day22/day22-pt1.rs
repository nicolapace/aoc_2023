use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    /* PARSING */
    let mut snapshots : Vec< (Vec<usize>, Vec<usize>) > = vec![];
    for line in fs::read_to_string("input").expect("No file\n").split("\n") {
        let block = line.split("~").collect::<Vec<_>>();
        let start = block[0].split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
        let end = block[1].split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
        snapshots.push((start, end));
    }
    
    /* FIND DIMENSIONS OF GRID */
    let mut max : (usize, usize, usize) = (0,0,0); // x y z
    for snapshot in &snapshots {
        if snapshot.1[0] > max.0 {
            max.0 = snapshot.1[0] ;
        }
        if snapshot.1[1] > max.1 {
            max.1 = snapshot.1[1] ;
        }
        if snapshot.1[2] > max.2 {
            max.2 = snapshot.1[2] ;
        }
    }

    let mut grid : Vec<Vec<Vec<u32>>> = vec![vec![vec![0;max.0+1];max.1+1];max.2];
    let mut block_positions :HashMap<u32, Vec<(usize,usize,usize)>> = HashMap::new();

    let mut num :u32 = 1;
    for snapshot in &snapshots {
        for k in snapshot.0[2]..=snapshot.1[2]{
            for j in snapshot.0[1]..=snapshot.1[1]{
                for i in snapshot.0[0]..=snapshot.1[0]{
                    grid[k-1][j][i] = num;

                    if let Some( positions ) = block_positions.get_mut(&num) {
                        positions.push((i,j,k-1));
                    }else{
                        block_positions.insert(num, vec![(i,j,k-1)]);
                    }
                }
            }   
        }
        num+=1;
    }

    for k in 1..=grid.len(){
        let plane = &grid[grid.len()-k];

        for j in 0..plane.len() {
            for i in 0..plane[j].len() {
                print!("{}\t",plane[j][i]);
            }
            println!();
        }
        println!();   
    }
    //let mut block_shadows :HashMap<u32, ((usize,usize),(usize,usize)) > = HashMap::new();

    /* GRAVITY LOOP */
    let mut changed=true;
    while changed {
        changed=false;
        let mut new_block_positions :HashMap<u32, Vec<(usize,usize,usize)>> = HashMap::new();

        //one gravity step for each block
        for block in &block_positions{ //to do gravity faster one can order blocks by their z-axis min value and process low z axis first

            let mut minmax_z :(usize, usize)  = (9999,0); // min z and max z
            let mut minmax_y :(usize, usize)  = (9999,0); // min y and max y
            let mut minmax_x :(usize, usize)  = (9999,0); // min x and max x
    
            for pos in block.1{
                // x
                if pos.0 < minmax_x.0 {
                    minmax_x.0 = pos.0;
                }
                if pos.0 > minmax_x.1 {
                    minmax_x.1 = pos.0;
                }
                // y
                if pos.1 < minmax_y.0 {
                    minmax_y.0 = pos.1;
                }
                if pos.1 > minmax_y.1 {
                    minmax_y.1 = pos.1;
                }
                // z
                if pos.2 < minmax_z.0 {
                    minmax_z.0 = pos.2;
                }
                if pos.2 > minmax_z.1 {
                    minmax_z.1 = pos.2;
                }
            }
    
            if minmax_z.0>0{
    
                let mut all_free = true;    
                for j in minmax_y.0..=minmax_y.1{
                    for i in minmax_x.0..=minmax_x.1{
                        if grid[minmax_z.0-1][j][i]!=0 {
                            all_free = false;
                        }
                    }
                }
    
                if all_free {
                    for k in minmax_z.0..=minmax_z.1{
                        for j in minmax_y.0..=minmax_y.1{
                            for i in minmax_x.0..=minmax_x.1{
                                grid[k][j][i]=0;
                                grid[k-1][j][i]=*block.0;

                                if let Some( positions ) = block_positions.get(&block.0) {
                                    let mut new_positions = vec![];
                                    for position in positions{
                                        new_positions.push((position.0, position.1, position.2-1));
                                    }
                                    new_block_positions.insert(*block.0, new_positions);
                                }
                                changed=true;
                            }
                        }
                    }
                }else{
                    if let Some( positions ) = block_positions.get(&block.0) {
                        new_block_positions.insert(*block.0, positions.clone());
                    }
                }
            }else{
                if let Some( positions ) = block_positions.get(&block.0) {
                    new_block_positions.insert(*block.0, positions.clone());
                }
            }
        }
        block_positions = new_block_positions;
    }

    println!("-------------- v AFTER FALL v ----------------");
    
    for k in 1..=grid.len(){
        let plane = &grid[grid.len()-k];

        for j in 0..plane.len() {
            for i in 0..plane[j].len() {
                print!("{}\t",plane[j][i]);
            }
            println!();
        }
        println!();
        
    }

    /* BUILD UP AND DOWN FOR EACH BLOCK */
    let mut up_n_down :HashMap< u32 , (HashSet<u32>,HashSet<u32>)> = HashMap::new(); // block -> (num_blocks_up , num_blocks_up)
    for block in &block_positions{ 

        let mut minmax_z :(usize, usize)  = (9999,0); // min z and max z
        let mut minmax_y :(usize, usize)  = (9999,0); // min y and max y
        let mut minmax_x :(usize, usize)  = (9999,0); // min x and max x

        for pos in block.1{
            // x
            if pos.0 < minmax_x.0 {
                minmax_x.0 = pos.0;
            }
            if pos.0 > minmax_x.1 {
                minmax_x.1 = pos.0;
            }
            // y
            if pos.1 < minmax_y.0 {
                minmax_y.0 = pos.1;
            }
            if pos.1 > minmax_y.1 {
                minmax_y.1 = pos.1;
            }
            // z
            if pos.2 < minmax_z.0 {
                minmax_z.0 = pos.2;
            }
            if pos.2 > minmax_z.1 {
                minmax_z.1 = pos.2;
            }
        }

        let mut blocks_down :HashSet<u32> = HashSet::new();
        if minmax_z.0>0{
            for j in minmax_y.0..=minmax_y.1{
                for i in minmax_x.0..=minmax_x.1{
                    if grid[minmax_z.0-1][j][i]!=0 {
                        blocks_down.insert(grid[minmax_z.0-1][j][i]);
                    }
                }
            }
        }

        let mut blocks_up :HashSet<u32> = HashSet::new();
        if minmax_z.1<grid.len()-1{ 
            for j in minmax_y.0..=minmax_y.1{
                for i in minmax_x.0..=minmax_x.1{
                    if grid[minmax_z.1+1][j][i]!=0 {
                        blocks_up.insert(grid[minmax_z.1+1][j][i]);
                    }
                }
            }
        }
        up_n_down.insert(*block.0, (blocks_down, blocks_up));
    }

    /* COUNT POSSIBLE REMOVABLE BLOCKS */
    let mut num_block_disintegrateble = 0;
    for block in &up_n_down{ 

        if block.1.1.len() == 0 {
            num_block_disintegrateble+=1;
            println!("{:?}", block);
            continue;
        }else{
            let mut not_the_only_base = true;
            for up_block in &block.1.1{
                if up_n_down.get(&up_block).unwrap().0.len() == 1 {
                    not_the_only_base = false
                }
            }
            if not_the_only_base {
                num_block_disintegrateble+=1;
                println!("{:?}", block);
            }
        }
    }

    println!("result: {}", num_block_disintegrateble);

}