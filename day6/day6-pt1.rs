fn main() {
    // let time = [7.0, 15.0, 30.0];
    // let distance = [9.0, 40.0, 200.0];
    let time = [48.0, 93.0, 84.0, 66.0];
    let distance = [261.0, 1192.0, 1019.0, 1063.0];

    let mut res = 1;
    for i in 0..=time.len()-1{
        // equation: x(time-x) = distance -> -x**2 + time *x - distance = 0 -> x**2 - time *x + distance = 0 
        let sol2 = ((0.5* (time[i] + (time[i]*time[i] - 4.0 * distance[i] as f32).sqrt() ))-0.00001).trunc() as u32;
        let sol1 =  (0.5* (time[i] - (time[i]*time[i] - 4.0 * distance[i] as f32).sqrt() )).trunc() as u32;

        println!("soluzioni: {} - {} = {}", sol2, sol1, sol2-sol1 );
        res*= sol2-sol1;
    }
    println!("res: {}", res );
}
