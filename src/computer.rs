use crate::slice::PuzzleSlice;

use crate::wheels;

pub struct Computer {
    computer_wheels: [[[u8; 12]; 4]; 5],
    pub current_state: [usize; 4],
}

impl Computer {
    pub fn new(init_state: Option<[usize; 4]>, inner_first: bool) -> Self {
        Self {
            computer_wheels: wheels::get_wheels(inner_first),
            current_state: init_state.unwrap_or([0, 0, 0, 0]),
        }
    }

    pub fn rotate_wheel(&mut self, wheel_idx: usize) {
        self.current_state[wheel_idx] = (self.current_state[wheel_idx] + 1) % 12;
    }

    pub fn get_slice(&self, computer_index: usize) -> PuzzleSlice {
        let mut slice_array = [0u8; 4];
        let (base, not_base) = self.computer_wheels.split_last().unwrap();

        for slice_idx in 0..4 {
            // Try to find a non-zero value from the wheels
            for (wheel_idx, wheel) in not_base.iter().enumerate() {
                let wheel_turn_idx = (self.current_state[wheel_idx] + computer_index) % 12;
                
                if wheel[slice_idx][wheel_turn_idx] > 0 && slice_array[slice_idx] == 0 {
                    slice_array[slice_idx] = wheel[slice_idx][wheel_turn_idx];
                    break; // Found a value, no need to check other wheels
                }
            }
            
            // If no value was found, use the base wheel
            if slice_array[slice_idx] == 0 {
                slice_array[slice_idx] = base[slice_idx][computer_index];
            }
        }

        PuzzleSlice {
            inner: slice_array[0],
            inner_middle: slice_array[1],
            outer_middle: slice_array[2],
            outer: slice_array[3],
        }
    }

    pub fn get_all_slices(&self) -> [PuzzleSlice; 12] {
        let mut slices = [PuzzleSlice { inner: 0, inner_middle: 0, outer_middle: 0, outer: 0 }; 12];
        for (i, slice) in slices.iter_mut().enumerate() {
            *slice = self.get_slice(i);
        }
        slices
    }

    pub fn get_all_slice_sums(&self) -> [u8; 12] {
        let slices = self.get_all_slices();
        let mut sums = [0u8; 12];
        
        for (i, slice) in slices.iter().enumerate() {
            sums[i] = slice.sum();
        }
        
        sums
    }
}
