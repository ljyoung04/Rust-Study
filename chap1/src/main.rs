use std::io;

fn main() {
    println!("간단한 덧셈 프로그램");

    let mut a = String::new();
    let mut b = String::new();

    println!("A : ");
    io::stdin().read_line(&mut a).expect("fail to read line");

    println!("B : ");
    io::stdin().read_line(&mut b).expect("fail to read line");

    let sum : i32 = a.trim().parse::<i32>().unwrap() + b.trim().parse::<i32>().unwrap();

    println!("{a} + {b} = {sum}");
}

/* 

개선할 점

숫자가 아닌 경우에 대한 에러를 처리할 수 있도록 해야한다.
print!로 할 경우 라인 버퍼라 출력이 안된다. 따로 flush를 해줘야함

*/
