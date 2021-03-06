use libwfa::{affine_wavefront::*, bindings::*, mm_allocator::*, penalties::*};

fn new_complete<'a>(
    alloc: &'a MMAllocator,
    pattern_len: usize,
    text_len: usize,
    penalties: &mut AffinePenalties,
) -> AffineWavefronts<'a> {
    AffineWavefronts::new_complete(pattern_len, text_len, penalties, alloc)
}

fn run_complete<'a>(
    alloc: &'a MMAllocator,
    pattern_len: usize,
    text_len: usize,
    pattern: &str,
    text: &str,
    penalties: &mut AffinePenalties,
) -> Result<AffineWavefronts<'a>, WavefrontError> {
    let mut wavefronts =
        AffineWavefronts::new_complete(pattern_len, text_len, penalties, alloc);

    wavefronts.align(pattern.as_bytes(), text.as_bytes())?;

    Ok(wavefronts)
}

#[test]
#[should_panic]
fn empty_texts() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let pattern = String::from("");
    let text = String::from("");

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    let result = run_complete(&alloc, 0, 0, &pattern, &text, &mut penalties);
    assert!(result.is_ok());
}

#[test]
fn empty_wavefronts() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    let pattern = String::from("");
    let text = String::from("");

    let mut wavefronts = new_complete(
        &alloc,
        pattern.len() + 10,
        text.len() + 10,
        &mut penalties,
    );

    let result = wavefronts.align(pattern.as_bytes(), text.as_bytes());

    assert!(result.is_ok());

    let score = wavefronts.edit_cigar_score(&mut penalties);
    assert_eq!(score, 0);

    let cigar = wavefronts.cigar_bytes_raw();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    assert_eq!("", cg_str);
}

#[test]
fn longer_texts() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let pattern = String::from("TCTTTACTCGCGCGTTGGAGAAATACAATAGT");
    let text = String::from("TCTATACTGCGCGTTTGGAGAAATAAAATAGT");

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    let result = run_complete(&alloc, 10, 10, &pattern, &text, &mut penalties);

    assert!(result.is_err());
}

#[test]
fn shorter_texts() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let pattern = String::from("TCTTTACTCGCGCGTTGGAGAAATACAATAGT");
    let text = String::from("TCTATACTGCGCGTTTGGAGAAATAAAATAGT");

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    let mut wavefronts = new_complete(
        &alloc,
        pattern.len() + 10,
        text.len() + 10,
        &mut penalties,
    );

    let result = wavefronts.align(pattern.as_bytes(), text.as_bytes());

    assert!(result.is_ok());

    let score = wavefronts.edit_cigar_score(&mut penalties);
    assert_eq!(score, -24);

    let cigar = wavefronts.cigar_bytes_raw();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    assert_eq!("MMMXMMMMDMMMMMMMIMMMMMMMMMXMMMMMM", cg_str);
}

#[test]
fn wavefronts_complete_align() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let pattern = String::from("TCTTTACTCGCGCGTTGGAGAAATACAATAGT");
    let text = String::from("TCTATACTGCGCGTTTGGAGAAATAAAATAGT");

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    let pat_len = pattern.as_bytes().len();
    let text_len = text.as_bytes().len();

    let mut wavefronts = AffineWavefronts::new_complete(
        pat_len,
        text_len,
        &mut penalties,
        &alloc,
    );

    let result = wavefronts.align(pattern.as_bytes(), text.as_bytes());

    assert!(result.is_ok());

    let score = wavefronts.edit_cigar_score(&mut penalties);
    assert_eq!(score, -24);

    let cigar = wavefronts.cigar_bytes_raw();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    assert_eq!("MMMXMMMMDMMMMMMMIMMMMMMMMMXMMMMMM", cg_str);

    let cigar = wavefronts.cigar_bytes();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    assert_eq!("3M1X4M1D7M1I9M1X6M", cg_str);
}

#[test]
fn wavefronts_reduced_align() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let pattern = String::from("TCTTTACTCGCGCGTTGGAGAAATACAATAGT");
    let text = String::from("TCTATACTGCGCGTTTGGAGAAATAAAATAGT");

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    let pat_len = pattern.as_bytes().len();
    let text_len = text.as_bytes().len();

    let mut wavefronts = AffineWavefronts::new_reduced(
        pat_len,
        text_len,
        &mut penalties,
        10,
        50,
        &alloc,
    );

    let result = wavefronts.align(pattern.as_bytes(), text.as_bytes());

    assert!(result.is_ok());

    let score = wavefronts.edit_cigar_score(&mut penalties);
    assert_eq!(score, -24);

    let cigar = wavefronts.cigar_bytes_raw();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    assert_eq!("MMMXMMMMDMMMMMMMIMMMMMMMMMXMMMMMM", cg_str);
}

use std::fs::File;
use std::io::{BufRead, BufReader};

#[test]
fn wavefronts_complete_align_sars2() -> Result<(), std::io::Error> {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let filen = "tests/data/20VR2012.fasta";
    let file = File::open(filen)?;
    let reader = BufReader::new(file);
    let mut seqs1 = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        println!("{}. {}", index + 1, line);
        seqs1.push(line);
    }
    let seq1 = String::from(seqs1.join(""));

    let filen = "tests/data/MT655751.1.fasta";
    let file = File::open(filen)?;
    let reader = BufReader::new(file);
    let mut seqs2 = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        println!("{}. {}", index + 1, line);
        seqs2.push(line);
    }
    let seq2 = String::from(seqs2.join(""));
    println!("{}", seq2);

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    let seq1_len = seq1.as_bytes().len();
    let seq2_len = seq2.as_bytes().len();

    let mut wavefronts = AffineWavefronts::new_complete(
        seq1_len,
        seq2_len,
        &mut penalties,
        &alloc,
    );

    let result = wavefronts.align(seq1.as_bytes(), seq2.as_bytes());
    assert!(result.is_ok());

    let score = wavefronts.edit_cigar_score(&mut penalties);
    let cigar = wavefronts.cigar_bytes_raw();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    println!("{}", cg_str);
    assert_eq!(score, -24);

    assert_eq!("MMMXMMMMDMMMMMMMIMMMMMMMMMXMMMMMM", cg_str);

    let cigar = wavefronts.cigar_bytes();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    assert_eq!("3M1X4M1D7M1I9M1X6M", cg_str);
    Ok(())
}
