#[allow(dead_code)]
struct SeatManager {
    seats: Vec<i32>,
}

#[allow(dead_code)]
impl SeatManager {
    fn new(n: i32) -> Self {
        Self {
            seats: (1..=n).rev().collect(),
        }
    }
    fn reserve(&mut self) -> i32 {
        self.seats.pop().unwrap()
    }
    fn unreserve(&mut self, seat_number: i32){
        let mut left: usize = 0;
        let mut right: usize = self.seats.len();

        while left < right {
            let middle: usize = left + (right - left) / 2;
            if self.seats[middle] < seat_number {
                right = middle;
            } else {
                left = middle + 1;
            }
        }
        self.seats.insert(left, seat_number);
    }
}
