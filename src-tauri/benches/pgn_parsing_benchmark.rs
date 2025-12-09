use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use pgn_reader::{Visitor, BufferedReader};
use shakmaty::{Chess, Position};

// 1. Create a dummy PGN string to test against.
// In a real app, you might load this from a file, but a constant string is faster for micro-benchmarks.
const SAMPLE_PGN: &str = r#"
[Event "F/S Return Match"]
[Site "Belgrade, Serbia JUG"]
[Date "1992.11.04"]
[Round "29"]
[White "Fischer, Robert J."]
[Black "Spassky, Boris V."]
[Result "1/2-1/2"]

1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7 11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5 Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6 23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5 hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5 35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6 Nf2 42. g4 Bd3 43. Re6 1/2-1/2
"#;

// 2. Define a Visitor.
// pgn-reader works by "visiting" events. We need a struct that implements the Visitor trait.
struct BenchmarkVisitor {
    game_count: usize,
    move_count: usize,
}

impl BenchmarkVisitor {
    fn new() -> Self {
        Self { game_count: 0, move_count: 0 }
    }
}

impl Visitor for BenchmarkVisitor {
    type Result = ();

    fn begin_game(&mut self) {
        self.game_count += 1;
    }

    fn san(&mut self, _move_text: &[u8]) {
        // This method is called for every move (e.g., "e4", "Nf3").
        // In a real app, you would parse this into a Shakmaty Move here.
        self.move_count += 1;
    }

    fn end_game(&mut self) -> Self::Result {
        // Game finished
    }
}

// 3. The Function we want to benchmark
fn parse_pgn_logic(pgn_data: &[u8]) {
    let mut visitor = BenchmarkVisitor::new();
    let mut reader = BufferedReader::new_cursor(pgn_data);

    // We expect valid PGN, so we unwrap for the benchmark (or handle errors if you prefer)
    let _ = reader.read_all(&mut visitor);
}

// 4. The Benchmark Configuration
fn benchmark_pgn_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("PGN Parsing");

    // Tell Criterion the size of the data so it can calculate "MB/s" throughput
    group.throughput(Throughput::Bytes(SAMPLE_PGN.len() as u64));

    // RUN THE TEST
    // black_box prevents the compiler from optimizing away the loop if it thinks the result isn't used.
    group.bench_function("parse_single_game", |b| {
        b.iter(|| parse_pgn_logic(black_box(SAMPLE_PGN.as_bytes())))
    });

    group.finish();
}

criterion_group!(benches, benchmark_pgn_parsing);
criterion_main!(benches);