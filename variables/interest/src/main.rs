/*
Escribir un algoritmo que calcule cuanto le dara su banco después de realizar una imposición a plazo fijo,
para ello el programa debe de pedir la cantidad que desea invertir en el banco, el tipo de interés anual 
que le paga el banco por el dinero y el plazo que se mantiene la inversión.
El programa debe calcular el dinero que se obtiene después de dicho plazo. Recuerde que al pagarle los intereses
al banco le retendrá el 18% para institución tributaria.  
*/

fn read() -> f64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    input.trim().parse().expect("Entrada no válida, ingrese un número válido")
}

fn main() {
    const RETENTION: f64 = 0.18;
    const MONTHS_OF_THE_YEAR: u8 = 12;
    const MONTHS_OF_THE_YEAR_FORMAT: f64 = MONTHS_OF_THE_YEAR as f64;

    println!("Ingrese la cantidad que desea invertir: ");
    let invested_capital: f64 = read();

    println!("Ingrese el interés anual que le ofrece el banco: ");
    let annual_interest: f64 = read();

    println!("Ingrese el número de meses de inversión: ");
    let months_of_investment: f64 = read();

    let interest_earned: f64 = invested_capital * (annual_interest / 100.0)  * (months_of_investment / MONTHS_OF_THE_YEAR_FORMAT);
    let taxes: f64 = interest_earned * RETENTION;

    let interest_collected: f64 = interest_earned - taxes;

    println!("Calculo de interés: ");
    println!("Dinero a invertir: {}", invested_capital);
    println!("Interés anual: {}", annual_interest);
    println!("Interés a los {} meses: {}", months_of_investment, interest_earned);
    println!("Retención que se realiza: {}", taxes);
    println!("Intereses a cobrar: {}", interest_collected);
}
