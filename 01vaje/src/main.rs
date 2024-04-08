use core::panic;

/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(a0: u32, a1: u32, n: u32) -> u32 {
    match n {
        0 => a0,
        1 => a1,
        x => fib(a0, a1, n-1) + fib(a0, a1, n-2)
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

fn je_prestopno(n : u32) -> bool {
    if n % 400 == 0 {
        true
    }
    else if n % 100 == 0 {
        false
    }
    else if n % 4 == 0 {
        true
    }
    else {
        false
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);

fn je_veljaven_datum(datum: Date) -> bool {
    let (d, m, l) = datum;
    let seznam: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if m > 12 {
        return false;
    } else if je_prestopno(l) {
        if d <= 29 {return true;}
        else {return false;}
    } else if d <= seznam[(m - 1) as usize] {
        return true;
    } else {
        return false;
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, 
/// zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, 
/// ki zadošča zaustavitvenemu pogoju.

fn iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    while !cond(fun(start)) {
        start = fun(start)
    }
    start
}

fn iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b] glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

fn bisekcija(mut a: f64, mut b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    let c = (a + b) / 2.0;
    println!("{}, {}, {}", a, b, c);
    if fun(c).abs() < prec || (b - a) < prec {
        c
    } else if fun(a) * fun(c) < 0.0 {
        bisekcija (a, c, fun, prec)
    } else {
        bisekcija(c, b, fun, prec)
    }
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.

use std::io::stdin;

// fn igra() {
//     fn ugibaj () -> i32 {
//         let mut prvi = String::new();
//         stdin().read_line(&mut prvi).expect("Fail");
//         let mut drugi = String::new();
//         stdin().read_line(&mut prvi).expect("Fail");
//         let ena: i32 = match prvi.trim().parse() {
//             Ok(num) => num,
//             Err(_) => 1
//         };
//     }
//     let dva: i32 = match drugi.trim().parse() {
//         Ok(num) => num,
//         Err(_) => 1
//     };

//     let diferenca = dva - ena;


// }

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    let [[q, p], [c, d]] = a;
    let [[x, y], [z, w]] = b;
    return [[(q*x + p*z),(q*y + p*w)], [(c*x + d*z), (c*y + d*w)]]
    }

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    let l = arr.len();
    if l <= 2 {
       return true
    }
    else if arr[0] < arr[1] {
        for i in 0..(l-1) {
            if arr[*&i] > arr[*&i + 1]
            {return false}
        }
    }
    else if arr[0] > arr[1] {
        for i in 0..(l-1) {
            if arr[*&i] < arr[*&i + 1]
            {return false}
        }
    }
    return true
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`

fn pow(mut x: u32, mut n: u32) -> u32 {
    if n == 0 {
        return 1
    }
    if n == 1{
        return x
    }
    if n == 2 {
        return x * x
    }
    else if n % 2 == 0 {
        return pow(pow(x, n/2), 2)
    }
    else {
        return x * pow(pow(x, (n - 1)/2), 2)
    }
}

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32 {
    if n == 0 {
        return 1 % m
    }
    if n == 1{
        return x % m
    }
    if n == 2 {
        return (x * x) % m
    }
    else if n % 2 == 0 {
        return pow(pow(x, n/2) % m, 2) % m 
    }
    else {
        return x * pow(pow(x, (n - 1)/2) % m, 2) % m
    }
}

/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(arr: &mut [u32]) {}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn pyramid(n: u32) {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahkom predpostavite, da bo n največ 26.
///      A
///    A B A
///   A B C B A
/// A B C D C B A
/// Napišite funkcijo `fn selection_sort(mut arr: [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem
fn selection_sort(mut arr: &mut[u32]) -> &[u32] {
    let l = arr.len();
    if l <= 1 {
        return arr
    }
    fn minimum (ar: &[u32]) -> usize {
        let li = ar.len();
        let mut m = ar[0];
        let mut indeks = 0;
        for i in 0..li {
            if ar[i] < m {
                m = ar[i];
                indeks = i;
            }
        } 
        indeks
    }
    for i in 0..(l-1) {
        let novi = &arr[i..=l-1];
        let indeks = minimum(novi) + i;

        let a = arr[i];
        let b = arr[indeks as usize];
        arr[i] = b;
        arr[indeks as usize] = a;
    }
    println!("{:?}", arr);
    arr
}

fn zvezna2 (x: f64) -> f64 {
    x - 2.0
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        assert_eq!(result, ());
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(0, 1, 0), 0);
        assert_eq!(fib(0, 1, 1), 1);
        assert_eq!(fib(0, 1, 2), 1);
        assert_eq!(fib(0, 1, 3), 2);
        assert_eq!(fib(0, 1, 4), 3);
        assert_eq!(fib(0, 1, 5), 5);
        assert_eq!(fib(0, 1, 6), 8);
        assert_eq!(fib(0, 1, 7), 13);
        assert_eq!(fib(0, 1, 8), 21);
        assert_eq!(fib(0, 1, 9), 34);
        assert_eq!(fib(0, 1, 10), 55);
    }

    #[test]
    fn test_prestopno() {
        assert_eq!(je_prestopno(2023), false);
        assert_eq!(je_prestopno(2024), true);
        assert_eq!(je_prestopno(2020), true);
        assert_eq!(je_prestopno(400), true);
        assert_eq!(je_prestopno(100), false);
        assert_eq!(je_prestopno(4), true);
    }

    #[test]
    fn test_je_veljaven_datum() {
        assert_eq!(je_veljaven_datum((1,13,2000)), false);
        assert_eq!(je_veljaven_datum((29,2,2024)), true);
        assert_eq!(je_veljaven_datum((1,2,2000)), true);
        assert_eq!(je_veljaven_datum((32,1,20)), false);
        assert_eq!(je_veljaven_datum((29,2,2023)), false);
    } 

    fn manjsi_od7 (a: u32) -> bool {
        return a < 7
    }

    fn pristej1 (a: u32) -> u32 {
        a + 1
    }

    fn odstej1 (a: u32) -> u32 {
        a - 1
    }

    #[test]
    fn test_iteracija() {
        assert_eq!(iteracija(0, pristej1, manjsi_od7), 0);
        assert_eq!(iteracija(10, odstej1, manjsi_od7), 7);
    }

    fn zvezna (x: f64) -> f64 {
        x
    }

    fn zvezna2 (x: f64) -> f64 {
        x - 2.0
    }

    use more_asserts;

    #[test]
    fn test_bisekcija() {
        more_asserts::assert_le!((bisekcija(-3.0, 3.0, zvezna2, 0.01) - 2.0).abs(), 0.01);
        assert_eq!(bisekcija(-1.0, 1.0, zvezna, 0.01), 0.0);
    }

    #[test]
    fn test_matrike() {
        assert_eq!(mat_mul([[1,0],[0,1]], [[2,3],[4,5]]), [[2, 3], [4, 5]]);
    }

    #[test]
    fn test_urejen() {
        assert_eq!(ordered(&[1]), true);
        assert_eq!(ordered(&[1, 2]), true);
        assert_eq!(ordered(&[1, 2, 3]), true);
        assert_eq!(ordered(&[1, 2, 3, 2]), false);
    }

    #[test]
    fn test_potence() {
        assert_eq!(pow(3, 2), 9);
        assert_eq!(pow(2, 0), 1);
        assert_eq!(pow(2, 1), 2);
        assert_eq!(pow(2, 10), 1024);
    }

    #[test]
    fn test_mod() {
        assert_eq!(pow_mod(3, 2, 2), 1);
        assert_eq!(pow_mod(2, 0, 1), 0);
        assert_eq!(pow_mod(2, 1, 1), 0);
        assert_eq!(pow_mod(2, 10, 1), 0);
    }

    #[test]
    fn test_sort() {
        assert_eq!(selection_sort(&mut[1]), [1]);
        assert_eq!(selection_sort(&mut[1, 2]), [1, 2]);
        assert_eq!(selection_sort(&mut[2, 1]), [1, 2]);
        assert_eq!(selection_sort(&mut[4, 5, 2, 9, 22, 1]), [1, 2, 4, 5, 9, 22]);
    }
}

