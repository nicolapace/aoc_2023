use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn next_node(nodes_map : &HashMap<(usize,usize), Vec<(u32, (usize,usize))> >, visited_nodes : &mut HashSet<(usize, usize)>, steps: u32, max_steps: &mut u32, current_node: (usize, usize) , end: (usize, usize) ) {
    
    if current_node == end {
        if steps>*max_steps {
            *max_steps = steps;
            println!("steps: {}", steps);
        }
        return;
    }

    let mut next_nodes :Vec<(u32, (usize,usize))> = nodes_map.get(&current_node).unwrap().to_vec();

    /* sort nodes by their distance (we need to visit the nodes more away before) */ //( actually this is not needed but makes everything faster)
    next_nodes.sort_by(|a,b| b.0.cmp(&a.0));

    for node in next_nodes{
        if visited_nodes.get(&node.1)==None {
            let mut visited_nodes_clone = visited_nodes.clone();
            visited_nodes_clone.insert(node.1);
            next_node(nodes_map, &mut visited_nodes_clone, steps+node.0, max_steps, node.1, end);
        }
    }
}

fn main() {
    /* PARSE MAP */
    let mut map : Vec<Vec<char>> = vec![];
    for line in fs::read_to_string("input").expect("No file\n").split("\n") {
        map.push( line.chars().collect::<Vec<char>>() );
    }
    /* FIND NODES (INTERSECTIONS OF STREETS) IN THE MAP */
    let mut nodes : HashSet<(usize, usize)> = HashSet::new();
    nodes.insert((1,0));
    nodes.insert((map[0].len()-2,map.len()-1));
    for j in 1..map.len()-1{
        for i in 1..map[j].len()-1{
            let mut num_slopes = 0;
            if map[j-1][i] == '>' || map[j-1][i] == '<' || map[j-1][i] == 'v' {
                num_slopes+=1;
            } 
            if map[j+1][i] == '>' || map[j+1][i] == '<' || map[j+1][i] == 'v' {
                num_slopes+=1;
            } 
            if map[j][i-1] == '>' || map[j][i-1] == '<' || map[j][i-1] == 'v' {
                num_slopes+=1;
            } 
            if map[j][i+1] == '>' || map[j][i+1] == '<' || map[j][i+1] == 'v' {
                num_slopes+=1;
            } 

            if num_slopes > 2  {
                nodes.insert((i,j));
            }
        }
    }
    /* BUILD AN HASHMAP OF node -> Vec<(distance, node)> so that we have a graph to traverse */ 
    let mut nodes_map : HashMap< (usize,usize) , Vec<(u32, (usize,usize))> > = HashMap::new();
    for node in &nodes {
        let mut temp_map = map.clone();
        temp_map[1][1] = 'v';
        temp_map[map.len()-2][map[0].len()-2] = 'v';
        let mut linked_nodes : Vec<(u32, (usize,usize))> = vec![]; 
        let mut pos_vec :Vec<(usize, usize)> = vec![];
        let this_pos :(usize, usize)= *node;

        if *node == (1,0) {
            temp_map[this_pos.1][this_pos.0] = 'O';
            pos_vec.push((1,1));
        } else if *node == (map[0].len()-2,map.len()-1) {
            temp_map[this_pos.1][this_pos.0] = 'O';
            pos_vec.push((map[0].len()-2,map.len()-2));
        } else {
            temp_map[this_pos.1][this_pos.0] = 'O';
            if temp_map[this_pos.1-1][this_pos.0] != '#' {
                pos_vec.push((this_pos.0,this_pos.1-1));
            } 
            if temp_map[this_pos.1+1][this_pos.0] != '#' {
                pos_vec.push((this_pos.0,this_pos.1+1));
            } 
            if temp_map[this_pos.1][this_pos.0-1] != '#' {
                pos_vec.push((this_pos.0-1,this_pos.1));
            } 
            if temp_map[this_pos.1][this_pos.0+1] != '#' {
                pos_vec.push((this_pos.0+1,this_pos.1));
            } 
        }

        for mut pos in pos_vec {
            temp_map[pos.1][pos.0] = '.';
            let mut  steps = 2;
            while temp_map[pos.1][pos.0] == '.' {
                steps+=1;
                temp_map[pos.1][pos.0] = 'O';
                
                if pos.0!=temp_map[0].len()-1 && temp_map[pos.1][pos.0+1]!='#' && temp_map[pos.1][pos.0+1]!='O' {
                    pos = (pos.0+1,pos.1);
                } else if pos.1!=temp_map.len()-1 && temp_map[pos.1+1][pos.0]!='#' && temp_map[pos.1+1][pos.0]!='O' {
                    pos = (pos.0,pos.1+1);
                } else if pos.0!=0 && temp_map[pos.1][pos.0-1]!='#' && temp_map[pos.1][pos.0-1]!='O' {
                    pos = (pos.0-1,pos.1);
                } else if pos.1!=0 && temp_map[pos.1-1][pos.0]!='#' && temp_map[pos.1-1][pos.0]!='O' { 
                    pos = (pos.0,pos.1-1);
                }             
            }
            
            if pos.1!=temp_map.len()-1 && nodes.get(&(pos.0,pos.1+1)) != None {
                linked_nodes.push((steps, (pos.0,pos.1+1)));
            }
            if pos.0!=temp_map[0].len()-1 && nodes.get(&(pos.0+1,pos.1)) != None {
                linked_nodes.push((steps, (pos.0+1,pos.1)));
            }
            if pos.1!=0 && nodes.get(&(pos.0,pos.1-1)) != None {
                linked_nodes.push((steps, (pos.0,pos.1-1)));
            }
            if pos.0!=0 && nodes.get(&(pos.0-1,pos.1)) != None {
                linked_nodes.push((steps, (pos.0-1,pos.1)));
            }
        }
        nodes_map.insert(node.clone(), linked_nodes);
    }

    // now we have the nodes in an hashmap, let's do DFS but ordering the nodes to visit first from higher to lower :)
    let mut current_path : HashSet<(usize, usize)> = HashSet::new();
    current_path.insert((1,0));
    next_node( &nodes_map, &mut current_path, 0, &mut 0, (1,0), (map[0].len()-2,map.len()-1));
}