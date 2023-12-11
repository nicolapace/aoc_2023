fn main() {
    // let time = [71530.0];
    // let distance = [940200.0];
    let time = [48938466.0];
    let distance = [261119210191063.0];

    let mut res = 1;
    for i in 0..=time.len()-1{
        // equation: x(time-x) = distance -> -x**2 + time *x - distance = 0 -> x**2 - time *x + distance = 0 
        let sol2 = ((0.5* (time[i] + (time[i]*time[i] - 4.0 * distance[i] as f64).sqrt() ))-0.0000001).trunc() as u32;
        let sol1 =  (0.5* (time[i] - (time[i]*time[i] - 4.0 * distance[i] as f64).sqrt() )).trunc() as u32;

        println!("soluzioni: {} - {} = {}", sol2, sol1, sol2-sol1 );
        res*= sol2-sol1;
    }
    println!("res: {}", res );
}
