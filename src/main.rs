// функция double_int32 принимает 32-х битное целое знаковое число и возвращает 32-х битное целое знаковое число, равное удвоенному входному.
fn double_int32(a: i32) -> i32 {
    a * 2
}

// функция double_int64 принимает 32-х битное целое знаковое число и возвращает 64-х битное целое знаковое число, равное удвоенному входному.
fn double_int64(a: i32) -> i64 {
    (a * 2) as i64
}

// функция double_float32 принимает 32-х битное число с плавающей точкой и возвращает 32-х битное число с плавающей точкой, равное удвоенному входному.
fn double_float32(a: f32) -> f32 {
    a * 2.
}

// функция double_float64 принимает 32-х битное число с плавающей точкой и возвращает 64-х битное число с плавающей точкой, равное удвоенному входному.
fn double_float64(a: f32) -> f64 {
    (a * 2.) as f64
}

// функция int_plus_float_to_float принимает 32-х битное целое знаковое число и 32-х битное число с плавающей точкой. Возвращает 64-х битное число с плавающей точкой, равное сумме входных.
fn int_plus_float_to_float(a: i32, b: f32) -> f64 {
     (a as f32 + b) as f64
}

// функция int_plus_float_to_int принимает 32-х битное целое знаковое число и 32-х битное число с плавающей точкой. Возвращает 64-х битное целое знаковое число, равное сумме входных.
fn int_plus_float_to_int(a: i32, b: f32) -> i64 {
    (a as f32 + b) as i64
}

// функция tuple_sum принимает кортеж из двух целых чисел. Возвращает целое число, равное сумме чисел во входном кортеже.
fn tuple_sum(tup: (i32, i32)) -> i64 {
    (tup.0 + tup.1) as i64
}

// функция array_sum принимает массив из трёх целых чисел. Возвращает целое число, равное сумме чисел во входном массиве.
fn array_sum(arr: [i64; 3]) -> i64 {
    let sum: i64 = arr.iter().sum();
    return sum
}

fn main() {
    let t = (4, 7);
    let a:[i64; 3] = [2, 6, 1];

    println!("double_int32(5) result: {}", double_int32(5));
    println!("double_int64(7) result: {}", double_int64(7));
    println!("double_float32(3.6) result: {}", double_float32(3.6));
    println!("double_float64(5.8) result: {}", double_float64(5.8));
    println!("int_plus_float_to_float(2, 5.8) result: {}", int_plus_float_to_int(2, 5.8));
    println!("int_plus_float_to_int(2, 5.8) result: {}", int_plus_float_to_int(2, 5.8));
    println!("tuple_sum((4,7)) result: {}", tuple_sum(t));
    println!("array_sum([2,6,1]) result: {}", array_sum(a));
}