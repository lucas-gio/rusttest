use rayon::prelude::*;
use std::io::{self, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    loop {
        print_menu();
        let choice = get_user_input("Ingresa tu opción: ");

        match choice.trim().parse::<u32>() {
            Ok(1) => sieve_of_eratosthenes(),
            Ok(2) => calculate_pi(),
            Ok(3) => {
                println!("Saliendo...");
                break;
            }
            _ => println!("Opción no válida. Por favor, intenta de nuevo."),
        }
        println!(); // Añade una línea en blanco para separar las iteraciones del menú
    }
}

fn print_menu() {
    println!("===== MENÚ =====");
    println!("1. Criba de Eratóstenes");
    println!("2. Calcular decimales de Pi");
    println!("3. Salir");
    println!("===============");
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");
    input
}

fn sieve_of_eratosthenes() {
    let start = Instant::now();
    println!("\n--- Criba de Eratóstenes ---");
    let limit_str = get_user_input("Ingresa el número límite para encontrar primos: ");
    let limit = match limit_str.trim().parse::<usize>() {
        Ok(num) if num > 1 => num,
        _ => {
            println!("Límite no válido. Debe ser un número mayor que 1.");
            return;
        }
    };

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut last_update = Instant::now();

    for p in 2..=(limit as f64).sqrt() as usize {
        if last_update.elapsed() >= Duration::from_secs(1) {
            print!("\rProcesando hasta el número {}...", p);
            io::stdout().flush().unwrap();
            last_update = Instant::now();
        }

        if is_prime[p] {
            for i in (p * p..=limit).step_by(p) {
                is_prime[i] = false;
            }
        }
    }
    
    print!("\rProcesamiento completado.{}", " ".repeat(30)); // Limpia la línea de progreso
    println!();


    println!("\nNúmeros primos hasta {}:", limit);
    for (num, prime) in is_prime.iter().enumerate() {
        if *prime {
            print!("{} ", num);
        }
    }
    println!();
    let duration = start.elapsed();
    println!("La operación tomó: {:?}", duration);
}

fn calculate_pi() {
    let start = Instant::now();
    println!("\n--- Calcular Decimales de Pi (Versión Paralela) ---");
    println!("Esta es una implementación simple usando la fórmula de Leibniz.");
    println!("Es computacionalmente intensiva y converge lentamente.");

    let iterations_str =
        get_user_input("Ingresa el número de iteraciones (más iteraciones = más precisión): ");
    let iterations = match iterations_str.trim().parse::<u64>() {
        Ok(num) => num,
        _ => {
            println!("Número de iteraciones no válido.");
            return;
        }
    };

    let processed_count = Arc::new(AtomicU64::new(0));
    let progress_counter = Arc::clone(&processed_count);

    let progress_thread = thread::spawn(move || {
        while progress_counter.load(Ordering::Relaxed) < iterations {
            let completed = progress_counter.load(Ordering::Relaxed);
            let percentage = (completed as f64 / iterations as f64) * 100.0;
            print!("\rProgreso: {:.2}%", percentage);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        print!("\rProgreso: 100.00%");
        io::stdout().flush().unwrap();
    });

    let pi: f64 = (0..iterations)
        .into_par_iter()
        .map(|i| {
            processed_count.fetch_add(1, Ordering::Relaxed);
            let term = 4.0 / (2.0 * i as f64 + 1.0);
            if i % 2 == 0 {
                term
            } else {
                -term
            }
        })
        .sum();
    
    progress_thread.join().unwrap();
    println!(); // Nueva línea después de la barra de progreso

    println!(
        "El valor aproximado de Pi después de {} iteraciones es: {}",
        iterations,
        pi
    );
    let duration = start.elapsed();
    println!("La operación tomó: {:?}", duration);
}
