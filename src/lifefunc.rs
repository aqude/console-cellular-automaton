pub fn gol(grid: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    // Числа ряда x
    let n = grid.len();
    // Числа столбов
    let m = grid[0].len();
    // Сетка для нового поколения
    let mut future: Vec<Vec<i8>> = vec![vec![0; n]; m];
    // Перебор ячейки
    for i in 0..n {
        for j in 0..m {
            // Запрос на (жив/умер)
            let cell_state = grid[i][j];
            // ослеживание для живых клеток
            let mut live_neighbors = 0;
            // Повторных перебор всех клеток
            for x in -1i8..=1 {
                for y in -1i8..=1 {
                    // позиция одного из клетки (new_x, new_y)
                    let new_x = (i as i8) + x;
                    let new_y = (j as i8) + y;
                    // Проверка на нахождение клетки в сетке
                    if new_x > 0 && new_y > 0 && new_x < n as i8 && new_y < m as i8 {
                        live_neighbors += grid[new_x as usize][new_y as usize];
                    }
                }
            }
            // вычитание состояния для получения колл-ва живых клеток
            live_neighbors -= cell_state;
            // правила для получения будущего поколения
            if cell_state == 1 && live_neighbors < 2 {
                future[i][j] = 0;
            } else if cell_state == 1 && live_neighbors > 3 {
                future[i][j] = 0;
            } else if cell_state == 0 && live_neighbors == 3 {
                future[i][j] = 1;
            } else {
                future[i][j] = cell_state;
            }
        }
    }
    // вернуть будущее поколение
    future
}