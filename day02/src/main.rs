use common::files;

fn main() {
    let boxes: Vec<Box> = files::get_file_lines("day02.txt").iter()
        .map(|l| parse_box(l))
        .collect();;

    let wrapping_paper_sqft: i32 = boxes.iter()
        .map(|b| wrapping_paper_needed(b))
        .sum();

    let ribbon_sqft: i32 = boxes.iter()
        .map(|b| ribbon_needed(b.l, b.w, b.h))
        .sum();

    println!("{}", wrapping_paper_sqft);
    println!("{}", ribbon_sqft);
}

struct Box {
    l: i32,
    w: i32,
    h: i32,
}

fn parse_box(line: &String) -> Box {
    let vec: Vec<i32> = line.split('x')
        .map(|x| x.parse::<i32>().expect("Failed to parse line."))
        .collect();

    Box {
        l: vec[0],
        w: vec[1],
        h: vec[2],
    }
}

fn wrapping_paper_needed(b: &Box) -> i32 {
    (2 * b.l * b.w) + (2 * b.w * b.h) + (2 * b.h * b.l) + slack_needed(b.l, b.w, b.h)
}

fn slack_needed(l: i32, w: i32, h: i32) -> i32 {
    if l >= w && l >= h {
        return w * h;
    } else if w >= l && w >= h {
        return l * h;
    } else {
        return l * w;
    }
}

fn ribbon_needed(l: i32, w: i32, h: i32) -> i32 {
    if l >= w && l >= h {
        return 2 * w + 2 * h + l * w * h;
    } else if w >= l && w >= h {
        return 2 * l + 2 * h+ l * w * h;
    } else {
        return 2 * l + 2 * w+ l * w * h;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapping_paper_needed() {
        assert_eq!(wrapping_paper_needed(&Box { l: 2, w: 3, h: 4 }), 58);
        assert_eq!(wrapping_paper_needed(&Box { l: 1, w: 1, h: 10 }), 43);
    }

    #[test]
    fn test_slack_needed() {
        assert_eq!(slack_needed(2, 3, 4), 6);
        assert_eq!(slack_needed(1, 1, 10), 1);
        assert_eq!(slack_needed(2, 2, 2), 4);
    }

    #[test]
    fn test_ribbon_needed() {
        assert_eq!(ribbon_needed(2, 3, 4), 34);
        assert_eq!(ribbon_needed(1, 1, 10), 14);
    }
}