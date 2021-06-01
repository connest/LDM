# Реализация алгоритма Кармаркара-Карпа (k = 2)

## Задача

Разделить множество S на k подмножеств так, чтобы минимизировать разность сумм элементов в этих подмножествах.

Это задача относится к NP-трудным, комбинаторным.

## Описание алгоритма

Согласно [wikipedia.org](https://en.wikipedia.org/wiki/Largest_differencing_method), существует $\alpha$-приблеженный полиномиальный алгоритм для k = 2:

1. Отсортировать множество S по убыванию
2. Выбирать два наибольших числа и вставлять обратно их разность. Повторять пока не останется 1 элемент: найденная разность множеств.
3. Вычислить [методом поиска с возвратом](https://ru.wikipedia.org/wiki/Поиск_с_возвратом) искомое разделение




