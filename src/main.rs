use std::iter::Iterator;
use std::time::Instant;

use greek_computer::computer::Computer;

fn main() {
    let start_time = Instant::now();
    let mut computer = Computer::new(None, true);

    // Loop until all slice sums equal 42
    while !computer.get_all_slice_sums().iter().all(|&n| n == 42) {
        // Increment wheels in a more structured way
        computer.rotate_wheel(0);
        
        // If wheels need to be rotated in sequence
        if computer.current_state[0] == 0 {
            computer.rotate_wheel(1);
        }
        
        if computer.current_state[0] == 0 && computer.current_state[1] == 0 {
            computer.rotate_wheel(2);
        }
        
        if computer.current_state[0] == 0 && computer.current_state[1] == 0 && computer.current_state[2] == 0 {
            computer.rotate_wheel(3);
        }
    }

    let elapsed = start_time.elapsed();
    
    println!("Puzzle solved in {:.2?}\n", elapsed);
    println!(
        "Computer State: {:?}\nSlices: {:?}\nSums: {:?}",
        computer.current_state,
        computer.get_all_slices(),
        computer.get_all_slice_sums()
    );
}
