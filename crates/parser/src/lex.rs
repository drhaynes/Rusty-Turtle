#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Location {
    col: u32,
    line: u32,
    index: usize,
}

impl Location {
    fn increment(&self, newline: bool) -> Location {
        match newline {
            true => Location {
                col: 0,
                line: self.line + 1,
                index: self.index + 1,
            },
            false => Location {
                col: self.col + 1,
                line: self.line,
                index: self.index + 1,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment() {
        let location = Location {
            col: 0,
            line: 0,
            index: 0,
        };

        let result = location.increment(false);
        assert_eq!(
            result,
            Location {
                col: 1,
                line: 0,
                index: 1
            }
        );
    }

    #[test]
    fn increment_with_newline() {
        let location = Location {
            col: 5,
            line: 0,
            index: 5,
        };

        let result = location.increment(true);

        assert_eq!(
            result,
            Location {
                col: 0,
                line: 1,
                index: 6
            }
        );
    }
}
