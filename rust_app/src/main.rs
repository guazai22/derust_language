fn main() {
    let num = fib(26);
    println!("fib(26) ->   {num:?}",);
    println!("fibs(26) ->   {:?}", fibs(26,),);
}
fn fib(n: u32) -> u128 {
    match n {
        | 1 => 1,
        | 2 => 1,
        | _ => fib(n - 1) + fib(n - 2),
    }
}
fn fibs(n: u32) -> Vec<u128> {
    let mut res = vec![1, 1];
    {
        let mut i = 0;
        while (i < (n - 2)) {
            i = i + 1;
            res.push(res[res.len() - 1] + res[res.len() - 2]);
        }
    }
    res
}
