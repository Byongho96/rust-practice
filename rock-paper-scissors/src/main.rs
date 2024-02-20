use std::io; // 활용하고자 하는 패키지

fn main() {
    println!("[가위,바위,보] 중 하나를 입력해주세요!");

    // 변수 선언 & 초기화
    let mut decision = String::new();

    // 사용자 입력 & 실패 시 대체값
    io::stdin().read_line(&mut decision).expect("입력실패");

    // 변수 출력
    println!("당신의 선택: {decision}");
}
