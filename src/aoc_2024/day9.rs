use crate::aoc_2024::day9::MemoryType::{File, Space};
use std::collections::HashSet;
use std::str::FromStr;

pub fn day9(input: &String) -> String {
    let memory_map = MemoryMap::from_str(input).unwrap();
    log::info!("Map parsed:    {:?}", memory_map.to_string());
    let optimized_map = memory_map.optimize(true);
    log::info!("Map optimized: {:?}", optimized_map.to_string());
    optimized_map.checksum().to_string()
}

pub fn day9_2(input: &String) -> String {
    let mut memory_map = MemoryMap::from_str(input).unwrap();
    log::info!("Map parsed:    {:?}", memory_map.to_string());
    memory_map.optimize_v2();
    log::info!("Map optimized: {:?}", memory_map.to_string());
    memory_map.checksum().to_string()
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct MemoryMap {
    blocks: Vec<MemoryBlock>,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
struct MemoryBlock {
    mem_type: MemoryType,
    id: usize,
    size: u8,
}

impl MemoryBlock {
    fn new_mem(id: usize, size: u8) -> MemoryBlock {
        MemoryBlock {
            mem_type: File,
            id,
            size,
        }
    }
    fn new_empty(size: u8) -> MemoryBlock {
        MemoryBlock {
            mem_type: Space,
            id: usize::MAX,
            size,
        }
    }

    fn is_not_empty(&self) -> bool {
        self.size != 0
    }

    fn reduce_size_by(&mut self, size: u8) {
        self.size = self.size - size;
    }

    fn reduce_size_to_zero(&mut self) {
        self.size = 0;
    }

    fn is_space(&self) -> bool {
        self.mem_type == Space
    }
}

struct MemoryCell {
    id: usize,
    mem_type: MemoryType,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum MemoryType {
    File,
    Space,
}

impl MemoryMap {
    fn new(block_ids: Vec<MemoryBlock>) -> MemoryMap {
        MemoryMap { blocks: block_ids }
    }

    fn push(&mut self, block: MemoryBlock) {
        self.blocks.push(block);
    }

    /// To calculate the checksum, add up the result of multiplying each of these blocks' position
    /// with the file ID number it contains. The leftmost block is in position 0.
    /// If a block contains free space, skip it instead.
    fn checksum(&self) -> usize {
        self.to_cells()
            .iter()
            .enumerate()
            .filter(|e| e.1.mem_type == File)
            .map(|(pos, d)| pos * d.id)
            .sum()
    }

    fn to_string(&self) -> String {
        self.to_cells()
            .iter()
            .map(|cell| match cell.mem_type {
                File => format!("[{}]", cell.id),
                Space => String::from("."),
            })
            .collect()
    }

    fn to_cells(&self) -> Vec<MemoryCell> {
        self.blocks
            .iter()
            .flat_map(|block| {
                (0..block.size)
                    .map(|_| MemoryCell {
                        id: block.id,
                        mem_type: block.mem_type,
                    })
                    .collect::<Vec<MemoryCell>>()
            })
            .collect::<Vec<MemoryCell>>()
    }

    fn fill_in_first_space_available(
        &mut self,
        block_pos: usize,
        ids_tried_moved: &mut HashSet<usize>,
    ) -> bool {
        let block_to_move = self.blocks[block_pos];
        if block_to_move.is_space() {
            return false;
        }
        if ids_tried_moved.contains(&block_to_move.id) {
            return false;
        }
        ids_tried_moved.insert(block_to_move.id);

        let space_needed = block_to_move.size;
        let first_space_available = self
            .blocks
            .iter_mut()
            .take(block_pos)
            .enumerate()
            .filter(|e| e.1.is_space())
            .filter(|e| e.1.size >= space_needed)
            .next();
        if first_space_available.is_some() {
            let (space_pos, space) = first_space_available.unwrap();
            space.reduce_size_by(block_to_move.size);
            self.blocks[block_pos] = MemoryBlock::new_empty(block_to_move.size);
            self.blocks.insert(space_pos, block_to_move);
            log::debug!(
                "Moved {} of size {} to location {}",
                block_to_move.id,
                block_to_move.size,
                space_pos
            );
            true
        } else {
            log::debug!("Couldn't Move {}", block_to_move.id);
            false
        }
    }

    fn optimize_v2(&mut self) {
        let mut ids_tried_moved = HashSet::<usize>::new();
        let mut index = self.blocks.len() - 1;
        while index > 0 {
            let moved_something = self.fill_in_first_space_available(index, &mut ids_tried_moved);
            index = index - if moved_something { 0 } else { 1 };
        }
    }

    fn optimize(&self, allow_split: bool) -> MemoryMap {
        let mut new_mem = MemoryMap::new(Vec::new());

        let mut working_copy = self.blocks.clone();

        let mut reverse_idx = working_copy.len() - 1;

        for block_index in 0..working_copy.len() {
            let block = &working_copy[block_index];
            if block.is_not_empty() {
                match block.mem_type {
                    File => {
                        // mem block, do nothing
                        new_mem.push(block.clone());
                        // and to be sure to not reuse it:
                        working_copy[block_index].reduce_size_to_zero();
                    }
                    Space => {
                        // let's fill it with what's at the end
                        let mut available_space = block.size.clone();
                        while available_space > 0 {
                            match working_copy[reverse_idx].mem_type {
                                Space => {
                                    // empty? size to 0 and go to next one
                                    working_copy[reverse_idx].reduce_size_to_zero();
                                    if reverse_idx > 0 {
                                        reverse_idx -= 1;
                                    } else {
                                        break;
                                    }
                                }
                                File => {
                                    // mem? can we fit it in the space?
                                    if working_copy[reverse_idx].size < available_space {
                                        // block has the place to go whole
                                        new_mem.push(working_copy[reverse_idx].clone());
                                        available_space =
                                            available_space - working_copy[reverse_idx].size;
                                        // soft remove the element
                                        working_copy[reverse_idx].reduce_size_to_zero();
                                        if reverse_idx > 0 {
                                            reverse_idx -= 1;
                                        } else {
                                            break;
                                        }
                                    } else {
                                        if allow_split {
                                            // cut the block
                                            let mut block_to_insert =
                                                working_copy[reverse_idx].clone();
                                            block_to_insert.reduce_size_by(
                                                working_copy[reverse_idx].size - available_space,
                                            );
                                            new_mem.push(block_to_insert);
                                            working_copy[reverse_idx]
                                                .reduce_size_by(available_space);
                                            // this will stop the loop
                                            available_space = 0;
                                        } else {
                                            // just go to next available value
                                            if reverse_idx > 0 {
                                                reverse_idx -= 1;
                                            } else {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        new_mem
    }
}

impl FromStr for MemoryMap {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(MemoryMap::new(
            input
                .chars()
                .filter_map(|c| c.to_digit(10))
                .enumerate()
                .map(|(position, block_size)| {
                    if position % 2 == 0 {
                        // file identifier
                        let block_id = position / 2;
                        MemoryBlock::new_mem(block_id, block_size as u8)
                    } else {
                        // empty block
                        MemoryBlock::new_empty(block_size as u8)
                    }
                })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &'static str = "2333133121414131402";

    #[test]
    #[test_log::test]
    fn test_day9() {
        let input = String::from(PUZZLE_INPUT);

        let result = day9(&input);

        assert_eq!(String::from("1928"), result);
    }

    #[test]
    fn test_day9_2() {
        let input = String::from(PUZZLE_INPUT);

        let result = day9_2(&input);

        assert_eq!(String::from("2858"), result);
    }

    #[test]
    fn test_day9_2_should_not_relocated_after() {
        let input = String::from("2010116");

        let result = day9_2(&input);

        assert_eq!(String::from("143"), result);
    }
}
