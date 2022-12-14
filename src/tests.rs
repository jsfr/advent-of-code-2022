#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use anyhow::Context;

    use crate::solution::Solution;

    fn read_input(day: &str) -> String {
        let file = format!("./input/{}", day);

        read_to_string(&file)
            .context(format!("Failed to read {file}"))
            .unwrap()
    }

    #[test]
    fn day_01() {
        let input = read_input("01");
        let day = crate::day_01::Day {};

        let answer = "74711";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "209481";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_02() {
        let input = read_input("02");
        let day = crate::day_02::Day {};

        let answer = "12740";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "11980";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_03() {
        let input = read_input("03");
        let day = crate::day_03::Day {};

        let answer = "8176";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "2689";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_04() {
        let input = read_input("04");
        let day = crate::day_04::Day {};

        let answer = "500";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "815";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_05() {
        let input = read_input("05");
        let day = crate::day_05::Day {};

        let answer = "TDCHVHJTG";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "NGCMPJLHV";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_06() {
        let input = read_input("06");
        let day = crate::day_06::Day {};

        let answer = "1282";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "3513";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_07() {
        let input = read_input("07");
        let day = crate::day_07::Day {};

        let answer = "1367870";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "549173";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_08() {
        let input = read_input("08");
        let day = crate::day_08::Day {};

        let answer = "1679";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "536625";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_09() {
        let input = read_input("09");
        let day = crate::day_09::Day {};

        let answer = "5981";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "2352";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_10() {
        let input = read_input("10");
        let day = crate::day_10::Day {};

        let answer = "17180";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "
■■■  ■■■■ ■  ■ ■■■  ■■■  ■    ■  ■ ■■■  
■  ■ ■    ■  ■ ■  ■ ■  ■ ■    ■  ■ ■  ■ 
■  ■ ■■■  ■■■■ ■  ■ ■  ■ ■    ■  ■ ■■■  
■■■  ■    ■  ■ ■■■  ■■■  ■    ■  ■ ■  ■ 
■ ■  ■    ■  ■ ■    ■ ■  ■    ■  ■ ■  ■ 
■  ■ ■■■■ ■  ■ ■    ■  ■ ■■■■  ■■  ■■■  ";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_11() {
        let input = read_input("11");
        let day = crate::day_11::Day {};

        let answer = "66124";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "19309892877";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_13() {
        let input = read_input("13");
        let day = crate::day_13::Day {};

        let answer = "5393";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "26712";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_14() {
        let input = read_input("14");
        let day = crate::day_14::Day {};

        let answer = "698";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }
}
