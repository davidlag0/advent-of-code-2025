/*
--- Day 1: Secret Entrance ---
The Elves have good news and bad news.

The good news is that they've discovered project management! This has given them the tools they need to prevent their usual Christmas emergency. For example, they now know that the North Pole decorations need to be finished soon so that other critical tasks can start on time.

The bad news is that they've realized they have a different emergency: according to their resource planning, none of them have any time left to decorate the North Pole!

To save Christmas, the Elves need you to finish decorating the North Pole by December 12th.

Collect stars by solving puzzles. Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You arrive at the secret entrance to the North Pole base ready to start decorating. Unfortunately, the password seems to have been changed, so you can't get in. A document taped to the wall helpfully explains:

"Due to new security protocols, the password is locked in the safe below. Please see the attached document for the new combination."

The safe has a dial with only an arrow on it; around the dial are the numbers 0 through 99 in order. As you turn the dial, it makes a small click noise as it reaches each number.

The attached document (your puzzle input) contains a sequence of rotations, one per line, which tell you how to open the safe. A rotation starts with an L or R which indicates whether the rotation should be to the left (toward lower numbers) or to the right (toward higher numbers). Then, the rotation has a distance value which indicates how many clicks the dial should be rotated in that direction.

So, if the dial were pointing at 11, a rotation of R8 would cause the dial to point at 19. After that, a rotation of L19 would cause it to point at 0.

Because the dial is a circle, turning the dial left from 0 one click makes it point at 99. Similarly, turning the dial right from 99 one click makes it point at 0.

So, if the dial were pointing at 5, a rotation of L10 would cause it to point at 95. After that, a rotation of R5 could cause it to point at 0.

The dial starts by pointing at 50.

You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy. The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.

For example, suppose the attached document contained the following rotations:

L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
Following these rotations would cause the dial to move as follows:

The dial starts by pointing at 50.
The dial is rotated L68 to point at 82.
The dial is rotated L30 to point at 52.
The dial is rotated R48 to point at 0.
The dial is rotated L5 to point at 95.
The dial is rotated R60 to point at 55.
The dial is rotated L55 to point at 0.
The dial is rotated L1 to point at 99.
The dial is rotated L99 to point at 0.
The dial is rotated R14 to point at 14.
The dial is rotated L82 to point at 32.
Because the dial points at 0 a total of three times during this process, the password in this example is 3.

Analyze the rotations in your attached document. What's the actual password to open the door?

--- Part Two ---
You're sure that's the right password, but the door won't open. You knock, but nobody answers. You build a snowman while you think.

As you're rolling the snowballs for your snowman, you find another security document that must have fallen into the snow:

"Due to newer security protocols, please use password method 0x434C49434B until further notice."

You remember from the training seminar that "method 0x434C49434B" means you're actually supposed to count the number of times any click causes the dial to point at 0, regardless of whether it happens during a rotation or at the end of one.

Following the same rotations as in the above example, the dial points at zero a few extra times during its rotations:

The dial starts by pointing at 50.
The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
The dial is rotated L30 to point at 52.
The dial is rotated R48 to point at 0.
The dial is rotated L5 to point at 95.
The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
The dial is rotated L55 to point at 0.
The dial is rotated L1 to point at 99.
The dial is rotated L99 to point at 0.
The dial is rotated R14 to point at 14.
The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
In this example, the dial points at 0 three times at the end of a rotation, plus three more times during a rotation. So, in this example, the new password would be 6.

Be careful: if the dial were pointing at 50, a single rotation like R1000 would cause the dial to point at 0 ten times before returning back to 50!

Using password method 0x434C49434B, what is the password to open the door?
*/

pub struct Dial {
    value: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Self { value: 50 }
    }
}

impl Dial {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Rotates the dial according to the instruction (e.g., "L68" or "R481")
    /// and returns `(final_position, zero_hits)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input string is shorter than 2 characters,
    /// contains an invalid number, or starts with an unknown command character.
    pub fn turn_dial(&mut self, input: &str) -> Result<(i32, u32), String> {
        let input = input.trim();
        if input.len() < 2 {
            return Err("Input string too short".to_string());
        }

        let command = input.chars().next().ok_or("Input string is empty")?;
        let amount: u32 = input[1..]
            .parse()
            .map_err(|_| format!("Invalid number format: '{}'", &input[1..]))?;

        let mut zero_hits: u32 = 0;

        match command {
            'L' => {
                // Moving Left means decreasing numbers.
                // Distance to the next 0 going down is `self.value`.
                // If currently at 0, the next 0 is 100 steps away.
                let dist_to_first_zero: u32 = if self.value == 0 {
                    100
                } else {
                    u32::try_from(self.value).unwrap_or(0)
                };

                if amount >= dist_to_first_zero {
                    zero_hits += 1 + (amount - dist_to_first_zero) / 100;
                }

                let delta = i32::try_from(amount % 100).unwrap_or(0);
                self.value = (self.value - delta).rem_euclid(100);
            }
            'R' => {
                // Moving Right means increasing numbers.
                // Distance to the next 0 going up is `100 - self.value`.
                // If currently at 0, the next 0 is 100 steps away.
                let dist_to_first_zero: u32 = if self.value == 0 {
                    100
                } else {
                    u32::try_from(100 - self.value).unwrap_or(0)
                };

                if amount >= dist_to_first_zero {
                    zero_hits += 1 + (amount - dist_to_first_zero) / 100;
                }

                let delta = i32::try_from(amount % 100).unwrap_or(0);
                self.value = (self.value + delta) % 100;
            }
            _ => return Err(format!("Unknown command char: '{command}'")),
        }

        Ok((self.value, zero_hits))
    }
}

/// Solves Day 1, Part 1 by counting how many rotations leave the dial pointing at 0.
///
/// # Errors
///
/// Returns an error if any line in the puzzle input contains an invalid instruction or format.
pub fn part1(input: &str) -> Result<String, String> {
    let mut dial = Dial::new();
    let mut count = 0;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (pos, _) = dial.turn_dial(line)?;
        if pos == 0 {
            count += 1;
        }
    }

    Ok(count.to_string())
}

/// Solves Day 1, Part 2 by counting every time any click causes the dial to point at 0.
///
/// # Errors
///
/// Returns an error if any line in the puzzle input contains an invalid instruction or format.
pub fn part2(input: &str) -> Result<String, String> {
    let mut dial = Dial::new();
    let mut total_zero_hits = 0;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (_, zero_hits) = dial.turn_dial(line)?;
        total_zero_hits += zero_hits;
    }

    Ok(total_zero_hits.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(3.to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok(6.to_string()));
    }
}
