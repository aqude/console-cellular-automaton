mod lifefunc;

fn main() {

    // границы карты
    let (rows, cols) = (10, 10);

    // создание сетки
    let mut grid: Vec<Vec<i8>> = vec![vec![0; cols]; rows];

    // Начальное состояние
    grid[1][2] = 1;
    grid[2][2] = 1;
    grid[3][2] = 1;
    grid[4][2] = 1;

    // Создание сетки
    println!("Initial grid:");
    grid.iter().for_each(|i| {
        println!("{:?}", i);
    });

    println!("");

    // колл-во генераций
    const ITR: u8 = 5;

    // запуск и вывод генерации
    for i in 0..ITR {
        // grid = gol(&grid);
        grid = crate::lifefunc::gol(&grid);

        println!("Generation {}:", i+1);
        grid.iter().for_each(|i| {
            println!("{:?}", i);
        });
        println!("");
    }
}