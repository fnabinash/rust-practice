// Implement a function that returns the nth Fibonacci number.

fn main() {
    let num: u32 = 15;

    if num == 1 {
        println!("1st fobonacci number is 0.");
        return;
    }

    let fibo: u32 = cal_fibo(num);
    println!("The {}th fibonacci number is {}.", num, fibo);
}

fn cal_fibo(num: u32) -> u32 {
    let mut fibo1: u32 = 0;
    let mut fibo2: u32 = 1;
    let mut fibo: u32 = 1;

    for _ in 3..num {
        fibo += fibo2;
        fibo2 += fibo1;
        fibo1 += fibo - fibo2 - fibo1;
    }

    fibo
}
