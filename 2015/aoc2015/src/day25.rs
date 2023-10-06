use std::ops::Rem;

static ROW: usize = 3010 - 1; // I retreive the one since I work with 0 index
static COLUMN: usize = 3019 - 1; // same for retreivign the one

fn get_index(row: usize, col: usize) -> usize {
    // function to get the value that will be store in the [row, col] case
    // first get the on which row of the triangle the indices are
    if row == 0 && col == 0 {
        return 1;
    }
    let triangle_row = row + col;
    let prev_triangle_size = (triangle_row * (triangle_row + 1)) / 2;

    // compute the number of element on the previous triangle

    return prev_triangle_size + col + 1;
}

fn get_next_value(value: usize) -> usize {
    // function to generate the next value from the current one
    // given the described algorithm
    let mut out = value * 252533;
    out = out.rem(33554393);
    out
}

fn get_value(index: usize) -> usize {
    let mut value = 20151125;

    for _i in 0..index - 1 {
        value = get_next_value(value);
    }
    value
}
pub fn main() {
    // first we need to compute the wich index we need
    let index = get_index(ROW, COLUMN);
    //

    // getting the initial value
    let mut value = get_value(index);
    println!("part I = {value}");

    // for the second part I think I need to answer with the next 49 values
    for _i in 0..49 {
        value = get_next_value(value);
    }
    println!("part II = {value}");
}
