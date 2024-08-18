mod rock_paper_scissors;
mod fibonnaci;

#[derive(Debug)]
struct Integer  {
    value:i32
}

fn main() {


    rock_paper_scissors::play();

    let n:Integer = Integer { value:10 };
    let fibo = fibonnaci::recursive(n);

    println!("{:?}",n);
    println!("{}", fibo);

}