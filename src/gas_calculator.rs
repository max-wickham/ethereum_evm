


#[inline]
pub fn call_data_gas_cost(data: &Vec<u8>) -> usize {
    let mut cost = 0;
    for byte in data {
        if *byte == 0 {
            cost += 4;
        } else {
            cost += 16;
        }
    }
    cost
}
