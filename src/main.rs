enum Value {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    White,
}

impl Copy for Value {}

impl Clone for Value {
    fn clone(&self) -> Value {
        *self
    }
}

use std::fmt;
impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Red => write!(f, "Red"),
            Value::Orange => write!(f, "Orange"),
            Value::Yellow => write!(f, "Yellow"),
            Value::Green => write!(f, "Green"),
            Value::Blue => write!(f, "Blue"),
            Value::White => write!(f, "White"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Red => match other {
                Value::Red => true,
                _ => false,
            },
            Value::Orange => match other {
                Value::Orange => true,
                _ => false,
            },
            Value::Yellow => match other {
                Value::Yellow => true,
                _ => false,
            },
            Value::Green => match other {
                Value::Green => true,
                _ => false,
            },
            Value::Blue => match other {
                Value::Blue => true,
                _ => false,
            },
            Value::White => match other {
                Value::White => true,
                _ => false,
            },
        }
    }
}

type Configuration = [Value; 54];
type ConfigurationRef<'t> = [&'t Value; 54];

fn realise(c: &ConfigurationRef) -> Configuration {
    let mut o = [Value::White; 54];
    (0..54).for_each(|index| o[index] = *(c[index]));
    o
}

fn to_ref(c: &Configuration) -> ConfigurationRef {
    let mut o = [&Value::White; 54];
    (0..54).for_each(|index| o[index] = &(c[index]));
    o
}
fn identity<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[0], &c[1], &c[2], &c[3], &c[4], &c[5], &c[6], &c[7], &c[8], &c[9], &c[10], &c[11],
        &c[12], &c[13], &c[14], &c[15], &c[16], &c[17], &c[18], &c[19], &c[20], &c[21], &c[22],
        &c[23], &c[24], &c[25], &c[26], &c[27], &c[28], &c[29], &c[30], &c[31], &c[32], &c[33],
        &c[34], &c[35], &c[36], &c[37], &c[38], &c[39], &c[40], &c[41], &c[42], &c[43], &c[44],
        &c[45], &c[46], &c[47], &c[48], &c[49], &c[50], &c[51], &c[52], &c[53],
    ]
}
fn top_twist_right<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[2], &c[5], &c[8], &c[1], &c[4], &c[7], &c[0], &c[3], &c[6], &c[53], &c[52], &c[51],
        &c[9], &c[10], &c[11], &c[12], &c[13], &c[14], &c[18], &c[19], &c[20], &c[21], &c[22],
        &c[23], &c[24], &c[25], &c[26], &c[27], &c[28], &c[29], &c[30], &c[31], &c[32], &c[33],
        &c[34], &c[35], &c[36], &c[37], &c[38], &c[39], &c[40], &c[41], &c[42], &c[43], &c[44],
        &c[45], &c[46], &c[47], &c[48], &c[49], &c[50], &c[17], &c[16], &c[15],
    ]
}
fn top_twist_left<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[6], &c[3], &c[0], &c[7], &c[4], &c[1], &c[8], &c[5], &c[2], &c[12], &c[13], &c[14],
        &c[15], &c[16], &c[17], &c[53], &c[52], &c[51], &c[18], &c[19], &c[20], &c[21], &c[22],
        &c[23], &c[24], &c[25], &c[26], &c[27], &c[28], &c[29], &c[30], &c[31], &c[32], &c[33],
        &c[34], &c[35], &c[36], &c[37], &c[38], &c[39], &c[40], &c[41], &c[42], &c[43], &c[44],
        &c[45], &c[46], &c[47], &c[48], &c[49], &c[50], &c[11], &c[10], &c[9],
    ]
}
fn front_twist_right<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[0], &c[1], &c[2], &c[3], &c[4], &c[5], &c[29], &c[20], &c[11], &c[9], &c[10], &c[36],
        &c[30], &c[21], &c[12], &c[6], &c[16], &c[17], &c[18], &c[19], &c[37], &c[31], &c[22],
        &c[13], &c[7], &c[25], &c[26], &c[27], &c[28], &c[38], &c[32], &c[23], &c[14], &c[8],
        &c[34], &c[35], &c[15], &c[24], &c[33], &c[39], &c[40], &c[41], &c[42], &c[43], &c[44],
        &c[45], &c[46], &c[47], &c[48], &c[49], &c[50], &c[51], &c[52], &c[53],
    ]
}
fn front_twist_left<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[0], &c[1], &c[2], &c[3], &c[4], &c[5], &c[15], &c[24], &c[33], &c[9], &c[10], &c[8],
        &c[14], &c[23], &c[32], &c[38], &c[16], &c[17], &c[18], &c[19], &c[7], &c[13], &c[22],
        &c[31], &c[37], &c[25], &c[26], &c[27], &c[28], &c[6], &c[12], &c[21], &c[30], &c[36],
        &c[34], &c[35], &c[11], &c[20], &c[29], &c[39], &c[40], &c[41], &c[42], &c[43], &c[44],
        &c[45], &c[46], &c[47], &c[48], &c[49], &c[50], &c[51], &c[52], &c[53],
    ]
}
fn right_twist_front<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[0], &c[1], &c[47], &c[3], &c[4], &c[50], &c[6], &c[7], &c[53], &c[9], &c[10], &c[11],
        &c[12], &c[13], &c[2], &c[17], &c[26], &c[35], &c[18], &c[19], &c[20], &c[21], &c[22],
        &c[5], &c[16], &c[25], &c[34], &c[27], &c[28], &c[29], &c[30], &c[31], &c[8], &c[15],
        &c[24], &c[33], &c[36], &c[37], &c[14], &c[39], &c[40], &c[23], &c[42], &c[43], &c[32],
        &c[45], &c[46], &c[38], &c[48], &c[49], &c[41], &c[51], &c[52], &c[44],
    ]
}
fn right_twist_back<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[0], &c[1], &c[14], &c[3], &c[4], &c[23], &c[6], &c[7], &c[32], &c[9], &c[10], &c[11],
        &c[12], &c[13], &c[38], &c[33], &c[24], &c[15], &c[18], &c[19], &c[20], &c[21], &c[22],
        &c[41], &c[34], &c[25], &c[16], &c[27], &c[28], &c[29], &c[30], &c[31], &c[44], &c[35],
        &c[26], &c[17], &c[36], &c[37], &c[47], &c[39], &c[40], &c[50], &c[42], &c[43], &c[53],
        &c[45], &c[46], &c[2], &c[48], &c[49], &c[5], &c[51], &c[52], &c[8],
    ]
}
fn bottom_twist_right<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[0], &c[1], &c[2], &c[3], &c[4], &c[5], &c[6], &c[7], &c[8], &c[9], &c[10], &c[11],
        &c[12], &c[13], &c[14], &c[15], &c[16], &c[17], &c[18], &c[19], &c[20], &c[21], &c[22],
        &c[23], &c[24], &c[25], &c[26], &c[47], &c[46], &c[45], &c[27], &c[28], &c[29], &c[30],
        &c[31], &c[32], &c[42], &c[39], &c[36], &c[43], &c[40], &c[37], &c[44], &c[41], &c[38],
        &c[35], &c[34], &c[33], &c[48], &c[49], &c[50], &c[51], &c[52], &c[53],
    ]
}
fn bottom_twist_left<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[0], &c[1], &c[2], &c[3], &c[4], &c[5], &c[6], &c[7], &c[8], &c[9], &c[10], &c[11],
        &c[12], &c[13], &c[14], &c[15], &c[16], &c[17], &c[18], &c[19], &c[20], &c[21], &c[22],
        &c[23], &c[24], &c[25], &c[26], &c[30], &c[31], &c[32], &c[33], &c[34], &c[35], &c[47],
        &c[46], &c[45], &c[38], &c[41], &c[44], &c[37], &c[40], &c[43], &c[36], &c[39], &c[42],
        &c[29], &c[28], &c[27], &c[48], &c[49], &c[50], &c[51], &c[52], &c[53],
    ]
}
fn back_twist_right<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[27], &c[18], &c[9], &c[3], &c[4], &c[5], &c[6], &c[7], &c[8], &c[42], &c[10], &c[11],
        &c[12], &c[13], &c[14], &c[15], &c[16], &c[0], &c[43], &c[19], &c[20], &c[21], &c[22],
        &c[23], &c[24], &c[25], &c[1], &c[44], &c[28], &c[29], &c[30], &c[31], &c[32], &c[33],
        &c[34], &c[2], &c[36], &c[37], &c[38], &c[39], &c[40], &c[41], &c[35], &c[26], &c[17],
        &c[47], &c[50], &c[53], &c[46], &c[49], &c[52], &c[45], &c[48], &c[51],
    ]
}
fn back_twist_left<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[17], &c[26], &c[35], &c[3], &c[4], &c[5], &c[6], &c[7], &c[8], &c[2], &c[10], &c[11],
        &c[12], &c[13], &c[14], &c[15], &c[16], &c[44], &c[1], &c[19], &c[20], &c[21], &c[22],
        &c[23], &c[24], &c[25], &c[43], &c[0], &c[28], &c[29], &c[30], &c[31], &c[32], &c[33],
        &c[34], &c[42], &c[36], &c[37], &c[38], &c[39], &c[40], &c[41], &c[9], &c[18], &c[27],
        &c[51], &c[48], &c[45], &c[52], &c[49], &c[46], &c[53], &c[50], &c[47],
    ]
}
fn left_twist_front<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[45], &c[1], &c[2], &c[48], &c[4], &c[5], &c[51], &c[7], &c[8], &c[27], &c[18], &c[9],
        &c[0], &c[13], &c[14], &c[15], &c[16], &c[17], &c[28], &c[19], &c[10], &c[3], &c[22],
        &c[23], &c[24], &c[25], &c[26], &c[29], &c[20], &c[11], &c[6], &c[31], &c[32], &c[33],
        &c[34], &c[35], &c[12], &c[37], &c[38], &c[21], &c[40], &c[41], &c[30], &c[43], &c[44],
        &c[36], &c[46], &c[47], &c[39], &c[49], &c[50], &c[42], &c[52], &c[53],
    ]
}
fn left_twist_back<'t>(c: &'t ConfigurationRef) -> ConfigurationRef<'t> {
    [
        &c[12], &c[1], &c[2], &c[21], &c[4], &c[5], &c[30], &c[7], &c[8], &c[11], &c[20], &c[29],
        &c[36], &c[13], &c[14], &c[15], &c[16], &c[17], &c[10], &c[19], &c[28], &c[39], &c[22],
        &c[23], &c[24], &c[25], &c[26], &c[9], &c[18], &c[27], &c[42], &c[31], &c[32], &c[33],
        &c[34], &c[35], &c[45], &c[37], &c[38], &c[48], &c[40], &c[41], &c[51], &c[43], &c[44],
        &c[0], &c[46], &c[47], &c[3], &c[49], &c[50], &c[6], &c[52], &c[53],
    ]
}

enum Operation {
    Identity,
    TopRight,
    TopLeft,
    FrontRight,
    FrontLeft,
    RightFront,
    RightBack,
    BottomRight,
    BottomLeft,
    BackRight,
    BackLeft,
    LeftFront,
    LeftBack,
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::Identity => write!(f, "Identity"),
            Operation::TopRight => write!(f, "TopRight"),
            Operation::TopLeft => write!(f, "TopLeft"),
            Operation::FrontRight => write!(f, "FrontRight"),
            Operation::FrontLeft => write!(f, "FrontLeft"),
            Operation::RightFront => write!(f, "RightFront"),
            Operation::RightBack => write!(f, "RightBack"),
            Operation::BottomRight => write!(f, "BottomRight"),
            Operation::BottomLeft => write!(f, "BottomLeft"),
            Operation::BackRight => write!(f, "BackRight"),
            Operation::BackLeft => write!(f, "BackLeft"),
            Operation::LeftFront => write!(f, "LeftFront"),
            Operation::LeftBack => write!(f, "LeftBack"),
        }
    }
}

fn operation_to_function<'t>(
    x: &'t Operation,
) -> Box<fn(&'t ConfigurationRef) -> ConfigurationRef<'t>> {
    match x {
        Operation::Identity => Box::new(identity),
        Operation::TopRight => Box::new(top_twist_right),
        Operation::TopLeft => Box::new(top_twist_left),
        Operation::FrontRight => Box::new(front_twist_right),
        Operation::FrontLeft => Box::new(front_twist_left),
        Operation::RightFront => Box::new(right_twist_front),
        Operation::RightBack => Box::new(right_twist_back),
        Operation::BottomRight => Box::new(bottom_twist_right),
        Operation::BottomLeft => Box::new(bottom_twist_left),
        Operation::BackRight => Box::new(back_twist_right),
        Operation::BackLeft => Box::new(back_twist_left),
        Operation::LeftFront => Box::new(left_twist_front),
        Operation::LeftBack => Box::new(left_twist_back),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_realise() {
        let config: ConfigurationRef = [
            &Value::White,
            &Value::Blue,
            &Value::White,
            &Value::Yellow,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::Red,
            &Value::White,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::White,
            &Value::Green,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Green,
            &Value::Green,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Red,
            &Value::Red,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Blue,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Green,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::Yellow,
            &Value::Red,
            &Value::White,
            &Value::Yellow,
            &Value::Blue,
            &Value::Yellow,
            &Value::Yellow,
            &Value::Green,
        ];
        let expected: Configuration = [
            Value::White,
            Value::Blue,
            Value::White,
            Value::Yellow,
            Value::White,
            Value::White,
            Value::Blue,
            Value::Red,
            Value::White,
            Value::Orange,
            Value::Orange,
            Value::Orange,
            Value::Yellow,
            Value::White,
            Value::Green,
            Value::Red,
            Value::Green,
            Value::Orange,
            Value::Orange,
            Value::Orange,
            Value::Orange,
            Value::Green,
            Value::Green,
            Value::Yellow,
            Value::Red,
            Value::Red,
            Value::Green,
            Value::Orange,
            Value::Orange,
            Value::Yellow,
            Value::Blue,
            Value::Blue,
            Value::Green,
            Value::Red,
            Value::Red,
            Value::Yellow,
            Value::Red,
            Value::Red,
            Value::Blue,
            Value::Blue,
            Value::Blue,
            Value::Green,
            Value::Green,
            Value::White,
            Value::White,
            Value::Blue,
            Value::Yellow,
            Value::Red,
            Value::White,
            Value::Yellow,
            Value::Blue,
            Value::Yellow,
            Value::Yellow,
            Value::Green,
        ];
        let result = realise(&config);
        (0..54).for_each(|index| assert_eq!(result[index], expected[index]));
    }
    #[test]
    fn test_to_ref() {
        let config: Configuration = [
            Value::White,
            Value::White,
            Value::White,
            Value::Blue,
            Value::White,
            Value::Red,
            Value::White,
            Value::Yellow,
            Value::Blue,
            Value::Green,
            Value::Yellow,
            Value::Yellow,
            Value::Orange,
            Value::Orange,
            Value::Orange,
            Value::Yellow,
            Value::White,
            Value::Green,
            Value::Orange,
            Value::Orange,
            Value::Orange,
            Value::Green,
            Value::Green,
            Value::Yellow,
            Value::Red,
            Value::Red,
            Value::Green,
            Value::Orange,
            Value::Orange,
            Value::Yellow,
            Value::Blue,
            Value::Blue,
            Value::Green,
            Value::Red,
            Value::Red,
            Value::Yellow,
            Value::Red,
            Value::Red,
            Value::Blue,
            Value::Blue,
            Value::Blue,
            Value::Green,
            Value::Green,
            Value::White,
            Value::White,
            Value::Blue,
            Value::Yellow,
            Value::Red,
            Value::White,
            Value::Yellow,
            Value::Blue,
            Value::Orange,
            Value::Green,
            Value::Red,
        ];

        let expected: ConfigurationRef = [
            &Value::White,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::White,
            &Value::Red,
            &Value::White,
            &Value::Yellow,
            &Value::Blue,
            &Value::Green,
            &Value::Yellow,
            &Value::Yellow,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::White,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Green,
            &Value::Green,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Red,
            &Value::Red,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Blue,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Green,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::Yellow,
            &Value::Red,
            &Value::White,
            &Value::Yellow,
            &Value::Blue,
            &Value::Orange,
            &Value::Green,
            &Value::Red,
        ];
        let result = to_ref(&config);
        (0..54).for_each(|index| assert_eq!(*result[index], *expected[index]));
    }
    #[test]
    fn test_top_twist_right() {
        let config: ConfigurationRef = [
            &Value::White,
            &Value::Blue,
            &Value::White,
            &Value::Yellow,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::Red,
            &Value::White,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::White,
            &Value::Green,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Green,
            &Value::Green,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Red,
            &Value::Red,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Blue,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Green,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::Yellow,
            &Value::Red,
            &Value::White,
            &Value::Yellow,
            &Value::Blue,
            &Value::Yellow,
            &Value::Yellow,
            &Value::Green,
        ];
        let expected = [
            &Value::White,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::White,
            &Value::Red,
            &Value::White,
            &Value::Yellow,
            &Value::Blue,
            &Value::Green,
            &Value::Yellow,
            &Value::Yellow,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::White,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Green,
            &Value::Green,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Red,
            &Value::Red,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Blue,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Green,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::Yellow,
            &Value::Red,
            &Value::White,
            &Value::Yellow,
            &Value::Blue,
            &Value::Orange,
            &Value::Green,
            &Value::Red,
        ];
        let result = top_twist_right(&config);
        (0..54).for_each(|index| {
            println!("{}\t{:?}->{:?}", index, *result[index], *expected[index]);
            assert_eq!(*result[index], *expected[index])
        });
    }
    #[test]
    fn test_top_twist_left() {
        let config: ConfigurationRef = [
            &Value::White,
            &Value::Blue,
            &Value::White,
            &Value::Yellow,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::Red,
            &Value::White,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::White,
            &Value::Green,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Green,
            &Value::Green,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Red,
            &Value::Red,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Blue,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Green,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::Yellow,
            &Value::Red,
            &Value::White,
            &Value::Yellow,
            &Value::Blue,
            &Value::Yellow,
            &Value::Yellow,
            &Value::Green,
        ];
        let expected = [
            &Value::Blue,   //0
            &Value::Yellow, //1
            &Value::White,  //2
            &Value::Red,    //3
            &Value::White,  //4
            &Value::Blue,   //5
            &Value::White,  //6
            &Value::White,  //7
            &Value::White,  //8
            &Value::Yellow, //9
            &Value::White,  //10
            &Value::Green,  //11
            &Value::Red,    //12
            &Value::Green,  //13
            &Value::Orange, //14
            &Value::Green,  //15
            &Value::Yellow, //16
            &Value::Yellow, //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Orange, //20
            &Value::Green,  //21
            &Value::Green,  //22
            &Value::Yellow, //23
            &Value::Red,    //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Orange, //27
            &Value::Orange, //28
            &Value::Yellow, //29
            &Value::Blue,   //30
            &Value::Blue,   //31
            &Value::Green,  //32
            &Value::Red,    //33
            &Value::Red,    //34
            &Value::Yellow, //35
            &Value::Red,    //36
            &Value::Red,    //37
            &Value::Blue,   //38
            &Value::Blue,   //39
            &Value::Blue,   //40
            &Value::Green,  //41
            &Value::Green,  //42
            &Value::White,  //43
            &Value::White,  //44
            &Value::Blue,   //45
            &Value::Yellow, //46
            &Value::Red,    //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::Blue,   //50
            &Value::Orange, //51
            &Value::Orange, //52
            &Value::Orange, //53
        ];
        let result = top_twist_left(&config);
        (0..54).for_each(|index| {
            println!("{}\t{:?}->{:?}", index, *result[index], *expected[index]);
            assert_eq!(*result[index], *expected[index])
        });
    }
    #[test]
    fn test_front_twist_right() {
        let config: ConfigurationRef = [
            &Value::White,
            &Value::Blue,
            &Value::White,
            &Value::Yellow,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::Red,
            &Value::White,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::White,
            &Value::Green,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Green,
            &Value::Green,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Red,
            &Value::Red,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Blue,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Green,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::Yellow,
            &Value::Red,
            &Value::White,
            &Value::Yellow,
            &Value::Blue,
            &Value::Yellow,
            &Value::Yellow,
            &Value::Green,
        ];
        let expected = [
            &Value::White,  //0
            &Value::Blue,   //1
            &Value::White,  //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::White,  //5
            &Value::Yellow, //6
            &Value::Orange, //7
            &Value::Orange, //8
            &Value::Orange, //9
            &Value::Orange, //10
            &Value::Red,    //11
            &Value::Blue,   //12
            &Value::Green,  //13
            &Value::Yellow, //14
            &Value::Blue,   //15
            &Value::Green,  //16
            &Value::Orange, //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Red,    //20
            &Value::Blue,   //21
            &Value::Green,  //22
            &Value::White,  //23
            &Value::Red,    //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Orange, //27
            &Value::Orange, //28
            &Value::Blue,   //29
            &Value::Green,  //30
            &Value::Yellow, //31
            &Value::Green,  //32
            &Value::White,  //33
            &Value::Red,    //34
            &Value::Yellow, //35
            &Value::Red,    //36
            &Value::Red,    //37
            &Value::Red,    //38
            &Value::Blue,   //39
            &Value::Blue,   //40
            &Value::Green,  //41
            &Value::Green,  //42
            &Value::White,  //43
            &Value::White,  //44
            &Value::Blue,   //45
            &Value::Yellow, //46
            &Value::Red,    //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::Blue,   //50
            &Value::Yellow, //51
            &Value::Yellow, //52
            &Value::Green,  //53
        ];
        let result = front_twist_right(&config);
        (0..54).for_each(|index| {
            println!("{}\t{:?}->{:?}", index, *result[index], *expected[index]);
            assert_eq!(*result[index], *expected[index])
        });
    }
    #[test]
    fn test_front_twist_left() {
        let config: ConfigurationRef = [
            &Value::White,
            &Value::Blue,
            &Value::White,
            &Value::Yellow,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::Red,
            &Value::White,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::White,
            &Value::Green,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Green,
            &Value::Green,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Red,
            &Value::Red,
            &Value::Yellow,
            &Value::Red,
            &Value::Red,
            &Value::Blue,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Green,
            &Value::White,
            &Value::White,
            &Value::Blue,
            &Value::Yellow,
            &Value::Red,
            &Value::White,
            &Value::Yellow,
            &Value::Blue,
            &Value::Yellow,
            &Value::Yellow,
            &Value::Green,
        ];
        let expected = [
            &Value::White,  //0
            &Value::Blue,   //1
            &Value::White,  //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::White,  //5
            &Value::Red,    //6
            &Value::Red,    //7
            &Value::Red,    //8
            &Value::Orange, //9
            &Value::Orange, //10
            &Value::White,  //11
            &Value::Green,  //12
            &Value::Yellow, //13
            &Value::Green,  //14
            &Value::Blue,   //15
            &Value::Green,  //16
            &Value::Orange, //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Red,    //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Red,    //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Orange, //27
            &Value::Orange, //28
            &Value::Blue,   //29
            &Value::Yellow, //30
            &Value::Green,  //31
            &Value::Blue,   //32
            &Value::Red,    //33
            &Value::Red,    //34
            &Value::Yellow, //35
            &Value::Orange, //36
            &Value::Orange, //37
            &Value::Yellow, //38
            &Value::Blue,   //39
            &Value::Blue,   //40
            &Value::Green,  //41
            &Value::Green,  //42
            &Value::White,  //43
            &Value::White,  //44
            &Value::Blue,   //45
            &Value::Yellow, //46
            &Value::Red,    //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::Blue,   //50
            &Value::Yellow, //51
            &Value::Yellow, //52
            &Value::Green,  //53
        ];
        let result = front_twist_left(&config);
        (0..54).for_each(|index| {
            println!("{}\t{:?}->{:?}", index, *result[index], *expected[index]);
            assert_eq!(*result[index], *expected[index])
        });
    }
    #[test]
    fn test_right_twist_front() {
        let config: ConfigurationRef = [
            &Value::White,  //0
            &Value::Blue,   //1
            &Value::Green,  //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::Yellow, //5
            &Value::Orange, //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Orange, //9
            &Value::Orange, //10
            &Value::Blue,   //11
            &Value::Green,  //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::White,  //15
            &Value::Red,    //16
            &Value::Red,    //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Yellow, //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Orange, //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Blue,   //27
            &Value::Blue,   //28
            &Value::Green,  //29
            &Value::Red,    //30
            &Value::Green,  //31
            &Value::Orange, //32
            &Value::Blue,   //33
            &Value::Green,  //34
            &Value::White,  //35
            &Value::Blue,   //36
            &Value::Blue,   //37
            &Value::Yellow, //38
            &Value::Red,    //39
            &Value::Blue,   //40
            &Value::White,  //41
            &Value::Red,    //42
            &Value::Orange, //43
            &Value::Red,    //44
            &Value::Yellow, //45
            &Value::Green,  //46
            &Value::Yellow, //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::Red,    //50
            &Value::Yellow, //51
            &Value::Yellow, //52
            &Value::White,  //53
        ];
        let expected = [
            &Value::White,  //0
            &Value::Blue,   //1
            &Value::Yellow, //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::Red,    //5
            &Value::Orange, //6
            &Value::Red,    //7
            &Value::White,  //8
            &Value::Orange, //9
            &Value::Orange, //10
            &Value::Blue,   //11
            &Value::Green,  //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::Red,    //15
            &Value::Green,  //16
            &Value::White,  //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Yellow, //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::Yellow, //23
            &Value::Red,    //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Blue,   //27
            &Value::Blue,   //28
            &Value::Green,  //29
            &Value::Red,    //30
            &Value::Green,  //31
            &Value::Orange, //32
            &Value::White,  //33
            &Value::Orange, //34
            &Value::Blue,   //35
            &Value::Blue,   //36
            &Value::Blue,   //37
            &Value::Green,  //38
            &Value::Red,    //39
            &Value::Blue,   //40
            &Value::Blue,   //41
            &Value::Red,    //42
            &Value::Orange, //43
            &Value::Orange, //44
            &Value::Yellow, //45
            &Value::Green,  //46
            &Value::Yellow, //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::White,  //50
            &Value::Yellow, //51
            &Value::Yellow, //52
            &Value::Red,    //53
        ];
        let result = right_twist_front(&config);
        (0..54).for_each(|index| {
            println!("{}\t{:?}->{:?}", index, *result[index], *expected[index]);
            assert_eq!(*result[index], *expected[index])
        });
    }
    #[test]
    fn test_right_twist_back() {
        let config: ConfigurationRef = [
            &Value::White,
            &Value::Blue,
            &Value::Green,
            &Value::Yellow,
            &Value::White,
            &Value::Yellow,
            &Value::Orange,
            &Value::Red,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Blue,
            &Value::Green,
            &Value::White,
            &Value::Green,
            &Value::White,
            &Value::Red,
            &Value::Red,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::White,
            &Value::Green,
            &Value::Blue,
            &Value::Orange,
            &Value::Red,
            &Value::Green,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Blue,
            &Value::Green,
            &Value::White,
            &Value::Blue,
            &Value::Blue,
            &Value::Yellow,
            &Value::Red,
            &Value::Blue,
            &Value::White,
            &Value::Red,
            &Value::Orange,
            &Value::Red,
            &Value::Yellow,
            &Value::Green,
            &Value::Yellow,
            &Value::White,
            &Value::Yellow,
            &Value::Red,
            &Value::Yellow,
            &Value::Yellow,
            &Value::White,
        ];
        let expected = [
            &Value::White,  //0
            &Value::Blue,   //1
            &Value::Green,  //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::Blue,   //5
            &Value::Orange, //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Orange, //9
            &Value::Orange, //10
            &Value::Blue,   //11
            &Value::Green,  //12
            &Value::White,  //13
            &Value::Yellow, //14
            &Value::Blue,   //15
            &Value::Orange, //16
            &Value::White,  //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Yellow, //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::White,  //23
            &Value::Green,  //24
            &Value::Red,    //25
            &Value::Red,    //26
            &Value::Blue,   //27
            &Value::Blue,   //28
            &Value::Green,  //29
            &Value::Red,    //30
            &Value::Green,  //31
            &Value::Red,    //32
            &Value::White,  //33
            &Value::Green,  //34
            &Value::Red,    //35
            &Value::Blue,   //36
            &Value::Blue,   //37
            &Value::Yellow, //38
            &Value::Red,    //39
            &Value::Blue,   //40
            &Value::Red,    //41
            &Value::Red,    //42
            &Value::Orange, //43
            &Value::White,  //44
            &Value::Yellow, //45
            &Value::Green,  //46
            &Value::Green,  //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::Yellow, //50
            &Value::Yellow, //51
            &Value::Yellow, //52
            &Value::Orange, //53
        ];
        let result = right_twist_back(&config);
        (0..54).for_each(|index| {
            println!("{}\t{:?}->{:?}", index, *result[index], *expected[index]);
            assert_eq!(*result[index], *expected[index])
        });
    }
    #[test]
    fn test_bottom_twist_right() {
        let config: ConfigurationRef = [
            &Value::White,
            &Value::Blue,
            &Value::Green,
            &Value::Yellow,
            &Value::White,
            &Value::Yellow,
            &Value::Orange,
            &Value::Red,
            &Value::Orange,
            &Value::Orange,
            &Value::Orange,
            &Value::Blue,
            &Value::Green,
            &Value::White,
            &Value::Green,
            &Value::White,
            &Value::Red,
            &Value::Red,
            &Value::Orange,
            &Value::Orange,
            &Value::Yellow,
            &Value::White,
            &Value::Green,
            &Value::Blue,
            &Value::Orange,
            &Value::Red,
            &Value::Green,
            &Value::Blue,
            &Value::Blue,
            &Value::Green,
            &Value::Red,
            &Value::Green,
            &Value::Orange,
            &Value::Blue,
            &Value::Green,
            &Value::White,
            &Value::Blue,
            &Value::Blue,
            &Value::Yellow,
            &Value::Red,
            &Value::Blue,
            &Value::White,
            &Value::Red,
            &Value::Orange,
            &Value::Red,
            &Value::Yellow,
            &Value::Green,
            &Value::Yellow,
            &Value::White,
            &Value::Yellow,
            &Value::Red,
            &Value::Yellow,
            &Value::Yellow,
            &Value::White,
        ];
        let expected = [
            &Value::White,  //0
            &Value::Blue,   //1
            &Value::Green,  //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::Yellow, //5
            &Value::Orange, //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Orange, //9
            &Value::Orange, //10
            &Value::Blue,   //11
            &Value::Green,  //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::White,  //15
            &Value::Red,    //16
            &Value::Red,    //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Yellow, //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Orange, //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Yellow, //27
            &Value::Green,  //28
            &Value::Yellow, //29
            &Value::Blue,   //30
            &Value::Blue,   //31
            &Value::Green,  //32
            &Value::Red,    //33
            &Value::Green,  //34
            &Value::Orange, //35
            &Value::Red,    //36
            &Value::Red,    //37
            &Value::Blue,   //38
            &Value::Orange, //39
            &Value::Blue,   //40
            &Value::Blue,   //41
            &Value::Red,    //42
            &Value::White,  //43
            &Value::Yellow, //44
            &Value::White,  //45
            &Value::Green,  //46
            &Value::Blue,   //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::Red,    //50
            &Value::Yellow, //51
            &Value::Yellow, //52
            &Value::White,  //53
        ];
        let result = bottom_twist_right(&config);
        (0..54).for_each(|index| {
            println!("{}\t{:?}->{:?}", index, *result[index], *expected[index]);
            assert_eq!(*result[index], *expected[index])
        });
    }
    #[test]
    fn test_bottom_twist_left() {
        let config: ConfigurationRef = [
            &Value::White,  //0
            &Value::Blue,   //1
            &Value::Green,  //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::Yellow, //5
            &Value::Orange, //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Orange, //9
            &Value::Orange, //10
            &Value::Blue,   //11
            &Value::Green,  //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::White,  //15
            &Value::Red,    //16
            &Value::Red,    //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Yellow, //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Orange, //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Blue,   //27
            &Value::Blue,   //28
            &Value::Green,  //29
            &Value::Red,    //30
            &Value::Green,  //31
            &Value::Orange, //32
            &Value::Blue,   //33
            &Value::Green,  //34
            &Value::White,  //35
            &Value::Blue,   //36
            &Value::Blue,   //37
            &Value::Yellow, //38
            &Value::Red,    //39
            &Value::Blue,   //40
            &Value::White,  //41
            &Value::Red,    //42
            &Value::Orange, //43
            &Value::Red,    //44
            &Value::Yellow, //45
            &Value::Green,  //46
            &Value::Yellow, //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::Red,    //50
            &Value::Yellow, //51
            &Value::Yellow, //52
            &Value::White,  //53
        ];
        let expected = [
            &Value::White,  //0
            &Value::Blue,   //1
            &Value::Green,  //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::Yellow, //5
            &Value::Orange, //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Orange, //9
            &Value::Orange, //10
            &Value::Blue,   //11
            &Value::Green,  //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::White,  //15
            &Value::Red,    //16
            &Value::Red,    //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Yellow, //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Orange, //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Red,    //27
            &Value::Green,  //28
            &Value::Orange, //29
            &Value::Blue,   //30
            &Value::Green,  //31
            &Value::White,  //32
            &Value::Yellow, //33
            &Value::Green,  //34
            &Value::Yellow, //35
            &Value::Yellow, //36
            &Value::White,  //37
            &Value::Red,    //38
            &Value::Blue,   //39
            &Value::Blue,   //40
            &Value::Orange, //41
            &Value::Blue,   //42
            &Value::Red,    //43
            &Value::Red,    //44
            &Value::Green,  //45
            &Value::Blue,   //46
            &Value::Blue,   //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::Red,    //50
            &Value::Yellow, //51
            &Value::Yellow, //52
            &Value::White,  //53
        ];
        let result = bottom_twist_left(&config);
        (0..54).for_each(|index| {
            println!("{}\t{:?}->{:?}", index, *result[index], *expected[index]);
            assert_eq!(*result[index], *expected[index])
        });
    }
    #[test]
    fn test_back_twist_right() {
        let config: ConfigurationRef = [
            &Value::White,  //0
            &Value::Blue,   //1
            &Value::Green,  //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::Yellow, //5
            &Value::Orange, //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Orange, //9
            &Value::Orange, //10
            &Value::Blue,   //11
            &Value::Green,  //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::White,  //15
            &Value::Red,    //16
            &Value::Red,    //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Yellow, //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Orange, //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Blue,   //27
            &Value::Blue,   //28
            &Value::Green,  //29
            &Value::Red,    //30
            &Value::Green,  //31
            &Value::Orange, //32
            &Value::Blue,   //33
            &Value::Green,  //34
            &Value::White,  //35
            &Value::Blue,   //36
            &Value::Blue,   //37
            &Value::Yellow, //38
            &Value::Red,    //39
            &Value::Blue,   //40
            &Value::White,  //41
            &Value::Red,    //42
            &Value::Orange, //43
            &Value::Red,    //44
            &Value::Yellow, //45
            &Value::Green,  //46
            &Value::Yellow, //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::Red,    //50
            &Value::Yellow, //51
            &Value::Yellow, //52
            &Value::White,  //53
        ];
        let expected = [
            &Value::Blue,   //0
            &Value::Orange, //1
            &Value::Orange, //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::Yellow, //5
            &Value::Orange, //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Red,    //9
            &Value::Orange, //10
            &Value::Blue,   //11
            &Value::Green,  //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::White,  //15
            &Value::Red,    //16
            &Value::White,  //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Yellow, //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Orange, //24
            &Value::Red,    //25
            &Value::Blue,   //26
            &Value::Red,    //27
            &Value::Blue,   //28
            &Value::Green,  //29
            &Value::Red,    //30
            &Value::Green,  //31
            &Value::Orange, //32
            &Value::Blue,   //33
            &Value::Green,  //34
            &Value::Green,  //35
            &Value::Blue,   //36
            &Value::Blue,   //37
            &Value::Yellow, //38
            &Value::Red,    //39
            &Value::Blue,   //40
            &Value::White,  //41
            &Value::White,  //42
            &Value::Green,  //43
            &Value::Red,    //44
            &Value::Yellow, //45
            &Value::Red,    //46
            &Value::White,  //47
            &Value::Green,  //48
            &Value::Yellow, //49
            &Value::Yellow, //50
            &Value::Yellow, //51
            &Value::White,  //52
            &Value::Yellow, //53
        ];
        let result = back_twist_right(&config);
        (0..54).for_each(|index| {
            println!("{}\t{:?}->{:?}", index, *result[index], *expected[index]);
            assert_eq!(*result[index], *expected[index])
        });
    }
    #[test]
    fn test_back_twist_left() {
        let config: ConfigurationRef = [
            &Value::White,  //0
            &Value::Blue,   //1
            &Value::Green,  //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::Yellow, //5
            &Value::Orange, //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Orange, //9
            &Value::Orange, //10
            &Value::Blue,   //11
            &Value::Green,  //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::White,  //15
            &Value::Red,    //16
            &Value::Red,    //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Yellow, //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Orange, //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Blue,   //27
            &Value::Blue,   //28
            &Value::Green,  //29
            &Value::Red,    //30
            &Value::Green,  //31
            &Value::Orange, //32
            &Value::Blue,   //33
            &Value::Green,  //34
            &Value::White,  //35
            &Value::Blue,   //36
            &Value::Blue,   //37
            &Value::Yellow, //38
            &Value::Red,    //39
            &Value::Blue,   //40
            &Value::White,  //41
            &Value::Red,    //42
            &Value::Orange, //43
            &Value::Red,    //44
            &Value::Yellow, //45
            &Value::Green,  //46
            &Value::Yellow, //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::Red,    //50
            &Value::Yellow, //51
            &Value::Yellow, //52
            &Value::White,  //53
        ];
        let expected = [
            &Value::Red,    //0
            &Value::Green,  //1
            &Value::White,  //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::Yellow, //5
            &Value::Orange, //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Green,  //9
            &Value::Orange, //10
            &Value::Blue,   //11
            &Value::Green,  //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::White,  //15
            &Value::Red,    //16
            &Value::Red,    //17
            &Value::Blue,   //18
            &Value::Orange, //19
            &Value::Yellow, //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Orange, //24
            &Value::Red,    //25
            &Value::Orange, //26
            &Value::White,  //27
            &Value::Blue,   //28
            &Value::Green,  //29
            &Value::Red,    //30
            &Value::Green,  //31
            &Value::Orange, //32
            &Value::Blue,   //33
            &Value::Green,  //34
            &Value::Red,    //35
            &Value::Blue,   //36
            &Value::Blue,   //37
            &Value::Yellow, //38
            &Value::Red,    //39
            &Value::Blue,   //40
            &Value::White,  //41
            &Value::Orange, //42
            &Value::Orange, //43
            &Value::Blue,   //44
            &Value::Yellow, //45
            &Value::White,  //46
            &Value::Yellow, //47
            &Value::Yellow, //48
            &Value::Yellow, //49
            &Value::Green,  //50
            &Value::White,  //51
            &Value::Red,    //52
            &Value::Yellow, //53
        ];
        let result = back_twist_left(&config);
        (0..54).for_each(|index| {
            println!("{}\t{:?}->{:?}", index, *result[index], *expected[index]);
            assert_eq!(*result[index], *expected[index])
        });
    }
    #[test]
    fn test_left_twist_front() {
        let config: ConfigurationRef = [
            &Value::White,  //0
            &Value::Blue,   //1
            &Value::Green,  //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::Yellow, //5
            &Value::Orange, //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Orange, //9
            &Value::Orange, //10
            &Value::Blue,   //11
            &Value::Green,  //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::White,  //15
            &Value::Red,    //16
            &Value::Red,    //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Yellow, //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Orange, //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Blue,   //27
            &Value::Blue,   //28
            &Value::Green,  //29
            &Value::Red,    //30
            &Value::Green,  //31
            &Value::Orange, //32
            &Value::Blue,   //33
            &Value::Green,  //34
            &Value::White,  //35
            &Value::Blue,   //36
            &Value::Blue,   //37
            &Value::Yellow, //38
            &Value::Red,    //39
            &Value::Blue,   //40
            &Value::White,  //41
            &Value::Red,    //42
            &Value::Orange, //43
            &Value::Red,    //44
            &Value::Yellow, //45
            &Value::Green,  //46
            &Value::Yellow, //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::Red,    //50
            &Value::Yellow, //51
            &Value::Yellow, //52
            &Value::White,  //53
        ];
        let expected = [
            &Value::Yellow, //0
            &Value::Blue,   //1
            &Value::Green,  //2
            &Value::White,  //3
            &Value::White,  //4
            &Value::Yellow, //5
            &Value::Yellow, //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Blue,   //9
            &Value::Orange, //10
            &Value::Orange, //11
            &Value::White,  //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::White,  //15
            &Value::Red,    //16
            &Value::Red,    //17
            &Value::Blue,   //18
            &Value::Orange, //19
            &Value::Orange, //20
            &Value::Yellow, //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Orange, //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Green,  //27
            &Value::Yellow, //28
            &Value::Blue,   //29
            &Value::Orange, //30
            &Value::Green,  //31
            &Value::Orange, //32
            &Value::Blue,   //33
            &Value::Green,  //34
            &Value::White,  //35
            &Value::Green,  //36
            &Value::Blue,   //37
            &Value::Yellow, //38
            &Value::White,  //39
            &Value::Blue,   //40
            &Value::White,  //41
            &Value::Red,    //42
            &Value::Orange, //43
            &Value::Red,    //44
            &Value::Blue,   //45
            &Value::Green,  //46
            &Value::Yellow, //47
            &Value::Red,    //48
            &Value::Yellow, //49
            &Value::Red,    //50
            &Value::Red,    //51
            &Value::Yellow, //52
            &Value::White,  //53
        ];
        let result = left_twist_front(&config);
        (0..54).for_each(|index| {
            println!("{}\t{:?}->{:?}", index, *result[index], *expected[index]);
            assert_eq!(*result[index], *expected[index])
        });
    }
    #[test]
    fn test_left_twist_back() {
        let config: ConfigurationRef = [
            &Value::White,  //0
            &Value::Blue,   //1
            &Value::Green,  //2
            &Value::Yellow, //3
            &Value::White,  //4
            &Value::Yellow, //5
            &Value::Orange, //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Orange, //9
            &Value::Orange, //10
            &Value::Blue,   //11
            &Value::Green,  //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::White,  //15
            &Value::Red,    //16
            &Value::Red,    //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Yellow, //20
            &Value::White,  //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Orange, //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Blue,   //27
            &Value::Blue,   //28
            &Value::Green,  //29
            &Value::Red,    //30
            &Value::Green,  //31
            &Value::Orange, //32
            &Value::Blue,   //33
            &Value::Green,  //34
            &Value::White,  //35
            &Value::Blue,   //36
            &Value::Blue,   //37
            &Value::Yellow, //38
            &Value::Red,    //39
            &Value::Blue,   //40
            &Value::White,  //41
            &Value::Red,    //42
            &Value::Orange, //43
            &Value::Red,    //44
            &Value::Yellow, //45
            &Value::Green,  //46
            &Value::Yellow, //47
            &Value::White,  //48
            &Value::Yellow, //49
            &Value::Red,    //50
            &Value::Yellow, //51
            &Value::Yellow, //52
            &Value::White,  //53
        ];
        let expected = [
            &Value::Green,  //0
            &Value::Blue,   //1
            &Value::Green,  //2
            &Value::White,  //3
            &Value::White,  //4
            &Value::Yellow, //5
            &Value::Red,    //6
            &Value::Red,    //7
            &Value::Orange, //8
            &Value::Blue,   //9
            &Value::Yellow, //10
            &Value::Green,  //11
            &Value::Blue,   //12
            &Value::White,  //13
            &Value::Green,  //14
            &Value::White,  //15
            &Value::Red,    //16
            &Value::Red,    //17
            &Value::Orange, //18
            &Value::Orange, //19
            &Value::Blue,   //20
            &Value::Red,    //21
            &Value::Green,  //22
            &Value::Blue,   //23
            &Value::Orange, //24
            &Value::Red,    //25
            &Value::Green,  //26
            &Value::Orange, //27
            &Value::Orange, //28
            &Value::Blue,   //29
            &Value::Red,    //30
            &Value::Green,  //31
            &Value::Orange, //32
            &Value::Blue,   //33
            &Value::Green,  //34
            &Value::White,  //35
            &Value::Yellow, //36
            &Value::Blue,   //37
            &Value::Yellow, //38
            &Value::White,  //39
            &Value::Blue,   //40
            &Value::White,  //41
            &Value::Yellow, //42
            &Value::Orange, //43
            &Value::Red,    //44
            &Value::White,  //45
            &Value::Green,  //46
            &Value::Yellow, //47
            &Value::Yellow, //48
            &Value::Yellow, //49
            &Value::Red,    //50
            &Value::Orange, //51
            &Value::Yellow, //52
            &Value::White,  //53
        ];
        let result = left_twist_back(&config);
        (0..54).for_each(|index| {
            println!("{}\t{:?}->{:?}", index, *result[index], *expected[index]);
            assert_eq!(*result[index], *expected[index])
        });
    }
}

fn main() {
    println!("Hello, world!");
}
