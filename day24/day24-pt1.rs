use std::fs;

fn main() {
    let mut x : Vec<i128> = vec![];
    let mut y : Vec<i128> = vec![];
    let mut vx : Vec<i128> = vec![];
    let mut vy : Vec<i128> = vec![];
    
    for line in fs::read_to_string("input").expect("No file\n").split("\n") {
        let pos: Vec<i128>  = line.split(" @ ").nth(0).unwrap().split(", ").map(|x| x.parse::<i128>().unwrap()).collect();
        let vel: Vec<i128> = line.split(" @ ").nth(1).unwrap().split(", ").map(|x| if x.chars().nth(0).unwrap() == '-' {-x[1..x.len()].parse::<i128>().unwrap()} else { x.parse::<i128>().unwrap()}).collect();

        x.push(pos[0]);
        y.push(pos[1]);
        vx.push(vel[0]);
        vy.push(vel[1]);
        //println!("{:?} - {:?}", pos, vel);
    }
    
    let mut sum = 0;

    let _min = 200000000000000.0 /*7.0*/;
    let _max = 400000000000000.0 /*27.0*/;   

    for i in 0..x.len() {
        for j in i+1..x.len() {
            
            let x_cross :f64 = ( x[i]*vy[i]*vx[j] - x[j]*vy[j]*vx[i] + (y[j]-y[i])*(vx[i]*vx[j]) ) as f64 /( vy[i]*vx[j]-vy[j]*vx[i] ) as f64;
            let t_cross_i :f64 = (x_cross-x[i] as f64)/vx[i] as f64 ;
            let y_cross :f64 = y[i] as f64 +vy[i] as f64 *t_cross_i ;
            
            let t_cross_j :f64 = (x_cross-x[j] as f64)/vx[j] as f64 ;

            // println!("\n{} {} | {} {} \t|| {} {} | {} {}", x[i],y[i],vx[i],vy[i], x[j],y[j],vx[j],vy[j]);
            // println!("{},{} at time {} {}", x_cross, y_cross, t_cross_i,t_cross_j);
            
            if t_cross_i>=0.0 && t_cross_j>=0.0 && x_cross >= _min && x_cross <= _max && y_cross >= _min  && y_cross<= _max  {
                sum+=1;
            }
        }
    }
    println!("result: {}", sum)
}