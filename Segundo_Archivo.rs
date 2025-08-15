use std::{io};

/*fn mod_inv(a: u64, p: u64) -> u64 {
    let mut m0 = p;
    let mut y = 0;
    let mut x = 1;

    if p == 1 {
        return 0;
    }

    let mut a = a % p;
    while a > 1 {
        let q = a / m0;
        let mut t = m0;

        m0 = a % m0;
        a = t;
        t = y;

        // Asegúrate de que x no se vuelva negativo
        if x < q * y {
            x += p; // Ajusta x para evitar el desbordamiento
        }

        // Asegúrate de que la resta no cause un desbordamiento
        y = (x as i64 - (q as i64 * y as i64)) as u64; // Usar i64 para evitar desbordamiento
        if y >= p {
            y -= p; // Mantener y dentro del rango
        }
        x = t;
    }

    if x < 0 {
        x += p;
    }

    x
}

fn precompute_factorials(n: u64, modulo: u64) -> (Vec<u64>, Vec<u64>) {
    let mut fact = vec![1; (n + 1) as usize];
    let mut inv_fact = vec![1; (n + 1) as usize];

    for i in 2..=n {
        fact[i as usize] = (fact[(i - 1) as usize] * i) % modulo;
    }

    inv_fact[n as usize] = mod_inv(fact[n as usize], modulo);
    for i in (1..n).rev() {
        inv_fact[i as usize] = (inv_fact[(i + 1) as usize] * (i + 1)) % modulo;
    }

    (fact, inv_fact)
}

fn numero_catalan(n: u64, modulo: u64, fact: &Vec<u64>, inv_fact: &Vec<u64>) -> u64 {
    if n == 0 {
        return 1;
    }
    (fact[(2 * n) as usize] * inv_fact[(n + 1) as usize] % modulo * inv_fact[n as usize] % modulo) % modulo
}

fn ej_2063f1() {
    let mut tc_raw: String = String::new();
    io::stdin()
        .read_line(&mut tc_raw)
        .expect("Error al leer la línea");

    let tc_text = tc_raw.trim();
    let tc: u64 = tc_text.parse().unwrap(); // Cambiado a u64

    let modulo = 998244353;
    let (fact, inv_fact) = precompute_factorials(10000, modulo); // Precomputar hasta 2 * 5000

    for _i in 1..=tc {
        let mut num_gud_pairs_raw: String = String::new();
        io::stdin()
            .read_line(&mut num_gud_pairs_raw)
            .expect("Error al leer la línea");
        let num_gud_pairs_text: &str = num_gud_pairs_raw.trim();
        let num_gud_pairs: u64 = num_gud_pairs_text.parse().unwrap(); // Cambiado a u64
        let res1 = numero_catalan(num_gud_pairs, modulo, &fact, &inv_fact);
        print!("{} ", res1);
        let mut cant_pares = 0;
        let mut taken = vec![0; (num_gud_pairs * 2) as usize]; // Cambiado a u64
        for _j in 1..=num_gud_pairs {
            let mut i_f_raw: String = String::new();
            io::stdin()
                .read_line(&mut i_f_raw)
                .expect("Error al leer la línea");
            let i_f: &str = i_f_raw.trim();
            let i: usize = i_f.split(' ').collect::<Vec<_>>()[0].parse().unwrap();
            let f: usize = i_f.split(' ').collect::<Vec<_>>()[1].parse().unwrap();
            taken[i - 1] = 1;
            taken[f - 1] = 2;
            cant_pares += 1;
            let mut cant_tomados = 0;
            let mut cant_pares_tomados = 0;
            let mut pas = 1;
            let mut res: Vec<u64> = vec![0; (num_gud_pairs * 2) as usize]; // Cambiado a u64
            let mut ind = 0;
            for num in &taken {
                cant_tomados += 1;
                match num {
                    1 => {
                        ind += 1;
                    }
                    2 => {
                        if res[ind] > 2 {
                            pas *= numero_catalan((res[ind] / 2), modulo, &fact, &inv_fact);
                        }
                        res[ind] = 0;
                        ind -= 1;
                        cant_pares_tomados += 1;
                    }
                    _ => {
                        res[ind] += 1; // Incrementar vacíos para otros números
                    }
                }
                if cant_pares_tomados == cant_pares {
                    break;
                }
            }
            if (((num_gud_pairs * 2) - (cant_tomados) + res[0]) / 2) <= 1 {
                if pas <= 1 {
                    print!("1 ");
                } else {
                    print!("{} ", pas % modulo);
                }
            } else {
                pas *= numero_catalan((((num_gud_pairs * 2) - (cant_tomados) + res[0]) / 2).try_into().unwrap(), modulo, &fact, &inv_fact);
                print!("{} ", pas % modulo);
            }
        }
        println!();
    }
}

fn main() {
    let modulo = 998244353;
    let (fact, inv_fact) = precompute_factorials(10000, modulo); // Precomputar hasta 2 * 5000

    // Ahora puedes calcular los números de Catalan
    //println!("{}, {}", numero_catalan(18, modulo, &fact, &inv_fact), numero_catalan(4, modulo, &fact, &inv_fact));
    // ej_2063f1();
}

fn ej1_coprimos(){
    // Crear una nueva cadena para almacenar la entrada del usuario
    let mut tc_raw: String = String::new();
    // Numero de casos
    io::stdin()
        .read_line(&mut tc_raw)
        .expect("Error al leer la línea");

    // Eliminar el salto de línea al final de la entrada
    let tc_text = tc_raw.trim();
    let tc: i32 = tc_text.parse().unwrap();
    for _i in 1..=tc{
        let mut l_r_raw: String = String::new();
        io::stdin()
            .read_line(&mut l_r_raw)
            .expect("Error al leer la línea");
        let l_r: &str = l_r_raw.trim();
        let l: i32 = l_r.split(' ').collect::<Vec<_>>()[0].parse().unwrap();
        let r: i32= l_r.split(' ').collect::<Vec<_>>()[1].parse().unwrap();
        if l == r && l == 1 {
            println!("1");
        }
        else{
            println!("{}", (l-r).abs())
        }
    }
    //nombre = nombre.split(' ').collect::<Vec<_>>()[1];
    // Imprimir el saludo
    
}*/
fn mod_inv(a: u64, p: u64) -> u64 {
    let mut base = a;
    let mut exp = p - 2;
    let mut res = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % p;
        }
        base = (base * base) % p;
        exp /= 2;
    }
    res
}

fn precompute_factorials(n: u64, modulo: u64) -> (Vec<u64>, Vec<u64>) {
    let mut fact = vec![1; (n + 1) as usize];
    let mut inv_fact = vec![1; (n + 1) as usize];
 
    for i in 2..=n {
        fact[i as usize] = (fact[(i - 1) as usize] * i) % modulo;
    }
 
    inv_fact[n as usize] = mod_inv(fact[n as usize], modulo);
    for i in (1..n).rev() {
        inv_fact[i as usize] = (inv_fact[(i + 1) as usize] * (i + 1)) % modulo;
    }
 
    (fact, inv_fact)
}

fn numero_catalan(n: u64, modulo: u64, fact: &Vec<u64>, inv_fact: &Vec<u64>) -> u64 {
    if n == 0 {
        return 1;
    }
    (fact[(2 * n) as usize] * inv_fact[(n + 1) as usize] % modulo * inv_fact[n as usize] % modulo) % modulo
}
fn ej_2063f1() {
    let mut tc_raw = String::new();
    io::stdin().read_line(&mut tc_raw).expect("Error al leer la línea");
    let tc: u64 = tc_raw.trim().parse().unwrap();
 
    let modulo = 998244353;
    let (fact, inv_fact) = precompute_factorials(10000, modulo);
 
    for _ in 0..tc {
        let mut num_gud_pairs_raw = String::new();
        io::stdin().read_line(&mut num_gud_pairs_raw).expect("Error al leer la línea");
        let num_gud_pairs: u64 = num_gud_pairs_raw.trim().parse().unwrap();
 
        let res1 = numero_catalan(num_gud_pairs, modulo, &fact, &inv_fact);
        print!("{} ", res1);
 
        let mut cant_pares = 0;
        let mut taken = vec![0; (num_gud_pairs * 2) as usize];
 
        for _ in 0..num_gud_pairs {
            let mut i_f_raw = String::new();
            io::stdin().read_line(&mut i_f_raw).expect("Error al leer la línea");
            let parts: Vec<usize> = i_f_raw.trim().split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let (i, f) = (parts[0], parts[1]);
 
            taken[i - 1] = 1;
            taken[f - 1] = 2;
            cant_pares += 1;
 
            let mut cant_tomados = 0;
            let mut cant_pares_tomados = 0;
            let mut pas = 1;
            let mut res = vec![0; (num_gud_pairs * 2) as usize];
            let mut ind = 0;
 
            for num in &taken {
                cant_tomados += 1;
                match num {
                    1 => {
                        ind += 1;
                    }
                    2 => {
                        if res[ind] > 2 {
                            pas = (pas * numero_catalan((res[ind] / 2), modulo, &fact, &inv_fact)) % modulo;
                        }
                        res[ind] = 0;
                        if ind > 0 { ind -= 1; }
                        cant_pares_tomados += 1;
                    }
                    _ => {
                        res[ind] += 1;
                    }
                }
                if cant_pares_tomados == cant_pares {
                    break;
                }
            }
 
            let remaining_pairs = ((num_gud_pairs * 2) - cant_tomados + res[0]) / 2;
            if remaining_pairs <= 1 {
                print!("{} ", pas % modulo);
            } else {
                pas = (pas * numero_catalan(remaining_pairs, modulo, &fact, &inv_fact)) % modulo;
                print!("{} ", pas);
            }
        }
        println!();
    }
}
fn main() {
    ej_2063f1();
}