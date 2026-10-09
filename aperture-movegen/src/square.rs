#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Square {
    A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
    A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
    A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
    A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
    A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
    A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
    A2 = 8 , B2 = 9 , C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
    A1 = 0 , B1 = 1 , C1 = 2 , D1 = 3 , E1 = 4 , F1 = 5 , G1 = 6 , H1 = 7 ,
}

impl TryFrom<u8> for Square {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=63_u8).contains(&value) {
            /*
            This is safe becuase the Enum is represented with a u8
            and carries no data other than the discriminator, which ranges from 0 to 63 (inclusive)
            */
            let square: Square = unsafe { std::mem::transmute(value) };
            Ok(square)
        } else {
            Err("Out of range".to_owned())
        }
    }
}

impl std::str::FromStr for Square {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let Some(file) = chars.next() else {
            return Err("Missing square field: file".to_owned());
        };
        let file_number: u8 = match file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err(format!("Invalid file: unexpected character '{file}'")),
        };

        let Some(rank) = chars.next() else {
            return Err("Missing square field: rank".to_owned());
        };
        let Some(rank_number) = rank.to_digit(10) else {
            return Err(format!("Invalid rank: unexpected character '{rank}'"));
        };
        if !(1..=8).contains(&rank_number) {
            return Err(format!("Invalid rank: out of range '{rank_number}'"));
        }
        #[expect(clippy::cast_possible_truncation)]
        let rank_number = (rank_number - 1) as u8;

        Self::try_from(rank_number * 8 + file_number)
    }
}
