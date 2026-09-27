use std::{format, print, println, vec};

fn main() {
    let s = 1111;
    println!("{:?}", magic_square(s));
}

fn magic_square(n: u32) -> Vec<Vec<u32>> {
    let n = n as usize;
    let mut list = vec![vec![0; n]; n];

    list[0][n / 2] = 1;
    let mut point = (0, n / 2);

    for i in 2..=(n.pow(2)) as u32 {
        if point.0 == 0 {
            if point.1 == n - 1 {
                if list[n-1][0] != 0 {
                    point.0 += 1;

                    list[point.0][point.1] = i;
                    continue;
                } else {
                    point.0 = n-1;
                    point.1 = 0;

                    list[point.0][point.1] = i;
                    continue;
                }
            }
            point.0 = n - 1;
            point.1 += 1;

            list[point.0][point.1] = i;
            continue;
        }

        if point.1 == n - 1 {
            point.0 -= 1;
            point.1 = 0;

            list[point.0][point.1] = i;
            continue;
        }

        if list[point.0 - 1][point.1 + 1] != 0 {
            point.0 += 1;

            list[point.0][point.1] = i;
            continue;
        }

        point.0 -= 1;
        point.1 += 1;

        list[point.0][point.1] = i;
    }

    for i in &list {
        for j in i {
            print!("{j} ")
        }
        println!("");
    }
    
    list
}