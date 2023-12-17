/*
Escribir un programa que lea y escriba la información de 10 clientes de una determinada empresa. Los clientes tienen un
nombre, el número de unidades solicitadas, el precio de cada unidad y el estado en que se encuentra: moroso, atrasado,
pagado.
Añadir al programa una función que permita mostrar la factura de todos los clientes de cada una de las categorías moroso, atrasado o pagado.
CONCEPTOS:
Definicion de structs simples con tipos de datos genericos.
TODO:
Aún no aplicamos el concepto de impl
*/

struct Customer {
    name: String,
    number_of_units_ordered: i64,
    price_per_unit: f64,
    status: String,
}

fn read_i64() -> i64 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    input
        .trim()
        .parse()
        .expect("Entrada no válida, ingrese un número válido")
}

fn read_f64() -> f64 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    input
        .trim()
        .parse()
        .expect("Entrada no válida, ingrese un número válido")
}

fn read() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    input.trim().to_string()
}

fn display_customers(customers: &Vec<Customer>, status: &str) {
    let customers_by_status: Vec<&Customer> = customers
        .iter()
        .filter(|customer| customer.status == status)
        .collect();

    println!("Clientes {}", status);
    for customer_by_status in &customers_by_status {
        println!(
            "Nombre: {} ------------- Estado: {}",
            customer_by_status.name, customer_by_status.status
        );
        println!(
            "Unidades ordenadas: {} ------------- Precio por unidad: {}",
            customer_by_status.number_of_units_ordered, customer_by_status.price_per_unit
        );
    }
    println!("------------------------------------------------------");
}

fn create_customer(customers: &mut Vec<Customer>) {
    println!("Ingrese el nombre del cliente: ");
    let name = read();

    println!("Ingrese el número de unidades ordenadas: ");
    let number_of_units_ordered = read_i64();

    println!("Ingrese el precio por unidad ordenada: ");
    let price_per_unit = read_f64();

    println!("Ingrese el estado del cliente (MOROSO, PAGADO, ATRASADO): ");
    let status = read();

    customers.push(Customer {
        name,
        number_of_units_ordered,
        price_per_unit,
        status,
    })
}

fn main() {
    let mut customers: Vec<Customer> = Vec::new();

    customers.push(Customer {
        name: String::from("Daniela castillo"),
        number_of_units_ordered: 50,
        price_per_unit: 150.0,
        status: String::from("MOROSO"),
    });

    customers.push(Customer {
        name: String::from("Maria Lopez"),
        number_of_units_ordered: 30,
        price_per_unit: 250.0,
        status: String::from("MOROSO"),
    });

    customers.push(Customer {
        name: String::from("Ana Pacheco"),
        number_of_units_ordered: 10,
        price_per_unit: 350.0,
        status: String::from("ATRASADO"),
    });

    customers.push(Customer {
        name: String::from("Joselin Paz"),
        number_of_units_ordered: 10,
        price_per_unit: 350.0,
        status: String::from("ATRASADO"),
    });

    customers.push(Customer {
        name: String::from("Mariela Casco"),
        number_of_units_ordered: 45,
        price_per_unit: 350.0,
        status: String::from("PAGADO"),
    });

    customers.push(Customer {
        name: String::from("Melisa Valladares"),
        number_of_units_ordered: 45,
        price_per_unit: 350.0,
        status: String::from("PAGADO"),
    });

    loop {
        println!("--- Menú ---");
        println!("1. Agregar un nuevo cliente");
        println!("2. Listar clientes morosos");
        println!("3. Listar clientes pagados");
        println!("4. Listar clientes atrasados");
        println!("5. Salir");

        println!("Ingrese el número de la opción deseada:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");
        let option = input.trim();

        match option {
            "1" => create_customer(&mut customers),
            "2" => display_customers(&customers, "MOROSO"),
            "3" => display_customers(&customers, "PAGADO"),
            "4" => display_customers(&customers, "ATRASADO"),
            "5" => {
                println!("Saliendo del programa. ¡Hasta luego!");
                break;
            }
            _ => println!("Opción inválida. Por favor, ingrese un número válido."),
        }
    }
}
