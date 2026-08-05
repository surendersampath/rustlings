fn main() {
    // You can optionally experiment here.
    let data = [ 10, 20, 30, 40, 50 ];

    let data_slice = &data[2..4];
    println!("data_slice has length {} and elements {:?}", data_slice.len(), data_slice);
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
