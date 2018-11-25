use std::collections::HashMap;

fn main() {
    let ints = vec![1, 19, 5, 22, 11, 5];
    let mut ints: Vec<i32> = Vec::from(ints);
    ints.sort();

    let len = ints.len();

    if len > 0 {
        let mut mean: f64 = 0.0;
        for &i in &ints {
            mean += i as f64;
        }
        mean /= len as f64;

        let median = if len % 2 == 0 {
            (ints[len / 2] + ints[len / 2 - 1]) as f64 / 2.0
        } else {
            ints[(len + 1) / 2] as f64
        };

        let mut counts: HashMap<i32, i32> = HashMap::new();
        for &i in &ints {
            let n = counts.entry(i).or_insert(0);
            *n += 1;
        }

        let mut max = 0;
        let mut mode = ints[0];
        for (key, value) in &counts {
            if *value > max {
                max = *value;
                mode = *key;
            }
        }

        println!("The mean is {}!", mean);
        println!("The median is {}!", median);
        println!("The mode is {}!", mode);
    }
}
