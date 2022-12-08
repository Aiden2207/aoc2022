pub fn part1(input: &str) -> usize {
    let mut lines = input
        .lines()
        .map(|s| {
            s.chars()
                .flat_map(|n| n.to_digit(10))
                .map(|n| (n as i32, false))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for row in lines.iter_mut() {
        visible(row.iter_mut());
        visible(row.iter_mut().rev());
    }
    for col in 0..lines[0].len() {
        visible(lines.iter_mut().map(|row| &mut row[col]));
        visible(lines.iter_mut().rev().map(|row| &mut row[col]));
    }

    lines.into_iter().flatten().filter(|(_, b)| *b).count()
}

pub fn part2(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|s| {
            s.chars()
                .flat_map(|n| n.to_digit(10))
                .map(|n| n as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut max = 0;
    for (i, row) in Rows::new(&lines).enumerate() {
        for (j, col) in Columns::new(&lines).enumerate() {
            if i == 0 || j == 0 || i == lines.len() - 1 || j == lines[0].len() - 1 {
                continue;
            }
            let mut score = 1;
            score *= visible_count(row[j..].iter().copied());
            score *= visible_count(row[..=j].iter().rev().copied());
            score *= visible_count(col[i..].iter().copied().copied());
            score *= visible_count(col[..=i].iter().rev().copied().copied());

            max = max.max(score);
        }
    }
    max
}

fn visible<'a>(iter: impl IntoIterator<Item = &'a mut (i32, bool)>) {
    iter.into_iter().fold(-1, |max_height, height| {
        if max_height < height.0 {
            height.1 = true;
            height.0
        } else {
            max_height
        }
    });
}
fn visible_count(iter: impl IntoIterator<Item = i32> + Clone) -> usize {
    let mut iter = iter.into_iter();
    let Some(first) = iter.next() else { return 0 };
    let mut count = 0;
    for i in iter {
        count += 1;
        if i >= first {
            break;
        }
    }
    count
}

//double ended iterator over rows of a borrowed 2d vector
struct Rows<'a, T> {
    vec: &'a Vec<Vec<T>>,
    index: usize,
}
impl<'a, T> Rows<'a, T> {
    fn new(vec: &'a Vec<Vec<T>>) -> Self {
        Self { vec, index: 0 }
    }
}
impl<'a, T> Iterator for Rows<'a, T> {
    type Item = &'a Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let index = self.index;
            self.index += 1;
            Some(&self.vec[index])
        } else {
            None
        }
    }
}

//double ended iterator over columns of a borrowed 2d vector
struct Columns<'a, T> {
    vec: &'a Vec<Vec<T>>,
    index: usize,
}
impl<'a, T> Columns<'a, T> {
    fn new(vec: &'a Vec<Vec<T>>) -> Self {
        Self { vec, index: 0 }
    }
}
impl<'a, T> Iterator for Columns<'a, T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec[0].len() {
            let index = self.index;
            self.index += 1;
            Some(self.vec.iter().map(|row| &row[index]).collect())
        } else {
            None
        }
    }
}
