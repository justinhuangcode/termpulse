//! Benchmarks for termpulse-core hot paths.
//!
//! Run with: cargo bench -p termpulse-core

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use termpulse_core::{
    OscSequence, ProgressState, Terminator, find_sequences, sanitize_label, strip_sequences,
};

fn bench_write_to(c: &mut Criterion) {
    let seq = OscSequence {
        state: ProgressState::Normal,
        percent: Some(50),
        label: Some("Building project"),
        terminator: Terminator::St,
    };

    c.bench_function("OscSequence::write_to", |b| {
        let mut buf = [0u8; 256];
        b.iter(|| {
            black_box(seq.write_to(&mut buf).unwrap());
        });
    });
}

fn bench_write_to_no_label(c: &mut Criterion) {
    let seq = OscSequence {
        state: ProgressState::Normal,
        percent: Some(50),
        label: None,
        terminator: Terminator::St,
    };

    c.bench_function("OscSequence::write_to (no label)", |b| {
        let mut buf = [0u8; 256];
        b.iter(|| {
            black_box(seq.write_to(&mut buf).unwrap());
        });
    });
}

fn bench_sanitize_clean(c: &mut Criterion) {
    c.bench_function("sanitize_label (clean)", |b| {
        b.iter(|| {
            black_box(sanitize_label(black_box("Building project v1.2.3")));
        });
    });
}

fn bench_sanitize_dirty(c: &mut Criterion) {
    c.bench_function("sanitize_label (dirty)", |b| {
        b.iter(|| {
            black_box(sanitize_label(black_box("evil\x1b]inject\x07payload")));
        });
    });
}

fn bench_find_sequences(c: &mut Criterion) {
    let input = b"prefix\x1b]9;4;1;50;Building\x1b\\middle\x1b]9;4;0;0;\x1b\\suffix";

    c.bench_function("find_sequences (2 seqs)", |b| {
        b.iter(|| {
            let mut out = [termpulse_core::ParsedSequence::EMPTY; 8];
            black_box(find_sequences(black_box(input), &mut out));
        });
    });
}

fn bench_strip_sequences(c: &mut Criterion) {
    let input = b"prefix\x1b]9;4;1;50;Building\x1b\\suffix";

    c.bench_function("strip_sequences", |b| {
        let mut out = [0u8; 256];
        b.iter(|| {
            black_box(strip_sequences(black_box(input), &mut out));
        });
    });
}

criterion_group!(
    benches,
    bench_write_to,
    bench_write_to_no_label,
    bench_sanitize_clean,
    bench_sanitize_dirty,
    bench_find_sequences,
    bench_strip_sequences,
);
criterion_main!(benches);
