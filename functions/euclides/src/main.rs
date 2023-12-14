/*
Diseñar un algoritmo que calcule el máximo común divisor de dos números mediante el algoritmo de Euclides.

Teoria:
Sean los dos números A y B. El método para hallar el máximo común divisor (mcd) de dos números A y B por el método
de Euclides es:
1. Dividir el número mayor (A) por el menor (B). Si el resto de la división es cero, el número B es el máximo común
divisor.
2. Si la división no es exacta, se divide el número menor (B) por el resto de la división anterior.
3. Se siguen los pasos anteriores hasta obtener un resto cero. El último divisor es el mcd buscado.

Caso de prueba
a=10; b=25; MCD=5;
*/

fn read() -> i64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    input.trim().parse().expect("Entrada no válida, ingrese un número válido")
}

fn euclides(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn euclides_subtraction_method(mut a: i64, mut b: i64) -> i64 {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
}

fn main() {
    println!("Ingrese el valor de 'a': ");
    let a: i64 = read();

    println!("Ingrese el valor de 'b': ");
    let b: i64 = read();

    let mcd = euclides(a, b);
    let mcd_subtraction_method = euclides_subtraction_method(a, b);

    println!("El MCD de {} y {} es: {}", a, b, mcd);
    println!("Por método de restas el MCD de {} y {} es: {}", a, b, mcd_subtraction_method);
}
