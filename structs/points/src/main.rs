/*
Un punto en el plano se puede representar mediante una estructura con dos campos. Escribir un programa que realice las
siguientes operaciones con puntos en el plano.
• Dados dos puntos calcular la distancia entre ellos.
• Dados dos puntos determinar el punto medio de la línea que los une.

Caso de prueba:
Punto 1: x=-3; y=5;
Punto 2: x=-7; y=1;
Distancia: 5.65
Punto medio: (-5, 3)

Punto 1: x=-4; y=-3;
Punto 2: x=2; y=3;
Distancia: 8.48
Punto medio: (-5, 3)
*/

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other_point: &Point) -> f64 {
        let point1 = f64::powf(other_point.x - self.x, 2.0);
        let point2 = f64::powf(other_point.y - self.y, 2.0);
        let sum = point1 + point2;

        sum.sqrt()
    }

    fn middle_point(&self, other_point: &Point) -> Point {
        let point1 = (self.x + other_point.x) / 2.0;
        let point2 = (self.y + other_point.y) / 2.0;

        Point {
            x: point1,
            y: point2,
        }
    }
}

fn main() {
    let point1 = Point { x: -3.0, y: 5.0 };

    let point2 = Point { x: -7.0, y: 1.0 };

    let middle_point = point1.middle_point(&point2);

    let point3 = Point { x: -4.0, y: -3.0 };

    let point4 = Point { x: 2.0, y: 3.0 };

    let middle_point2 = point3.middle_point(&point4);

    println!(
        "La distancia entre ({}, {}) y ({}, {}) es: {} ",
        point1.x,
        point1.y,
        point2.x,
        point2.y,
        point1.distance(&point2)
    );
    println!(
        "El punto medio es: ({}, {})",
        middle_point.x, middle_point.y
    );

    println!(
        "La distancia entre ({}, {}) y ({}, {}) es: {} ",
        point3.x,
        point3.y,
        point4.x,
        point4.y,
        point3.distance(&point4)
    );
    println!(
        "El punto medio es: ({}, {})",
        middle_point2.x, middle_point2.y
    );
}
