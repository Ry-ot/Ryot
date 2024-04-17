use crate::prelude::*;
use ryot_grid::prelude::*;

#[test]
#[ignore]
#[cfg(feature = "tiled")]
fn stress_test_path_finding() {
    use time_test::time_test;

    fn format_number(num: usize) -> String {
        if num >= 1_000_000 {
            format!("{:.1}M", num / 1_000_000)
        } else if num >= 1_000 {
            format!("{:.1}k", num / 1_000)
        } else {
            format!("{}", num)
        }
    }

    let scenarios = [
        (2, 3_000_000usize),
        (3, 1_700_000usize),
        (5, 800_000usize),
        (10, 120_000usize),
        (15, 50_000usize),
        (20, 25_000usize),
        (50, 2_500usize),
        (75, 800usize),
        (100, 400usize),
    ];

    for (distance, iterations) in scenarios.iter() {
        time_test!(format!(
            "\n{} iterations of path finding of distance {}",
            format_number(*iterations),
            distance,
        ));

        for _ in 0..(*iterations) {
            let from = TilePosition::new(rand::random::<i32>(), rand::random::<i32>(), 0);
            let to = TilePosition::new(from.x + *distance, from.y + *distance, 0);

            if from.path_to(to, |_| true, None).is_none() {
                panic!("Path finding failed");
            }
        }
    }
}
