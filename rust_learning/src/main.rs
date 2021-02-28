fn main() {
    // println!("Hello, world!");
    let numbers = vec![1,2,3,4,5,6,7,8,9,10].into_iter();
    let evens = numbers.filter(|x| *x % 2 ==0);
    let evens_squares = evens.clone().map(|x|x * x);
    let result = evens_squares.clone().collect::<Vec<_>>();
    println!("{:?}",result);
    println!("{:?}\n{:?}",evens,evens_squares);
}
