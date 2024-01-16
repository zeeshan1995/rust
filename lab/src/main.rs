
//use std::io;

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


fn chapter4_understanding_ownership() {
    println!("Running Chapter 4");

    //write a function that takes a string of words separated by spaces and
    //returns the first word it finds in that string.
    //If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
    //fn first_word(s: &String) -> &str {
    fn first_word(s: &str) -> &str { // deref coercions
        let index = s.find(' ');
        if index.is_some() { &s[..index.unwrap()] } else { s }
    }

    let s = String::from("Apple ball cat");
    //let s = String::from("Appleballcat");
    let x = first_word(&s);
    println!("{}", x);
}

fn chapter5_struct() {
    println!("Running Chapter 5");
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }

    let user1 = User {
        active: true,
        username: String::from("User1"),
        email: String::from("user1@rust"),
        sign_in_count: 1
    };

    //println!("{user1}");
    println!("{}", user1.username);

    //struct update syntax. Data members gets moved(not copied).
    let user2 = User {
        username: String::from("User2"),
        ..user1
    };
    println!("{}", user1.username);
    //println!("{}", user1.email); //Error value has been moved
    //

    //let’s write a program that calculates the area of a rectangle.

    #[derive(Debug)]
    struct Rectangle {
        length: u64,
        breadth: u64
    }

    impl Rectangle {
        fn area(& self) -> u64 {
            self.length * self.breadth
        }
    }

    let rec = Rectangle{
        length: 100,
        breadth: 50
    };

    println!("{:#?} | Area -> {}", rec, rec.area());
}

mod chapter6_enum;
mod chapter7_collections;
fn main() {
    /*
    println!("Hello, world!");

    let a = 255u8;
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");

    let b: u8 = b.trim().parse().expect("Invalid input");

    //let c = a+b;// panic
    let c = a.wrapping_add(b);
    
    println!("a+b = {c}");
*/
    chapter3_comman_programming_concepts();
    chapter4_understanding_ownership();
    chapter5_struct();
    println!("\n*************************************************************************\n");

    use crate::chapter6_enum;
    chapter6_enum::run();

    println!("\n***********************| Chapter 7: Collections |****************************\n");

    use crate::chapter7_collections;
    chapter7_collections::run();
}
