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
}

fn main() {
    println!("Hello, world!");
}