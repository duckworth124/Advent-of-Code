use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Permutation([usize; 7]);

#[derive(Clone, Copy)]
struct PartialPermutation([Option<usize>; 7]);

impl PartialPermutation {
    fn to_permutation(self) -> Option<Permutation> {
        let permutation = self
            .0
            .into_iter()
            .collect::<Option<Vec<_>>>()?
            .try_into()
            .unwrap();

        Some(Permutation(permutation))
    }

    fn is_all_unique(&self) -> bool {
        self.0.iter().flatten().all_unique()
    }
}

#[derive(Clone, Copy)]
struct PartialSevenSegmentDigit([Option<bool>; 7]);

impl PartialSevenSegmentDigit {
    fn could_be_a_digit(&self) -> bool {
        VALID_DIGITS.iter().any(|d| d.could_be_equal(*self))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct SevenSegmentDigit([bool; 7]);

impl SevenSegmentDigit {
    fn new(input: &str) -> Self {
        let mut segments = [false; 7];
        for c in input.chars() {
            let index = c as usize - 'a' as usize;
            segments[index] = true;
        }
        SevenSegmentDigit(segments)
    }
    fn value(&self) -> usize {
        VALID_DIGITS.iter().position(|d| d == self).unwrap()
    }

    fn count_segments(&self) -> usize {
        self.0.iter().filter(|b| **b).count()
    }

    fn could_be_equal(&self, partial_digit: PartialSevenSegmentDigit) -> bool {
        partial_digit
            .0
            .into_iter()
            .zip(self.0)
            .flat_map(|(b1, b2)| Some(b1? == b2))
            .all(|b| b)
    }

    fn permute(&self, permutation: Permutation) -> Self {
        let mut digits = [false; 7];
        for i in 0..7 {
            let j = permutation.0[i];
            digits[j] = self.0[i];
        }

        Self(digits)
    }

    fn partial_permute(&self, permutation: PartialPermutation) -> PartialSevenSegmentDigit {
        let mut digits = [None; 7];
        for i in 0..7 {
            if let Some(j) = permutation.0[i] {
                digits[j] = Some(self.0[i])
            }
        }

        PartialSevenSegmentDigit(digits)
    }
}

const ZERO: SevenSegmentDigit = SevenSegmentDigit([true, true, true, false, true, true, true]);
const ONE: SevenSegmentDigit = SevenSegmentDigit([false, false, true, false, false, true, false]);
const TWO: SevenSegmentDigit = SevenSegmentDigit([true, false, true, true, true, false, true]);
const THREE: SevenSegmentDigit = SevenSegmentDigit([true, false, true, true, false, true, true]);
const FOUR: SevenSegmentDigit = SevenSegmentDigit([false, true, true, true, false, true, false]);
const FIVE: SevenSegmentDigit = SevenSegmentDigit([true, true, false, true, false, true, true]);
const SIX: SevenSegmentDigit = SevenSegmentDigit([true, true, false, true, true, true, true]);
const SEVEN: SevenSegmentDigit = SevenSegmentDigit([true, false, true, false, false, true, false]);
const EIGHT: SevenSegmentDigit = SevenSegmentDigit([true, true, true, true, true, true, true]);
const NINE: SevenSegmentDigit = SevenSegmentDigit([true, true, true, true, false, true, true]);

const VALID_DIGITS: [SevenSegmentDigit; 10] =
    [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

struct SevenSegmentDisplay {
    previous_displays: [SevenSegmentDigit; 10],
    current_display: [SevenSegmentDigit; 4],
}

impl SevenSegmentDisplay {
    fn new(input: &str) -> Self {
        let start = input.chars().take_while(|c| *c != '|').collect::<String>();

        let end = input
            .chars()
            .skip_while(|c| *c != '|')
            .skip(1)
            .collect::<String>();

        let previous_displays = start
            .split_whitespace()
            .map(SevenSegmentDigit::new)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let current_display = end
            .split_whitespace()
            .map(SevenSegmentDigit::new)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            previous_displays,
            current_display,
        }
    }

    fn count_digits_with_unique_segment_numbers(&self) -> usize {
        self.current_display
            .iter()
            .filter(|d| [2, 3, 4, 7].contains(&d.count_segments()))
            .count()
    }

    fn is_valid(&self, permutation: PartialPermutation) -> bool {
        permutation.is_all_unique()
            && self
                .previous_displays
                .iter()
                .all(|d| d.partial_permute(permutation).could_be_a_digit())
    }

    fn get_permutation(&self) -> Permutation {
        let mut current_permutation = PartialPermutation([None; 7]);
        let mut current_segment = 0;
        loop {
            let new_value = current_permutation.0[current_segment]
                .map(|x| x + 1)
                .unwrap_or(0);

            if new_value == 7 {
                current_permutation.0[current_segment] = None;
                current_segment = current_segment.checked_sub(1).unwrap();
                continue;
            }

            current_permutation.0[current_segment] = Some(new_value);
            if !self.is_valid(current_permutation) {
                continue;
            }

            if let Some(permutation) = current_permutation.to_permutation() {
                return permutation;
            }

            current_segment += 1;
        }
    }

    fn find_current_number(&self) -> u32 {
        let permutation = self.get_permutation();
        let digits = self
            .current_display
            .map(|d| d.permute(permutation).value().to_string());
        digits.concat().parse().unwrap()
    }
}

struct Displays(Vec<SevenSegmentDisplay>);

impl Displays {
    fn new(input: &str) -> Self {
        let displays = input.lines().map(SevenSegmentDisplay::new).collect();
        Displays(displays)
    }

    fn count_digits_with_unique_segment_numbers(&self) -> usize {
        self.0
            .iter()
            .map(|display| display.count_digits_with_unique_segment_numbers())
            .sum()
    }

    fn sum_of_current_numbers(&self) -> u32 {
        self.0.iter().map(|d| d.find_current_number()).sum()
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let displays = Displays::new(&input);
    let output_1 = displays.count_digits_with_unique_segment_numbers();
    let output_2 = displays.sum_of_current_numbers();

    println!("part 1: {output_1} part 2: {output_2}")
}
