use rand::prelude::*;

use crate::checkers::checkers_game::{Checker, CheckerColor, CheckerRank};

const BOARD_SIZE: usize = 64;

pub struct ZobristHashing {
    zobrist_table: [[u64; 8]; BOARD_SIZE]
}

impl ZobristHashing {
    pub fn register() -> Self {
        let mut rng = rand::rng();
        let mut table: [[u64; 8]; BOARD_SIZE] = [[0; 8]; BOARD_SIZE];
        for cell in table.iter_mut() {
            for value in cell.iter_mut() {
                *value = rng.next_u64();
            }
        }

        ZobristHashing { zobrist_table: table }
    }

    pub fn hash(&self, data: &[Checker; 64]) -> u64 {
        let mut h: u64 = 0;
        for cell in 0..BOARD_SIZE {
            let val = ZobristHashing::checker_to_u8(data[cell]);
            h ^= self.zobrist_table[cell][val as usize];
        }
        h
    }

    fn checker_to_u8(checker: Checker) -> u8 {
        match (checker.occupied, checker.color, checker.rank) {
            (false, _, _) => 0,
            (true, CheckerColor::Black, CheckerRank::Soldier) => 0b100,
            (true, CheckerColor::Black, CheckerRank::King) => 0b110,
            (true, CheckerColor::Red, CheckerRank::Soldier) => 0b101,
            (true, CheckerColor::Red, CheckerRank::King) => 0b111
        }
    }
}


#[derive(Clone)]
pub enum TTFlag {
    Exact,
    LowerBound,
    UpperBound
}

#[derive(Clone)]
pub struct TTEntry {
    pub hash: u64,
    pub score: i32,
    pub depth: u8,
    pub flag: TTFlag,
    pub best_move: u8
}
const _: () = assert!(size_of::<TTEntry>() == 16, "TTEntry is wrong size!");

pub struct TranspositionTable {
    table: Vec<TTEntry>,
    size: usize
}

impl TranspositionTable {
    /// Creates a new Transposition table.
    /// size_in_mb is the size cap of the table in base 1024 megabytes.
    pub fn new(size_in_mb: usize) -> Self {
        let number_of_entries = (size_in_mb * 1024 * 1024) / size_of::<TTEntry>();
        let mut tt = TranspositionTable { table: Vec::new(), size: 1 };
        while tt.size * 2 <= number_of_entries {
            tt.size *= 2;
        }

        tt.table.resize(tt.size, TTEntry { hash: 0, score: 0, depth: 0, flag: TTFlag::Exact, best_move: 255 });

        tt
    }

    pub fn probe(&self, hash: u64) -> Option<&TTEntry> {
        let index = hash as usize & (self.size - 1);
        return if self.table[index].hash == hash { Some(&self.table[index]) } else { None } ;
    }

    pub fn store(&mut self, hash: u64, score: i32, depth: u8, flag: TTFlag, best_move: u8) {
        let mut e: &mut TTEntry = &mut self.table[hash as usize & (self.size - 1)];

        if e.hash != hash && e.depth > depth {
            return;
        }

        *e = TTEntry{ hash, score, depth, flag, best_move };
    }
}