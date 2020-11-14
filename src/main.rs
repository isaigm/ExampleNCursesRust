use rand::*;
use ncurses::*;
use std::thread;
use std::time::Duration;

const MAX_ROWS : usize = 40;
const MAX_COLS : usize = 90;

type Cells = [[u8; MAX_COLS]; MAX_ROWS];

fn to_digits(mut n: u32, arr: &mut Vec<u8>){
    while n > 0{
        arr.push((n % 10) as u8);
        n /= 10;
    }
}
fn search(n: u8, arr: &Vec<u8>) -> bool{
    for e in arr.iter(){
        if *e == n {
            return true;
        }
    }
    return false;
}
struct Game{
    grid : Cells,
    aux : Cells,
    dead_rules: Vec<u8>,
    alive_rules: Vec<u8>,
    current_generations : usize,
}
impl Game{
    fn count_neighbors(&mut self, i : i32, j : i32) -> u8{
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
        let cell: chtype = chtype::from('0');
        attron(COLOR_PAIR(1) | A_BOLD());
        for i in 0..MAX_ROWS {
            for j in 0..MAX_COLS{
                match self.grid[i][j]{
                    0 => mvaddch((i+2) as i32, (j+25) as i32, space),
                    1 => mvaddch((i+2) as i32, (j+25) as i32, cell),
                    _ => continue,
                };
            }
        }
        attroff(COLOR_PAIR(1) | A_BOLD());
    }
    fn step(&mut self){
        for i in 0..MAX_ROWS {
            for j in 0..MAX_COLS{
                let not_dead_cells = self.count_neighbors(i as i32, j as i32);
                self.aux[i][j]  = match self.grid[i][j] {
                    0 if search(not_dead_cells, &self.alive_rules) => 1,
                    1 if !search(not_dead_cells, &self.dead_rules) => 0,
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
        let delay  = Duration::from_millis(100);
        const RESET : i32 = 'r' as i32;
        let info = newwin(15, 30, 2, 120);
        nodelay(info, true);
        loop{
            let ch = getch();
            match ch{
                27 => break,
                RESET => {
                    self.fill_random();
                    self.current_generations = 0;
                },
                _ => (),
            }
            self.step();
            clear();
            self.draw_cells();
            mvwprintw(info, 1, 1, format!("Current generations: {}", self.current_generations).as_str());
            mvwprintw(info, 4, 1, "r for restart");
            mvwprintw(info, 7, 1, "ESC for exit");
            box_(stdscr(), 0, 0);
            box_(info, 0, 0);
            wrefresh(stdscr());
            wrefresh(info);
            thread::sleep(delay);
            self.current_generations += 1;
        }
        delwin(info);
    }
    fn fill_random(&mut self){
        let max_cells = MAX_COLS * MAX_ROWS / 10;
        for i in 0..MAX_ROWS{
            for j in 0..MAX_COLS{
                self.grid[i][j] = 0;
                self.aux[i][j] = 0;
            }
        }
        for _ in 0..max_cells{
            let i = rand::thread_rng().gen_range(0, MAX_ROWS);
            let j = rand::thread_rng().gen_range(0, MAX_COLS);
            self.grid[i][j] = rand::thread_rng().gen_range(0, 2);
            self.aux[i][j] = self.grid[i][j];
        }
    }
    fn new(r1: u32, r2: u32) -> Self{
        let grid = [[0 as u8; MAX_COLS] ; MAX_ROWS];
        let aux =  [[0 as u8; MAX_COLS] ; MAX_ROWS];
        let mut alive_rules: Vec<u8> = Vec::new();
        let mut dead_rules: Vec<u8> = Vec::new();
        to_digits(r1, &mut alive_rules);
        to_digits(r2, &mut dead_rules);
        Self {grid, dead_rules, alive_rules, aux, current_generations : 0}
    }
}
fn main() {
    let mut game = Game::new(3, 23);
    game.fill_random();
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    keypad(stdscr(), true);
    noecho();
    nodelay(stdscr(), true);
    init_color(2, 600, 600, 0);
    init_pair(1, 2, COLOR_BLACK);
    game.game_loop();
    endwin();
}
