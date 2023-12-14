/*
Creame un algoritmo que ordena una lista de 20 elementos desordenados introducida en un listay y
posteriormente la imprime (o visualiza) en pantalla.

Caso de prueba:

lista_sin_orden = { 30, 35, 38, 58, 14, 15, 50, 27, 10, 20, 12, 85, 49, 65, 86, 60, 25, 90, 5, 16 }
lista_ordenada = { 5, 10, 12, 14, 15, 16, 20, 25, 27, 30, 35, 38, 49, 50, 58, 60, 65, 85, 86, 90 }
*/

fn sorting_by_exchange(list: &mut[i64]) {
    let n = list.len();

    for i in 0..n {
        for j in 0..n - 1 - i {
            if list[j] > list[j + 1] {
                let temp = list[j];
                list[j] = list[j + 1];
                list[j + 1] = temp;
            }
        }
    }
}

fn main() {
    let mut list = [30, 35, 38, 58, 14, 15, 50, 27, 10, 20, 12, 85, 49, 65, 86, 60, 25, 90, 5, 16];

    println!("Lista desordenada: {:?}", list);

    sorting_by_exchange(&mut list);

    println!("Lista ordenada: {:?}", list);
}
