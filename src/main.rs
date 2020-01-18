use rand::*;
use ncurses::*;
use std::thread;
use std::time::Duration;

const MAX_ROWS : usize = 30;
const MAX_COLS : usize = 80;

type Cells = [[u8; MAX_COLS]; MAX_ROWS];

struct Game{
    grid : Cells,
    aux : Cells,
    current_generations : usize,
}
impl Game{
    fn count_neighbors(&mut self, i : i32, j : i32) -> i32{
        let mut not_dead_cells = 0;
        for i1 in i-1..i+2{
            for j1 in j-1..j+2{
                if i1 < 0 || i1 >= MAX_ROWS as i32 || j1 < 0 || j1 >= MAX_COLS as i32 || (i1 == i && j1 == j){
                    continue;
                }else{
                    if self.grid[i1 as usize][j1 as usize] == 1{
                        not_dead_cells += 1;
                    }
                }
            }
        }
        return not_dead_cells;
    }
    fn draw_cells(&self){
        let space : chtype = chtype::from(' ');
        let cell: chtype = chtype::from('^');
        attron(COLOR_PAIR(1) | A_BOLD());
        for i in 0..MAX_ROWS {
            for j in 0..MAX_COLS{
                match self.grid[i][j]{
                    0 => mvaddch((i+5) as i32, (j+35) as i32, space),
                    1 => mvaddch((i+5) as i32, (j+35) as i32, cell),
                    _ => continue,
                };
            }
        }
        refresh();
        attroff(COLOR_PAIR(1) | A_BOLD());
    }
    fn step(&mut self){
        for i in 0..MAX_ROWS {
            for j in 0..MAX_COLS{
                let not_dead_cells = self.count_neighbors(i as i32, j as i32);
                self.aux[i][j]  = match self.grid[i][j] {
                    0 if not_dead_cells == 3 => 1,
                    1 if !(not_dead_cells == 2 || not_dead_cells == 3) => 0,
                    _ => continue,
                }
            }
        }
        for i in 0..MAX_ROWS {
            for j in 0..MAX_COLS{
                self.grid[i][j] = self.aux[i][j];
            }
        }
    }
    fn game_loop(&mut self){
        let delay  = Duration::from_millis(140);
        const RESET : i32 = 'r' as i32;
        loop{
            let ch = getch();
            match ch{
                27 => break,
                RESET => {
                    for i in 0..MAX_ROWS {
                        for j in 0..MAX_COLS{
                            self.grid[i][j] = rand::thread_rng().gen_range(0, 2);
                            self.aux[i][j] = self.grid[i][j];
                        }
                    }
                    self.current_generations = 0;
                },
                _ => (),
            }
            self.step();
            wrefresh(stdscr());
            self.draw_cells();
            mvprintw(10, 120, format!("Current generations: {}", self.current_generations).as_str());
            refresh();
            thread::sleep(delay);
            self.current_generations += 1;
        }
    }
    fn new() -> Game{
        let mut grid = [[0 as u8; MAX_COLS] ; MAX_ROWS];
        let mut aux =  [[0 as u8; MAX_COLS] ; MAX_ROWS];
        for i in 0..MAX_ROWS {
            for j in 0..MAX_COLS{
                grid[i][j] = rand::thread_rng().gen_range(0, 2);
                aux[i][j] = grid[i][j];
            }
        }
        Game {grid, aux, current_generations : 0}
    }
}
fn main() {
    let mut game = Game::new();
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    noecho();
    nodelay(stdscr(), true);
    init_pair(1, COLOR_RED, COLOR_BLACK);
    game.game_loop();
    endwin();
}