 #[derive(Debug)]
 struct Foo {
     a : String
 }

 #[derive(Debug)]
 struct Bar {
     a : i32
 }

 #[derive(Debug)]
 enum FooBar {
     foo(Foo),
     bar(Bar)
 }

 impl FooBar {
     fn call(&self) {
         match self {
             FooBar::foo(f) => { println!("Foo => {}", f.a); }
             FooBar::bar(b) => { println!("Bar => {}", b.a); }
         }
     }
 }

 enum Coin {
     Penny,
     Nickel,
     Dime,
     Quater
 }

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
 }

pub fn run() {
    println!("Running chapter 6: Enum");
    let f = FooBar::foo(Foo{a:String::from("Foo")});
    println!("{:#?}", f);

    let nickel = Coin::Nickel;
    println!("{}", nickel.value());
    println!("{}", nickel.gt5());
    let dime = Coin::Dime;
    println!("{}", dime.gt5());
}
