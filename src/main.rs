extern crate ncurses;
use rand::*;
use ncurses::*;
const MAX_ROWS : usize = 30;
const MAX_COLS : usize = 80;
fn count_neighbors(grid : [[u8; MAX_COLS]; MAX_ROWS], i : usize, j : usize) -> i32{
    let mut not_dead_cells = 0;
    for i1 in i-1..i+2{
        for j1 in j-1..j+2{
            if (i1 > 0 && i1 < MAX_ROWS) || (j1 > 0 && j1 < MAX_COLS){
                continue;
            }else{
                if grid[i1][j1] == 1{
                    not_dead_cells += 1;
                }
            }
        }
    }
    return not_dead_cells;
}
fn draw_cells(grid : [[u8; MAX_COLS]; MAX_ROWS]){
    let space : chtype = chtype::from(' ');
    let cell: chtype = chtype::from('*');
    for i in 0..MAX_ROWS {
        for j in 0..MAX_COLS{
            attron(COLOR_PAIR(1) | A_BOLD());
            match grid[i][j]{
                0 => mvaddch((i+5) as i32, (j+35) as i32, space),
                1 => mvaddch((i+5) as i32, (j+35) as i32, cell),
                _ => continue,
            };
            attroff(COLOR_PAIR(0) | A_BOLD());
            refresh();
        }
    }
}
fn step(grid : [[u8; MAX_COLS]; MAX_ROWS], aux : [[u8; MAX_COLS]; MAX_ROWS]){

}
fn main() {
    let mut grid = [[0 as u8; MAX_COLS] ; MAX_ROWS];
    let mut aux =  [[0 as u8; MAX_COLS] ; MAX_ROWS];
    for i in 0..MAX_ROWS {
        for j in 0..MAX_COLS{
            grid[i][j] = rand::thread_rng().gen_range(0, 2);
            aux[i][j] = grid[i][j];
        }
    }
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    init_pair(1, COLOR_RED, COLOR_BLACK);
    draw_cells(grid);
    getch();
    endwin();
}
