pub(crate) fn pack_u32(data: &[(u8, usize)]) -> u32 {
    let (result, _) = data.iter().fold((0, 0), |(v_acc, shift_acc), &(v, shift)| {
        (
            v_acc | (([v as u32, (1 << shift) - 1][(v > (1 << shift) - 1) as usize]) << shift_acc),
            shift_acc + shift,
        )
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_u32() {
        assert_eq!(
            pack_u32(&[(31, 5), (31, 5), (31, 5), (0, 5), (0, 5), (31, 5), (0, 1), (1, 1)]),
            0b10_11111_00000_00000_11111_11111_11111
        );
    }
}
