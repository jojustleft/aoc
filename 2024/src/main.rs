mod day01;
//mod day02;
//mod day03;
//mod day04;
//mod day05;

fn main() {
    day01::historian("./../inputs/input_1_1.txt", true);
    day01::historian("./../inputs/input_1_test.txt", true);
    day01::historian_p2("./../inputs/input_1_2.txt", true);
    day01::historian_p2("./../inputs/input_1_test.txt", true);

    //day02::reports("inputs/input_2_1.txt", true);
    //day03::salvage("inputs/input_3_1.txt", true);
    //day04::word_search("inputs/input_4_1.txt", true);
    //day05::rule_check("inputs/input_5_1.txt", true);
}
