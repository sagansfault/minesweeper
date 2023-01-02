use rand::Rng;

pub struct Grid {
    pub state: Vec<Cell>,
    pub len: usize
}

impl Grid {
    pub fn new(size_args: Size) -> Grid {
        let size = size_args.num_val();
        let mine_count = size_args.mine_count();
        let mut vec: Vec<Cell> = vec![Cell::Sus(0, CellState::Hidden); size * size];

        let mut locs: Vec<(usize, usize)> = Vec::new();

        for x in 0..size {
            for y in 0..size {
                locs.push((x, y));
            }
        }

        let mut mines: Vec<(usize, usize)> = Vec::new();
        for _ in 0..mine_count {
            mines.push(locs.remove(rand::thread_rng().gen_range(0..locs.len())))
        }

        let full_len = size * size;
        for mine in mines {
            if let Some(val) = vec.get_mut(mine.1 * size + mine.0) {
                *val = Cell::Mine(CellState::Hidden);
            }

            let mut offsets: Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
            if mine.0 % size == 0 {
                offsets = offsets.into_iter().filter(|(x, _)| *x >= 0).collect();
            }
            if mine.0 + 1 % size == 0 {
                offsets = offsets.into_iter().filter(|(x, _)| *x <= 0).collect();
            }

            for off in offsets {
                let new_x = mine.0 as i32 + off.0;
                let new_y = mine.1 as i32 + off.1;
                if new_x < 0 || new_x >= full_len as i32 || new_y < 0 || new_y >= full_len as i32 {
                    continue;
                }
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if let Some(val) = vec.get_mut((new_y * size + new_x) as usize) {
                    if let Cell::Sus(sus_val, hidden) = *val {
                        *val = Cell::Sus(sus_val + 1, hidden);
                    }
                }
            }
        }

        return Grid {
            state: vec,
            len: size
        };
    }

    pub fn reveal_recurs(&mut self, index: usize) {
        if let Some(cell) = self.state.get_mut(index) {
            if match cell {
                Cell::Sus(_, state) | Cell::Mine(state) => *state
            } == CellState::Revealed {
                return;
            }
            *cell = match cell {
                Cell::Mine(_) => Cell::Mine(CellState::Revealed),
                Cell::Sus(val, _) => Cell::Sus(*val, CellState::Revealed),
            };
            if let Cell::Sus(val, _) = cell {
                if *val == 0 {
                    let index = index as i32;
                    let len = self.len as i32;
                    let mut offsets = vec![index - len, index + len];
                    if index + 1 % len != 0 {
                        offsets.push(index + 1);
                    }
                    if index % len != 0 {
                        offsets.push(index - 1);
                    }
                    for offset in offsets {
                        if offset >= 0 && offset < self.state.len() as i32 {
                            self.reveal_recurs(offset as usize);
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Cell {
    Mine(CellState),
    Sus(u8, CellState)
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CellState {
    Hidden,
    Flagged,
    Revealed
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Size {
    EIGHT, TEN, TWELVE
}

pub trait NumVal {
    fn num_val(&self) -> usize;
}

pub trait MineCount {
    fn mine_count(&self) -> usize;
}

impl NumVal for Size {
    fn num_val(&self) -> usize {
        match self {
            Size::EIGHT => 8,
            Size::TEN => 10,
            Size::TWELVE => 12,
        }
    }
}

impl MineCount for Size {
    fn mine_count(&self) -> usize {
        match self {
            Size::EIGHT => 10,
            Size::TEN => 13,
            Size::TWELVE => 17,
        }
    }
}