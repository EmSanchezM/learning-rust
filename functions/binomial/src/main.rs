/*
Dado un número real p entre cero y uno, un número entero n positivo, y otro número entero i comprendido entre 0 y n, se
sabe que si un suceso tiene probabilidad de que ocurra p, y el experimento aleatorio se repite n veces, la probabilidad de
que el suceso ocurra i veces viene dado por la función binomial de parámetros n, p e i dada por la siguiente fórmula:

Probabilidad(X=i) = combinatoria(n,i) * p^i * (1-p)^n-i; Donde:
P(X=i) es la probabilidad de obtener exactamente i éxitos en n intentos.

Escribir un programa que lea los valores de p, n e i, y calcule el valor dado por la función binomial.

Caso de prueba:
Caso 1: Lanzamiento de una moneda
Supongamos que lanzamos una moneda justa 5 veces y queremos encontrar la probabilidad de obtener exactamente 3 caras.

Experimento binomial: Cada lanzamiento tiene dos resultados posibles: cara (éxito) o cruz (fracaso).
Número de intentos: 5 lanzamientos.
Probabilidad de éxito: La probabilidad de obtener una cara en una moneda justa es p=0.5.

n=5; success_probability=0.5; number_of_successes=3;

Caso 2: Fabricación de productos
Una fábrica produce discos duros y sabe que el 2% de los discos duros son defectuosos. 
Si se selecciona al azar un lote de 100 discos duros, ¿cuál es la probabilidad de que exactamente 3 de ellos sean defectuosos?

Experimento binomial: Cada disco duro tiene dos resultados posibles: defectuoso (éxito) o no defectuoso (fracaso).
Número de intentos: Se seleccionan 100 discos duros.
Probabilidad de éxito: La probabilidad de que un disco duro sea defectuoso es p=0.02.

n=100; success_probability=0.02; number_of_successes=3;
*/

fn read() -> f64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    input.trim().parse().expect("Entrada no válida, ingrese un número válido")
}

fn pow(base: f64, exponent: f64) -> f64 {
    let mut result: f64 = 1.0;
    let iterator = exponent as u64;

    for _ in 0..iterator {
        result *= base;
    }

    result
}

fn factorial(n: f64) -> f64 {
    if n <= 1.0 {
        1.0
    } else {
        n * factorial(n - 1.0)
    }
}

fn combinatoria(m: f64, n: f64) -> f64 {
    let numerator = factorial(m);
    let denominator = factorial(n) * factorial(m-n);

    let combinatoria: f64 = numerator / denominator;

    combinatoria
}

fn binomial(n: f64, success_probability: f64, number_of_successes: f64) -> f64 {
    let p1 = pow(success_probability, number_of_successes);

    let aux_exponent = n - number_of_successes;
    let p2 = pow(1.0-success_probability, aux_exponent);

    let p3 = p1 * p2;

    let probability = combinatoria(n, number_of_successes) * p3;

    probability
}

fn main() {
    println!("Ingrese el número de intentos 'n': ");
    let n: f64 = read();

    println!("Ingrese la probabilidad de éxito: ");
    let success_probability: f64 = read();

    println!("Ingrese el número de éxitos: ");
    let number_of_successes: f64 = read();
    
    let result = binomial(n, success_probability, number_of_successes);

    let probability = result * 100.0;

    println!("La probabilidad de obtener exactamente {} exitos en {} intentos es: {} % ", number_of_successes, n, probability);
}
