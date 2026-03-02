use std::io;

fn main() {
    println!("섭씨 화씨 변환 프로그램");

    loop {
        let mut line = String::new();

        menu();

        io::stdin()
            .read_line(&mut line)
            .expect("Fail to read line");

        let mut parts = line.split_whitespace();

        let Some(mode_str) = parts.next() else {
            println!("유효하지 않은 옵션입니다.");
            continue;
        };

        let Ok(mode) = mode_str.parse::<u32>() else {
            println!("숫자를 입력하세요.");
            continue;
        };

        if mode == 3 {
            println!("프로그램 종료");
            break;
        }

        let Some(temp_str) = parts.next() else {
            println!("온도를 입력하세요.");
            continue;
        };

        let Ok(temp) = temp_str.parse::<f64>() else {
            println!("올바른 숫자를 입력하세요.");
            continue;
        };

        match mode {
            1 => {
                let c = (temp - 32.0) * 5.0 / 9.0;
                println!("{temp}°F → {c:.2}°C");
            }
            2 => {
                let f = temp * 9.0 / 5.0 + 32.0;
                println!("{temp}°C → {f:.2}°F");
            }
            _ => println!("1, 2, 3 중에서 선택하세요."),
        }

        println!();
    }
}

fn menu() {
    println!("1. 화씨 -> 섭씨");
    println!("2. 섭씨 -> 화씨");
    println!("3. 종료");
    println!("입력 예시 : 2 36.5");
}