#[derive(Clone)]
pub struct Number {
    start_i: usize,
    end_i: usize,
    value: u32,
}

impl Number {
    pub fn get_start_i(&self) -> usize {
        self.start_i
    }
    pub fn get_end_i(&self) -> usize {
        self.end_i
    }
    pub fn get_value(&self) -> u32 {
        self.value
    }
}

pub enum States {
    NoDigit,
    Digit(Number),
}

impl States {
    pub fn process_next_char(&mut self, c: char, i: usize) -> Option<Number> {
        if c.is_numeric() {
            match self {
                // We found a new number
                States::NoDigit => {
                    *self = States::Digit(Number {
                        start_i: i,
                        end_i: i,
                        value: c.to_digit(10).unwrap(),
                    });
                    None
                }
                // Continuation of the previous number
                States::Digit(ref mut number) => {
                    number.end_i = i;
                    number.value = number.value * 10 + c.to_digit(10).unwrap();
                    None
                }
            }
        } else {
            match self {
                // No number
                States::NoDigit => None,
                // End of a number
                States::Digit(number) => {
                    let number_to_process = number.clone();
                    *self = States::NoDigit;
                    Some(number_to_process)
                }
            }
        }
    }
}
