fn main() {
  let_variable();
  let_mut_variable();
  const_variable();
  variable_shadowing();
}

// let 변수 선언
fn let_variable() {
  let x = 3;
  println!("x의 값은 {x}입니다"); // x의 값은 3입니다

  // x = 7;  // Error
  // println!("x의 값은 {x}입니다");
}

// let mut 변수 선언
fn let_mut_variable() {
  let mut x = 3;
  println!("x의 값은 {x}입니다"); // x의 값은 3입니다

  x = 7;  
  println!("x의 값은 {x}입니다"); // x의 값은 7입니다
}

// 상수 선언
fn const_variable() {
  const PI: f32 = 3.141592; // 타입 표기

  println!("PI상수값은 {PI}입니다"); // PI상수값은 3.141592입니다
}


// 변수 shadowing
fn variable_shadowing() {
  let x = 3;
  println!("x의 값은 {x}입니다"); // x의 값은 3입니다

  let x = x + 1;
  println!("x의 값은 {x}입니다"); // x의 값은 4입니다

  {
    let x = x * 2;
    println!("안쪽 범위에서 x의 값은 {x}입니다"); // 안쪽 범위에서 x의 값은 8입니다
  } 

  println!("x의 값은 {x}입니다"); // x의 값은 4입니다
}
