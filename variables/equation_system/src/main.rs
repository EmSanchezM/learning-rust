/*
Escriba un programa que lea los coeficientes a, b, c , d, e, f de un sistema lineal de dos ecuaciones con dos incógnitas y
muestre la solución.
Ecuación 1: ax+by=c
Ecuación 2: dx+ey=f

Teoria o conceptos:
Un sistema de ecuaciones puede ser: 
    - Compatible
    - Incompatible

Un sistema de ecuaciones compatible puede ser de dos tipos:
    - Determinado si tiene solución única
    - Indeterminado si tiene mas de una

Un sistema de ecuaciones incompatible si no tiene solución.

La fórmula de Sarrus para determinar si un sistema determinado tiene solución única:
    det=ae−bd
donde det es el determinante del sistema;
Si determinante es distinto de cero tiene solución única.

Regla de crammer:
Solucion 1: x=determinante(ecuacion1)/determinante del sistema
Solución 2: y=determinante(ecuacion2)/determinante del sistema

Caso de prueba:
a=2; b=3; c=5; d=1; e=6; f=7;
Solución: x=1; y=1;
*/

fn read() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    input.trim().parse().expect("Entrada no válida, ingrese un número válido")
}

fn main() {
    println!("Ingrese el valor de 'a': ");
    let a: i32 = read();

    println!("Ingrese el valor de 'b': ");
    let b: i32 = read();

    println!("Ingrese el valor de 'c': ");
    let c: i32 = read();

    println!("Ingrese el valor de 'd': ");
    let d: i32 = read();

    println!("Ingrese el valor de 'e': ");
    let e: i32 = read();

    println!("Ingrese el valor de 'f': ");
    let f: i32 = read();

    let sarrus_rule = (a*e)-(b*d);

    if sarrus_rule != 0 {

        let determinant_equation1: i32 = c*e-b*f;
        let determinant_equation2: i32 = a*f-c*d;

        let x: i32 = determinant_equation1 / sarrus_rule;
        let y: i32 = determinant_equation2 / sarrus_rule;

        println!("Las soluciones son: x = {} y y = {}", x, y);
        
    } else {
        println!("El sistema de ecuaciones indeterminado no tiene soluciones unicas");
    }
}
