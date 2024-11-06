use std::collections::HashSet;

pub fn area_occupied() -> i32 {
    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    struct Rectangle {
        a: Point,
        b: Point,
    }

    // Тестові дані
    let rectangles = vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ];

    let mut unique_points: HashSet<(i32, i32)> = HashSet::new();

    for rect in &rectangles {
        for x in rect.a.x..rect.b.x {
            for y in rect.b.y..rect.a.y {
                unique_points.insert((x, y));
            }
        }
    }

    unique_points.len() as i32
}
