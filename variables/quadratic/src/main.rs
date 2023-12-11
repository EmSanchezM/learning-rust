/*
Escribir y comprobar un programa que resuelva la ecuación cuadrática (ax2 + bx + c = 0).

 Si Discriminante > 0, la ecuación tiene dos soluciones reales y distintas.
 Si Discriminante = 0, la ecuación tiene dos soluciones reales e iguales (raíces coincidentes).
 Si Discriminante < 0, la ecuación no tiene soluciones reales, las soluciones son números 

Casos de prueba: 
 - Discriminante > 0, a=1; b=-5; c=6;
 - Discriminante = 0, a=2; b=-4; c=2;
 - Discriminante < 0, a=3; b=6; c=10; 
*/

fn read() -> f64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    input.trim().parse().expect("Entrada no válida, ingrese un número válido")
}

fn main() {
    println!("Ingrese el coeficiente 'a': ");
    let a: f64 = read();

    println!("Ingrese el coeficiente 'b': ");
    let b: f64 = read();

    println!("Ingrese el coeficiente 'c': ");
    let c: f64 = read();
   
    let discriminant = b*b - 4.0*a*c;

    if discriminant > 0.0 {
        let discriminant_root = discriminant.sqrt();
        let x1 = (-b + discriminant_root) / (2.0 * a);
        let x2 = (-b - discriminant_root) / (2.0 * a);

        println!("Las raíces son: x1 = {} y x2 = {}", x1, x2);
    } else if discriminant == 0.0 {
        let x = -b / (2.0 * a);
        println!("La ecuación tiene una raíz doble en x = {}", x);
    } else {
        println!("La ecuación no tiene raíces reales");
    }
}
