fn main() {
    let input = "главрыба"; // Исходная строка

    let reversed = reverse_string(input); // Сохраняем перевернутую строку

    // Вывод результата
    println!("Исходная строка: {}", input);
    println!("Перевернутая строка: {}", reversed);
}

// Функция переворота строки
fn reverse_string(input: &str) -> String {
    input.chars().rev().collect() // Переворачиваем строку
}
