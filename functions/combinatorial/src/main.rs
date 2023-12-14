/*
Se define el número combinatorio m sobre n de la siguiente forma:
(m,n) = m!/n!(m-n)!
Escribir un programa que lea los valores de m y de n y calcule el valor de m sobre n.

Casos de prueba:
Caso 1:
    Calcular el número de formas diferentes de elegir un equipo de 3 personas de un grupo de 7. m=7; n=3;

Caso 2:
    Calcular el número de formas en las que 2 genes específicos pueden combinarse en la descendencia si hay 4 genes posibles. m=4; n=2;
*/

fn read() -> i64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    input.trim().parse().expect("Entrada no válida, ingrese un número válido")
}

fn factorial(n: i64) -> i64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn combinatorial(m: i64, n: i64) -> i64 {
    let numerator = factorial(m);
    let denominator = factorial(n) * factorial(m-n);

    let result: i64 = numerator / denominator;

    result
}

fn main() {
    println!("Ingrese el valor de 'm': ");
    let m: i64 = read();

    println!("Ingrese el valor de 'n': ");
    let n: i64 = read();
    
    let result = combinatorial(m, n);

    println!("El número combinatorio m sobre n es: {}", result);
}
