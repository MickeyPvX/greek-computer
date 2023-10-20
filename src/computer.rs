use crate::{slice::PuzzleSlice, wheels};

pub struct Computer {
    computer_wheels: [[[u8; 12]; 4]; 5],
    pub current_state: [usize; 4],
}

impl Computer {
    pub fn new(init_state: Option<[usize; 4]>, inner_first: bool) -> Self {
        match init_state {
            Some(state) => Self {
                computer_wheels: wheels::get_wheels(inner_first),
                current_state: state,
            },
            None => Self {
                computer_wheels: wheels::get_wheels(inner_first),
                current_state: [0, 0, 0, 0],
            },
        }
    }

    pub fn rotate_wheel(&mut self, wheel_idx: usize) {
        self.current_state[wheel_idx] = (self.current_state[wheel_idx] + 1) % 12;
    }

    pub fn get_slice(&self, computer_index: usize) -> PuzzleSlice {
        let mut slice_array: [u8; 4] = [0, 0, 0, 0];
        let (base, not_base) = self.computer_wheels.split_last().unwrap();

        for (slice_idx, _) in slice_array.clone().into_iter().enumerate() {
            for (wheel_idx, wheel) in not_base.iter().enumerate() {
                let wheel_turn_idx: usize =
                    (self.current_state[wheel_idx] + computer_index) % wheel[slice_idx].len();

                if wheel[slice_idx][wheel_turn_idx] > 0 && slice_array[slice_idx] == 0 {
                    slice_array[slice_idx] = wheel[slice_idx][wheel_turn_idx];
                    continue;
                }
            }
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
        let mut slice_vec: Vec<PuzzleSlice> = Vec::with_capacity(12);
        for computer_idx in 0..12 {
            slice_vec.push(self.get_slice(computer_idx));
        }

        slice_vec.try_into().unwrap()
    }

    pub fn get_all_slice_sums(&self) -> [u8; 12] {
        let mut sum_vec: Vec<u8> = Vec::with_capacity(12);

        for slice in self.get_all_slices() {
            sum_vec.push(slice.sum());
        }

        sum_vec.try_into().unwrap()
    }
}
