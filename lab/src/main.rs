
use std::io;

fn chapter3_comman_programming_concepts() {
    println!("Running chapter1");


    fn fib(n :i32) -> i32 {
        if n == 1 {
            return 0;
        }
        else if n == 2 {
            return 1;
        }
        let mut a = 0;
        let mut b = 1;
        for _ in 3..n+1 {
            let c = a+b;
            a = b;
            b = c;
        }
        b
    }

    fn fib_rec(n: usize, nums: &mut Vec<i32>) -> i32 {
        if n<=2 {
            return n as i32 - 1;
        }

        if nums[n-1usize] == 0 {
            nums[n-1usize] = fib_rec(n-1usize, nums) + fib_rec(n-2usize, nums);
        }
        return nums[n-1usize];
    }

    println!("{}", fib(10));

    let mut nums :Vec<i32> = vec![0;10];
    println!("{}", fib_rec(10, &mut nums));

    for n in &nums {
        println!("{}", n);
    }
}
fn main() {
    println!("Hello, world!");

    let a = 255u8;
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");

    let b: u8 = b.trim().parse().expect("Invalid input");

    //let c = a+b;// panic
    let c = a.wrapping_add(b);
    
    println!("a+b = {c}");

    chapter3_comman_programming_concepts();
}
