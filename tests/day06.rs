use advent_2022::day06::solve_a;

#[test]
fn test_solve_a() {
    assert_eq!(solve_a("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(solve_a("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(solve_a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(solve_a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}
