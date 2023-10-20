use std::iter::Iterator;

use greek_computer::computer::Computer;

fn main() {
    let mut computer = Computer::new(None, true);

    while !computer
        .get_all_slice_sums()
        .into_iter()
        .all(|n| n == 42 as u8)
    {
        computer.rotate_wheel(0);

        if computer.current_state[0] == 0 {
            computer.rotate_wheel(1);
        }

        if computer.current_state[0] == 0 && computer.current_state[1] == 0 {
            computer.rotate_wheel(2);
        }

        if computer.current_state[0] == 0
            && computer.current_state[1] == 0
            && computer.current_state[2] == 0
        {
            computer.rotate_wheel(3);
        }
    }

    println!(
        "Computer State: {:?}\nSlices: {:?}\nSums: {:?}",
        computer.current_state,
        computer.get_all_slices(),
        computer.get_all_slice_sums()
    );
}
