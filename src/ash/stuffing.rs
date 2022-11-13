#[derive(Clone)]
pub struct Stuffed(Vec<u8>);

use super::reserved::Reserved;

pub fn stuff(unstuffed: &[u8]) -> Stuffed {
    Stuffed(
        unstuffed
            .iter()
            .copied()
            .flat_map(|x| {
                if Reserved::is_reserved(x) {
                    vec![Reserved::Escape.into(), x ^ 0x10]
                } else {
                    vec![x]
                }
            })
            .collect(),
    )
}

pub fn unstuff(stuffed: Stuffed) -> Vec<u8> {
    let mut escaping = false;
    let mut result = Vec::with_capacity(stuffed.0.len());
    for x in stuffed.0 {
        if x == Reserved::Escape.into() {
            escaping = true;
        } else if !escaping {
            result.push(x);
        } else {
            result.push(x ^ 0x10);
            escaping = false;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ash::reserved::Reserved;

    #[test]
    fn no_op_stuff() {
        let unstuffed = &[7, 9, 13, 0x7a];
        assert!(!unstuffed.iter().any(|x| Reserved::is_reserved(*x)));

        let stuffed = stuff(unstuffed);
        assert_eq!(stuffed.0, unstuffed);
    }

    #[test]
    fn stuff_one() {
        let unstuffed = &[7, 9, Reserved::StopTransmission.into(), 13, 0x7a];
        let stuffed = stuff(unstuffed);
        assert_eq!(&unstuffed[0..2], &stuffed.0[0..2]);
        assert_eq!(0x7d, stuffed.0[2]);
        assert_eq!(unstuffed[2] ^ 0x10, stuffed.0[3]);
        assert_eq!(&unstuffed[3..], &stuffed.0[4..]);
    }

    #[test]
    fn unstuff_no_op() {
        let stuffed = Stuffed(vec![7, 9, 13]);
        let unstuffed = unstuff(stuffed.clone());
        assert_eq!(stuffed.0, unstuffed);
    }

    #[test]
    fn unstuff_one() {
        let stuffed = Stuffed(vec![7, 9, Reserved::Escape.into(), 55 ^ 0x10, 13]);
        let unstuffed = unstuff(stuffed.clone());
        assert_eq!(&stuffed.0[0..2], &unstuffed[0..2]);
        assert_eq!(55, unstuffed[2]);
        assert_eq!(&stuffed.0[4..], &unstuffed[3..]);
    }

    #[test]
    fn stuff_and_unstuff() {
        let unstuffed1 : Vec<u8> = (0..512).map(|x| (x & 0xff) as u8).collect();
        let stuffed = stuff(&unstuffed1);
        let unstuffed2 = unstuff(stuffed);
        assert_eq!(unstuffed1, unstuffed2);
    }
}
