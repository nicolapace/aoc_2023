use std::fs;
use std::collections::HashSet;

fn do_step( x : usize , y: usize , direction : u8 , map : &Vec<Vec<char>> , path : &mut HashSet<(usize,usize,u8)>)  {

    if path.get(&(x,y,direction)) != None {
        return;
    }
    path.insert((x,y,direction));
    
    if direction == 0 {  
        match map[y][x] {
            '|'  => {if y+1<map.len()       {do_step(x,y+1,2, &map, path)} ; if y>0 {do_step(x,y-1,3, &map, path)} },
            '\\' => {if y+1<map.len()       {do_step(x,y+1,2, &map, path)} },
            '/'  => {if y>0                 {do_step(x,y-1,3, &map, path)} },
             _   => {if x+1<map[0].len()    {do_step(x+1,y,0, &map, path)} },
        }
    } else if direction == 1 {
        match map[y][x] {
            '|'  => {if y+1<map.len()       {do_step(x,y+1,2, &map, path)}; if y>0 {do_step(x,y-1,3, &map, path)} },
            '\\' => {if y>0                 {do_step(x,y-1,3, &map, path)} },
            '/'  => {if y+1<map.len()       {do_step(x,y+1,2, &map, path)} },
             _   => {if x>0                 {do_step(x-1,y,1, &map, path)} },
        }
    } else if direction == 2 {
        match map[y][x] {
            '\\' => {if x+1<map[0].len()    {do_step(x+1,y,0, &map, path)} },
            '/'  => {if x>0                 {do_step(x-1,y,1, &map, path)} },
            '-'  => {if x+1<map[0].len()    {do_step(x+1,y,0, &map, path)} ; if x>0 {do_step(x-1,y,1, &map, path)} },
             _   => {if y+1<map.len()       {do_step(x,y+1,2, &map, path)} },
        }
    } else /*if direction == 3*/ {
        match map[y][x] {
            '\\' => {if x>0                 {do_step(x-1,y,1, &map, path)} },
            '/'  => {if x+1<map[0].len()    {do_step(x+1,y,0, &map, path)} },
            '-'  => {if x+1<map[0].len()    {do_step(x+1,y,0, &map, path)} ; if x>0 {do_step(x-1,y,1, &map, path)} },
             _   => {if y>0                 {do_step(x,y-1,3, &map, path)} },
        }
    }
}

fn main() {
    let mut map :Vec<Vec<char>> = vec![];
    for line in fs::read_to_string("input").expect("No file\n").split("\n"){
        map.push(line.chars().collect::<Vec<_>>());
    }
    
    let mut path :HashSet<(usize,usize,u8)> = HashSet::new();
    let mut singularpath :HashSet<(usize,usize)> = HashSet::new();   
    do_step(0,0,0,&map,&mut path);
    for el in path {
        singularpath.insert((el.0,el.1));
    }
    println!("result {:?}", singularpath.len());
}