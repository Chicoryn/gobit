use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gobit::{Color, Goban};

const BYUN_SANGIL_TANG_WEIXING: [(char, (u8, u8)); 262] = [
    ('B', (16, 3)), ('W', (15, 15)), ('B', (3, 2)), ('W', (2, 15)), ('B', (2, 4)), ('W', (14, 2)), ('B', (15, 4)),
    ('W', (11, 2)), ('B', (16, 13)), ('W', (16, 11)), ('B', (16, 8)), ('W', (13, 16)), ('B', (4, 16)), ('W', (6, 15)),
    ('B', (3, 14)), ('W', (3, 15)), ('B', (4, 15)), ('W', (4, 14)), ('B', (2, 14)), ('W', (1, 14)), ('B', (1, 13)),
    ('W', (1, 15)), ('B', (5, 14)), ('W', (4, 13)), ('B', (3, 12)), ('W', (4, 12)), ('B', (3, 11)), ('W', (4, 11)),
    ('B', (3, 10)), ('W', (4, 10)), ('B', (3, 9)), ('W', (6, 16)), ('B', (2, 17)), ('W', (1, 17)), ('B', (4, 9)),
    ('W', (3, 17)), ('B', (6, 9)), ('W', (7, 10)), ('B', (6, 10)), ('W', (7, 12)), ('B', (9, 2)), ('W', (4, 2)),
    ('B', (4, 3)), ('W', (15, 9)), ('B', (15, 8)), ('W', (13, 9)), ('B', (9, 4)), ('W', (5, 3)), ('B', (5, 2)),
    ('W', (4, 1)), ('B', (5, 1)), ('W', (16, 14)), ('B', (8, 14)), ('W', (6, 11)), ('B', (9, 16)), ('W', (10, 15)),
    ('B', (4, 17)), ('W', (3, 16)), ('B', (6, 14)), ('W', (7, 15)), ('B', (11, 16)), ('W', (11, 15)), ('B', (12, 16)),
    ('W', (12, 15)), ('B', (3, 18)), ('W', (7, 14)), ('B', (1, 18)), ('W', (0, 13)), ('B', (1, 12)), ('W', (0, 17)),
    ('B', (13, 17)), ('W', (14, 16)), ('B', (14, 17)), ('W', (15, 17)), ('B', (7, 17)), ('W', (6, 17)), ('B', (8, 16)),
    ('W', (7, 9)), ('B', (7, 8)), ('W', (8, 8)), ('B', (8, 7)), ('W', (9, 8)), ('B', (7, 7)), ('W', (8, 15)),
    ('B', (17, 10)), ('W', (17, 11)), ('B', (16, 10)), ('W', (15, 10)), ('B', (15, 11)), ('W', (17, 13)),
    ('B', (14, 8)), ('W', (12, 9)), ('B', (15, 2)), ('W', (3, 1)), ('B', (3, 3)), ('W', (2, 2)), ('B', (1, 2)),
    ('W', (14, 18)), ('B', (7, 18)), ('W', (6, 18)), ('B', (12, 17)), ('W', (1, 1)), ('B', (1, 3)), ('W', (9, 1)),
    ('B', (8, 1)), ('W', (8, 2)), ('B', (8, 3)), ('W', (7, 2)), ('B', (7, 1)), ('W', (10, 2)), ('B', (9, 3)),
    ('W', (14, 1)), ('B', (0, 14)), ('W', (0, 15)), ('B', (15, 12)), ('W', (15, 13)), ('B', (14, 9)), ('W', (14, 10)),
    ('B', (13, 10)), ('W', (14, 11)), ('B', (10, 1)), ('W', (11, 1)), ('B', (9, 0)), ('W', (16, 1)), ('B', (15, 1)),
    ('W', (15, 0)), ('B', (16, 2)), ('W', (17, 1)), ('B', (17, 2)), ('W', (10, 16)), ('B', (10, 17)), ('W', (13, 18)),
    ('B', (12, 18)), ('W', (15, 18)), ('B', (9, 17)), ('W', (13, 4)), ('B', (9, 15)), ('W', (9, 14)), ('B', (12, 7)),
    ('W', (11, 8)), ('B', (12, 3)), ('W', (11, 4)), ('B', (12, 4)), ('W', (12, 5)), ('B', (13, 5)), ('W', (13, 6)),
    ('B', (13, 3)), ('W', (14, 5)), ('B', (14, 3)), ('W', (16, 6)), ('B', (15, 6)), ('W', (15, 5)), ('B', (16, 5)),
    ('W', (17, 5)), ('B', (16, 4)), ('W', (17, 6)), ('B', (15, 7)), ('W', (17, 8)), ('B', (16, 9)), ('W', (14, 6)),
    ('B', (14, 12)), ('W', (13, 11)), ('B', (12, 10)), ('W', (11, 10)), ('B', (10, 14)), ('W', (9, 13)), ('B', (1, 16)),
    ('W', (11, 6)), ('B', (11, 3)), ('W', (4, 18)), ('B', (18, 11)), ('W', (16, 12)), ('B', (17, 7)), ('W', (16, 7)),
    ('B', (18, 4)), ('W', (10, 3)), ('B', (10, 4)), ('W', (17, 4)), ('B', (18, 3)), ('W', (18, 1)), ('B', (11, 5)),
    ('W', (18, 7)), ('B', (13, 7)), ('W', (10, 5)), ('B', (12, 6)), ('W', (11, 4)), ('B', (11, 7)), ('W', (6, 1)),
    ('B', (6, 3)), ('W', (18, 2)), ('B', (17, 3)), ('W', (18, 5)), ('B', (14, 7)), ('W', (10, 7)), ('B', (11, 5)),
    ('W', (6, 2)), ('B', (5, 4)), ('W', (11, 4)), ('B', (14, 4)), ('W', (12, 2)), ('B', (13, 0)), ('W', (12, 0)),
    ('B', (18, 9)), ('W', (18, 8)), ('B', (11, 0)), ('W', (10, 0)), ('B', (12, 11)), ('W', (13, 12)), ('B', (11, 0)),
    ('W', (5, 0)), ('B', (5, 3)), ('W', (10, 0)), ('B', (12, 12)), ('W', (14, 13)), ('B', (11, 0)), ('W', (11, 17)),
    ('B', (11, 18)), ('W', (10, 0)), ('B', (8, 11)), ('W', (9, 1)), ('B', (7, 11)), ('W', (8, 0)), ('B', (6, 12)),
    ('W', (10, 12)), ('B', (10, 9)), ('W', (10, 10)), ('B', (9, 10)), ('W', (9, 9)), ('B', (8, 13)), ('W', (6, 13)),
    ('B', (9, 12)), ('W', (11, 12)), ('B', (10, 13)), ('W', (11, 13)), ('B', (7, 13)), ('W', (5, 13)), ('B', (5, 15)),
    ('W', (2, 18)), ('B', (11, 14)), ('W', (12, 14)), ('B', (3, 18)), ('W', (5, 18)), ('B', (7, 16)), ('W', (2, 18)),
    ('B', (11, 5)), ('W', (10, 6)), ('B', (3, 18)), ('W', (2, 16)), ('B', (5, 11)), ('W', (2, 18)), ('B', (0, 1)),
    ('W', (2, 1)), ('B', (8, 9)), ('W', (0, 12)), ('B', (0, 11)), ('W', (0, 14)), ('B', (1, 11)), ('W', (1, 0)),
    ('B', (13, 5)), ('W', (18, 12)), ('B', (7, 3)), ('W', (7, 0))
];

fn features(moves: &[(char, (u8, u8))]) -> (Goban, usize) {
    let mut goban = Goban::new(19, 19);
    let mut count = 0;

    for (color, (x, y)) in moves {
        let at = (*x, *y).into();
        let color = match color {
            'B' => Color::Black,
            'W' => Color::White,
            _ => unreachable!(),
        };

        for point in goban.iter() {
            if goban.is_valid(point, color) {
                count += 1;
            }
        }

        goban.play(at, color);
    }

    (goban, count)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("byun_sangil_tang_weixing", |b| b.iter(|| features(black_box(&BYUN_SANGIL_TANG_WEIXING))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
