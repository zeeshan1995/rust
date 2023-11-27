
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

fn chapter6_enum() {

    #[derive(Debug)]
    struct Foo {
        a : String
    };

    #[derive(Debug)]
    struct Bar {
        a : i32
    };

    #[derive(Debug)]
    enum FooBar {
        foo(Foo),
        bar(Bar)
    };

    let f = FooBar::foo(Foo{a:String::from("Foo")});
    println!("{:#?}", f);

    impl FooBar {
        fn call(&self) {
            match self {
                FooBar::foo(f) => { println!("Foo => {}", f.a); }
                FooBar::bar(b) => { println!("Bar => {}", b.a); }
            }
        }
    };

    f.call();


    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quater
    };

    impl Coin {
        fn value(&self) -> i32 {
            match self {
                Coin::Penny => 1,
                Coin::Nickel=> 5,
                Coin::Dime => 10,
                Coin::Quater => 25,
            }
        }

        fn gt5(&self) -> bool {
            match self {
                Coin::Penny | Coin::Nickel => false,
                _ => true,
            }
        }
    };

    let nickel = Coin::Nickel;
    println!("{}", nickel.value());
    println!("{}", nickel.gt5());
    let dime = Coin::Dime;
    println!("{}", dime.gt5());
}
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
    chapter6_enum();
}
