use std::{ffi::c_float, io};

fn  soma(a: f32, b: f32) -> f32 {
    a + b
}

fn subtrair(a: f32, b: f32) -> f32 {
    a - b
}

fn multiplicacao(a: f32, b: f32) -> f32 {
    a * b
}

fn divisao(a: f32, b: f32) -> Option<f32> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    
    println!("Calculadora Rust!");
    println!("Escolha uma operação!");

    let mut operacao = String::new();
    io::stdin()
    .read_line(&mut operacao)
    .expect("Falha ao ler a operação");

    let a: f32;
    let b: f32;

    println!("Digite o primeiro número:");
    let mut input_a = String::new();
    io::stdin()
    .read_line(&mut input_a)
    .expect("Falha ao ler a operação");

    println!("Digite o segundo número:");
    let mut input_b: String = String::new();
    io::stdin()
    .read_line(&mut input_b)
    .expect("Falha ao ler a operação");
    

   let a: f32 = input_a.trim().parse().expect("Entrada inválida");
   let b: f32 = input_b.trim().parse().expect("Entrada inválida");

   match operacao.trim() {
        "soma" => {
            let resultado = soma(a, b);
            println!("O resultado da soma é: {}", resultado);
        }
        "subtrair" => {
            let resultado = subtrair(a, b);
            println!("O resultado da subtração é: {}", resultado);
        }
        "multiplicacao" => {
            let resultado = multiplicacao(a, b);
            println!("O resultado da multiplicacao é: {}", resultado);
        }
        "divisao" => {
            match divisao(a, b) {
                Some(resultado) => println!("O resultado da divisão é: {}", resultado),
                None => println!("Erro: Divisão por zero não é permitida."),
            }
        }
        _ => println!("Operação inválida!"),
    }



}

