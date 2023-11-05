pub fn alph(num: i32) -> String {
    let mut num = num;
    let mut res = Vec::<char>::new();
    while num > 0 {
        num = num / 26;
        let modnum = num % 26;
        res.push((modnum + 65) as u8 as char);
    }
    res.iter().rev().collect::<String>()
}

// N个同学围成一圈，第一个人从1开始报数，
pub fn circle(m: i32, n: i32) -> i32 {
    let mut res = vec![];
    for i in 1..=n {
        res.push(i);
    }
    let (mut i, mut cir) = (0, m-1);
    loop {
        cir -= 1;
        i = (i+1) % res.len();
        // println!("len {}", res.len());
        if cir == 0 {
            println!("{}", res[i]);
            res.remove(i);
            cir = m-1;
        }
        if res.len() == 1 {
            break;
        }
    }
    if res.len() == 1 {
        return res[0];
    } 
    -1
}

fn main() {
    let n = 17;
    let m = 3;
    println!("{}", circle(m, n));
}