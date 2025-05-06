fn navigate_and_compute_area(moves: &str) -> i32 {
    let mut path: Vec<(i32, i32)> = Vec::new();
    let (mut x, mut y) = (0, 0);
    path.push((x, y));

    for ch in moves.chars() {
        match ch {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => continue,
        }
        path.push((x, y));
    }

    // Must return to origin and form a polygon (at least 3 edges => 4 points)
    if path.first() != path.last() || path.len() < 4 {
        return -1;
    }

    // Shoelace formula
    let mut area = 0;
    for i in 0..path.len() - 1 {
        let (x0, y0) = path[i];
        let (x1, y1) = path[i + 1];
        area += x0 * y1 - x1 * y0;
    }

    area.abs() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_loop() {
        assert_eq!(navigate_and_compute_area("RDLU"), 1);
        assert_eq!(navigate_and_compute_area("DRUL"), 1);
    }

    #[test]
    fn opened_loop() {
        assert_eq!(navigate_and_compute_area("RDL"), -1);
        assert_eq!(navigate_and_compute_area("DRU"), -1);
    }
}
