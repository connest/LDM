# Реализация алгоритма Кармаркара-Карпа (k = 2)

## Задача

Разделить множество S на k подмножеств так, чтобы минимизировать разность сумм элементов в этих подмножествах.

Это задача относится к NP-трудным, комбинаторным.

## Описание алгоритма

Согласно [wikipedia.org](https://en.wikipedia.org/wiki/Largest_differencing_method), существует alpha-приблеженный полиномиальный алгоритм для k = 2:

1. Отсортировать множество S по убыванию.
2. Выбирать два наибольших числа и вставлять обратно их разность. Повторять пока не останется 1 элемент: найденная разность множеств.
3. Вычислить [методом поиска с возвратом](https://ru.wikipedia.org/wiki/Поиск_с_возвратом) искомое разделение.

## Пример

```rust
use largest_differencing_method::ldm;

// Ожидается слайс кортежей (T, "тяжесть" T)
let values: Vec<(&u64, u64)> = vec![(&8, 8), (&7, 7), (&6, 6), (&5, 5), (&4, 4)];

let ldm_result = ldm::largest_differencing_method(&values);
    
assert_eq!(ldm_result.set_1, &[&4, &7, &5]);
assert_eq!(ldm_result.set_2, &[&8, &6]);
assert_eq!(ldm_result.diff, 2);
```


## Запуск и сопровождение

[Установите Rust](https://www.rust-lang.org/ru/tools/install)

```
git clone https://github.com/connest/LDM.git
cd LDM

cargo test 		# запуск тестов (должны пройти!)
cargo doc --open 	# документация к библиотеке
```