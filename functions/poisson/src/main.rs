/*
La ley de probabilidad de que ocurra el suceso r veces de la distribución de Poisson de media m viene dado por:
Probabilidad(X=r) = (m^r / r!) * e^-m donde:
P(x=r) es la probabilidad de que ocurran r eventos.

Escribir un algoritmo que calcule mediante un menú el valor de:
a) El suceso ocurra exactamente r veces.
b) El suceso ocurra a lo sumo r veces.
c) El suceso ocurra por lo menos r veces.

Caso de prueba:
Caso 1: Procesos de llegada en un servicio
Supongamos que en promedio llegan 5 clientes por hora a un servicio al cliente. 
¿Cuál es la probabilidad de que exactamente 3 clientes lleguen en el próximo intervalo de una hora?
Distribución de Poisson: La tasa promedio de llegada de clientes es 5 por hora.
Número de eventos: 3 clientes.

m=5; r=3; probility=14.04%

Caso 2: Accidentes de tráfico
Supongamos que en promedio hay 2 accidentes de tráfico por día en una intersección particular. 
¿Cuál es la probabilidad de que no ocurra ningún accidente en un día específico?
Distribución de Poisson: La tasa promedio de accidentes es 2 por día.
Número de eventos: 0 accidentes.

m=2; r=0; probility=13.53%
*/

fn read() -> f64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    input.trim().parse().expect("Entrada no válida, ingrese un número válido")
}

fn factorial(n: f64) -> f64 {
    if n <= 1.0 {
        1.0
    } else {
        n * factorial(n - 1.0)
    }
}

fn poisson(m: f64, r:f64) -> f64 {
    let p1 = f64::powf(m,r);
    
    let euler_constant = std::f64::consts::E;
    let p2 = f64::powf(euler_constant, -m);
    
    let probability = (p1 * p2) / factorial(r);

    probability
}

fn main() {
    println!("Ingrese la tasa promedio 'm': ");
    let m: f64 = read();

    println!("Ingrese el numero de eventos 'r': ");
    let r: f64 = read();

    let result = poisson(m, r);

    let probability = result * 100.0;

    println!("La probabilidad de obtener {} exitos en {} intentos es: {} % ", m, r, probability);
}
