#[derive(Debug)]
pub struct Decimal(i64, i8);

impl Decimal {
    pub fn parse(data: &str) -> Self {
        let data = data.as_bytes();
        let mut dec: i8 = -1;
        let mut val: i64 = 0;
        let mut minus = false;
        for b in data {
            match b {
                b'.' => dec = 0,
                b'-' => minus = true,
                b'0'..=b'9' => {
                    val = val * 10 + (b - b'0') as i64;
                    if dec >= 0 {
                        dec = dec + 1;
                    }
                }
                _ => (),
            }
        }
        if minus {
            val = -val;
        }
        if dec < 0 {
            dec = 0;
        }
        Self(val, dec)
    }

    pub fn to_i64(&self) -> i64 {
        self.0 << 8 | (self.1 as i64)
    }
}
