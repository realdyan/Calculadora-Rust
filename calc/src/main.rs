
fn  somar(a: i32, b: i32) -> i32 {
    a + b
}

fn subtrair(a: i32, b: i32) -> i32 {
    a - b
}

fn multiplicar(a: i32, b: i32) -> i32 {
    a * b
}

fn dividir(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
// x
fn main() {
    println!("Calculadora Rust!");
    let a = 10; // Escreva o primeiro valor
    let b = 5; // Escreva o segundo valor
    println!("Soma: {}", somar(a, b)); // Estamos usando a função somar


}

