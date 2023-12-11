/*
Escribir un algoritmo que lea un número n>0 del teclado y sume la serie siguiente:
Sumatoria de ai con i=1 hasta n donde
ai es igual a la multiplicacion de i * i si i es impar
ai es igual a 2 si i es par

Caso de prueba 
n = 5; 
i=1; suma=2; 
i=2; suma=2+(2*2)=6; 
i=3; suma=6+2=8; 
i=4; suma=8+(4*4)=8+16=24; 
i=5; suma=24+2=26;
*/

fn read() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    input.trim().parse().expect("Entrada no válida, ingrese un número válido")
}

fn main() {
    println!("Ingrese el valor de 'n': ");
    let n: u64 = read();

    let mut i: u64 = 1;
    let mut summation: u64 = 0;

    while i <= n {
        if i % 2 == 0 {
            summation += i*i;
        } else {
            summation += 2;
        }

        i+= 1;
    }
    
    println!("El resultado de la sumatoria es: {}", summation);
}
