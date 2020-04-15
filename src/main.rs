/**

KC001 - Dodawanie liczb całkowitych
Napisz program, który oblicza sumę trzech liczb całkowitych.

Wejście
Na wejście programu podane zostaną trzy liczby całkowite (nieprzekraczające 100) rozdzielone znakiem nowej linii.

Wyjście
Na wyjściu ma się pojawić suma liczb, które pojawiły się na wejściu.

Przykład
Wejście:
100
-68
12

Wyjście:
44
*/

use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let sum: i16 = stdin.lock().lines().map(|el| el.unwrap().parse::<i16>().unwrap()).sum();
    println!("{}", sum);
}
