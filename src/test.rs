#[test]
fn sequence()
{
    use super::Kolakoski;

    // source: https://oeis.org/A000002
    let expected: [u8; 100] = [
        1, 2, 2, 1, 1, 2, 1, 2, 2, 1,
        2, 2, 1, 1, 2, 1, 1, 2, 2, 1,
        2, 1, 1, 2, 1, 2, 2, 1, 1, 2,
        1, 1, 2, 1, 2, 2, 1, 2, 2, 1,
        1, 2, 1, 2, 2, 1, 2, 1, 1, 2,
        1, 1, 2, 2, 1, 2, 2, 1, 1, 2,
        1, 2, 2, 1, 2, 2, 1, 1, 2, 1,
        1, 2, 1, 2, 2, 1, 2, 1, 1, 2,
        2, 1, 2, 2, 1, 1, 2, 1, 2, 2,
        1, 2, 2, 1, 1, 2, 1, 1, 2, 2];

    for (i, n) in Kolakoski::new().take(100)
        .enumerate()
    {
        assert!(expected[i] == n,
            "number {} was {}, expected {}", i + 1, n, expected[i]);
    }

    #[cfg(feature = "num-traits")]
    sequence_num();
}

#[cfg(feature = "num-traits")]
fn sequence_num()
{
    use super::KolakoskiNum;

    // source: https://oeis.org/A000002
    let expected: [u8; 100] = [
        1, 2, 2, 1, 1, 2, 1, 2, 2, 1,
        2, 2, 1, 1, 2, 1, 1, 2, 2, 1,
        2, 1, 1, 2, 1, 2, 2, 1, 1, 2,
        1, 1, 2, 1, 2, 2, 1, 2, 2, 1,
        1, 2, 1, 2, 2, 1, 2, 1, 1, 2,
        1, 1, 2, 2, 1, 2, 2, 1, 1, 2,
        1, 2, 2, 1, 2, 2, 1, 1, 2, 1,
        1, 2, 1, 2, 2, 1, 2, 1, 1, 2,
        2, 1, 2, 2, 1, 1, 2, 1, 2, 2,
        1, 2, 2, 1, 1, 2, 1, 1, 2, 2];

    for (i, n) in KolakoskiNum::<u8>::new().take(100)
        .enumerate()
    {
        assert!(expected[i] == n,
            "number {} was {}, expected {}", i + 1, n, expected[i]);
    }
}