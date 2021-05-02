#![allow(unused)]

use std::fs::{File, ReadDir};
use crate::tests::{snapshot_test_with_folder, Interactive};
use rand::Rng;
use std::fmt::Write;
use rand::rngs::ThreadRng;
use std::collections::vec_deque::VecDeque;
use rand::seq::SliceRandom;
use std::process::{Command, Stdio};
use rand::distributions::Alphanumeric;
use std::convert::TryInto;
use std::path::Path;
use std::fs;
use std::env::args;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::sync::RwLock;

fn main() {
    if args().any(|arg| arg == "--cov") {
        println!("Computing test case coverage");
        test_case_coverage();
        return;
    }
    let fuzz_dir = Path::new("fuzzer");
    let mut case = fs::read_dir(fuzz_dir).map(ReadDir::count).unwrap_or(0);
    let mut rng = rand::thread_rng();
    std::fs::create_dir_all("./fuzzer-data/books/");
    let mut book_i = 0usize;
    let mut used_filenames: Vec<String> = Vec::new();
    let mut books: Vec<_> = std::fs::read_dir("./fuzzer-data/books/")
        .unwrap()
        .into_iter()
        .map(|item| format!("fuzzer-data/books/{}", item.unwrap().file_name().to_str().unwrap()))
        .chain(std::iter::once(String::from("tests/resources/book-tmp.bin")))
        .collect();
    let mut timeout = 1i32;
    let mut last_coverage_line = String::from("");
    let mut last_coverage_not_changed = 0u64;
    loop {
        let case_dir = fuzz_dir.join(case.to_string());
        std::fs::remove_dir_all(&case_dir);
        let run_dir = case_dir.join("run_dir");

        fs::create_dir_all(&case_dir);
        let binary_folder = "../../../../zebra-1/";
        let binary = "zebra";
        let mut args = String::new();
        let interactive = match rng.gen_range::<i32, _>(0..6) {
            1 => Interactive::Dumb,
            _ => Interactive::None,
        };
        match interactive {
            Interactive::Dumb => {
                args.push_str("-l 0 0");
            }
            _ => {
                // TODO also test getting these args from command line with scanf
                let uneven_game = rng.gen_ratio(1, 4);
                let max_depth = timeout.min(20);
                if uneven_game {
                    write!(args, "-l {} {} {} {} {} {}",
                           rng.gen_range::<i32, _>(0..max_depth + 2),
                           rng.gen_range::<i32, _>(0..max_depth + 5),
                           rng.gen_range::<i32, _>(0..max_depth + 5),
                           rng.gen_range::<i32, _>(0..max_depth + 2),
                           rng.gen_range::<i32, _>(0..max_depth + 5),
                           rng.gen_range::<i32, _>(0..max_depth + 5),
                    );
                } else {
                    let base_depth = rng.gen_range(0..max_depth + 2);
                    let base_exact = rng.gen_range(0..max_depth + 5);
                    let base_wld = rng.gen_range(0..max_depth + 5);
                    write!(args, "-l {} {} {} {} {} {}",
                           base_depth + rng.gen_range(0..3),
                           base_exact + rng.gen_range(0..3),
                           base_wld + rng.gen_range(0..3),
                           base_depth + rng.gen_range(0..3),
                           base_exact + rng.gen_range(0..3),
                           base_wld + rng.gen_range(0..3),
                    );
                }
            }
        }

        //TODO generate also invalid arguments for these (there's bunch of jumps that are not executed in the main loop)
        // CountFlips_bitboard_d3, CountFlips_bitboard_d4, TestFlips_bitboard_d4,...
        // were not hit in 2000 random tests, what to do about that?
        //  generate -l with giant end solve numbers? Those are all starting positions though
        //  I think we can only test them with random boards

        // todo adjust_counter loop is also never tested
        //  VERBOSE is not tested
        //  get_book_move() = -1 is not covered, too
        //  is_panic_abort + force_return ifs are also not often executed
        //  it also seems that there's not enough draws tested
        //  invalid boards and move sequences are also undertested it seems (and missing files too)
        //  forced opening is undertested (and also probably just dead)
        //  mirror_symmetry error is not reached
        //  resize hash is also not tested
        //  sort_moves, float_move not tested
        //  game_learnable,get_stored_move,set_perturbation, count_all_wrapper, count_all,
        //   check_forced_opening, ponder_move,set_ponder_move, clear_ponder_move,get_current_eval
        //  stability_search, get_stable, complete_stability_search, add_ponder_time,get_search_statistics
        //  get_pv, report_move_evals, report_hash_move, perform_extended_solve,
        //  full_learn_public_game =  not reached / dead code
        //  =================================
        //  consequently, loop in calculate_perturbation is dead
        //  set_max_batch_size,set_black_force, set_white_force, set_eval_span, set_negamax_span is only used in booktool
        //  extended_compute_move is only used in enddev and practice
        //  force_return == 0 always so every code behind it is not tested
        //  produce_compact_eval doesn't have all return branches tested
        //  FullState::new is not covered - even though it's there. Is it because inline(always)?
        //  dumpch is not tested
        //  fatal_error_3, fatal_error_1 is not tested
        //  log_best_move is not tested
        //  some errors are not well tested  (missing file)
        //  add_new_game is undertested
        //  read_text_database and write_text_database are not tested

        let flags: &mut [(u32, &dyn Fn(&mut String, &mut ThreadRng))] = &mut [
            (12, &(|s, rng| { write!(s, "-public"); })),
            (12, &(|s, rng| { write!(s, "-private"); })),
            (12, &(|s, rng| { write!(s, "-keepdraw"); })),
            (12, &(|s, rng| { write!(s, "-draw2black"); })),
            (12, &(|s, rng| { write!(s, "-draw2white"); })),
            (12, &(|s, rng| { write!(s, "-draw2none"); })),
            (8, &(|s, rng| { write!(s, "-test"); })),
            (8, &(|s, rng| { write!(s, "-analyze"); })),
            (8, &(|s, rng| { write!(s, "-repeat {}", rng.gen_range::<i32, _>(0..(timeout/4) + 2)); })),
            (2, &(|s, rng| { write!(s, "-r {}", rng.gen_range::<i32, _>(0..2)); })),
            (6, &(|s, rng| { write!(s, "-slack {}", rng.gen_range::<f32, _>(0.0..100.0)); })),
            (6, &(|s, rng| { write!(s, "-randmove {}", rng.gen_range::<i32, _>(0..10)); })),
            (2, &(|s, rng| { write!(s, "-p {}", rng.gen_range::<i32, _>(0..2)); })),
            (2, &(|s, rng| { write!(s, "-e {}", rng.gen_range::<i32, _>(0..2)); })),
            (2, &(|s, rng| { write!(s, "-b {}", rng.gen_range::<i32, _>(0..2)); })),
            (4, &(|s, rng| { write!(s, "-w {}", rng.gen_range::<i32, _>(0..2)); })),
            (4, &(|s, rng| { write!(s, "-thor {}", rng.gen_range::<i32, _>(0..20)); })),
            (4, &(|s, rng| { write!(s, "-wld {}", rng.gen_range::<i32, _>(0..2)); })),
            (4, &(|s, rng| { write!(s, "-h {}", rng.gen_range::<i32, _>(0..22)); })),
            (8, &(|s, rng| {
                write!(s, "-dev {} {} {}",
                       rng.gen_range::<i32, _>(0..100),
                       rng.gen_range::<i32, _>(0..100),
                       rng.gen_range::<f32, _>(0.0..220.0),
                );
            })),
            // NOTE: -t is mutually exclusive with -l
            //  (it doesn't matter too much now, because -t will just override it)
            //   (later args override previous ones)
            (8, &(|s, rng| {
                let number_of_levels = rng.gen_range::<i32, _>(0..4);
                write!(s, "-t {}", number_of_levels);
                for _ in 0..number_of_levels {
                    write!(s, " {} {} {}",
                           // TODO allow human player too (by specifying zero here)
                           rng.gen_range::<i32, _>(1..timeout + 2),
                           rng.gen_range::<i32, _>(0..timeout + 1),
                           rng.gen_range::<i32, _>(0..timeout + 1),
                    );
                }
            })),
            // TODO test invalid board files
            (8, &(|s, rng| {
                if rng.gen_ratio(1, 4) {
                    static CHARSET: &[u8] = b"X-O";
                    let mut board_file = String::new();
                    board_file.extend((0..64).map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char));
                    board_file.push_str("\n");
                    if rng.gen_ratio(1, 2) {
                        board_file.push_str("White to move");
                    } else {
                        board_file.push_str("Black to move");
                    }
                    board_file.push_str("\nThis file was automatically generated\n");
                    std::fs::write(case_dir.join("board.txt"),board_file ).unwrap();
                    write!(s, "-g ../board.txt");
                } else {
                    loop {
                        let case_board_to_steal = rng.gen_range(0..case + 1);
                        if case_board_to_steal == case {
                            write!(s, "-g ../../../tests/resources/board.txt");
                            break;
                        }
                        let mut origin_board_path = fuzz_dir.join(case_board_to_steal.to_string());
                        origin_board_path.push("run_dir/current.gam");
                        if let Ok(_) = fs::copy(origin_board_path, case_dir.join("board.txt")) {
                            write!(s, "-g ../board.txt");
                            break;
                        };
                    }
                }
            })),
            // TODO test more randomly generated games
            (6, &(|s, rng| {
                let seq = new_seq( rng,fuzz_dir, case);
                s.push_str("-seq ");
                s.push_str(&seq);
            })),
            (8, &(|s, rng| {
                write!(s, "-time {} {} {} {}",
                       rng.gen_range(0..500i32),
                       rng.gen_range(0..500i32),
                       rng.gen_range(0..500i32),
                       rng.gen_range(0..500i32),
                );
            })),
            (6, &(|s, rng| {
                let seq = new_seq(rng, fuzz_dir, case);
                std::fs::write(case_dir.join("seq.txt"), seq).unwrap();
                write!(s, "-seqfile ../seq.txt");
            })),
            (8, &(|s, rng| {
                s.push_str("-log ");
                if rng.gen_ratio(1, 4) {
                    s.push_str(&used_filenames[rng.gen_range(0..used_filenames.len())])
                } else {
                    s.extend(rng.sample_iter(&Alphanumeric)
                        .take(10)
                        .map(char::from))
                };
            })),
            (8, &(|s, rng| {
                write!(s, "-learn {} {} ",
                       rng.gen_range(0..timeout + 1),
                       rng.gen_range(0..60 - timeout)
                );
            })),
        ];
        flags.shuffle(&mut rng);

        for (denominator, flag) in flags {
            if rng.gen_ratio(1, *denominator)  {
                args.push(' ');
                flag(&mut args, &mut rng);
            }
        }
        let arguments = args.as_str();

        let book_path = if rng.gen_ratio(1, 2) && !arguments.contains("-learn") {
            "book.bin"
        } else {
            books[rng.gen_range(0..books.len())].as_str()
        };

        let adjust = if rng.gen_ratio(1, 4) {
            println!("creating adjust.txt");
            Some(format!("{} {} {} {}\n",
                         rng.gen_range::<f32, _>(0.0..20.0),
                         rng.gen_range::<f32, _>(0.0..20.0),
                         rng.gen_range::<f32, _>(0.0..20.0),
                         rng.gen_range::<f32, _>(0.0..20.0),
            ))
        } else {
            None
        };
        let thor_files: Vec<_> = if arguments.contains("-thor") {
            std::fs::read_dir("thor").unwrap().filter_map(|dir| {
                let name = dir.unwrap().file_name();
                let name = name.to_str().unwrap();
                let chance_denominator = if name.contains("jou") || name.contains("trn") {
                    // those files are special, so let's give them higher probability
                    3
                } else {
                    8
                };
                if rng.gen_ratio(1, chance_denominator) {
                    println!("Using thor file: {}", name);
                    Some(name.to_owned())
                } else {
                    None
                }
            }).collect()
        } else {
            Vec::new()
        };

        let coeffs_path_from_run_dir = "./../../../coeffs2.bin";
        let book_path_from_run_dir = format!("./../../../{}", book_path);

        let command = format!("RUST_BACKTRACE=1  BOOK_PATH={} COEFFS_PATH={} ../../../test-target/release/zebra {}", book_path_from_run_dir, coeffs_path_from_run_dir, arguments);
        std::fs::write(case_dir.join("command.txt"), &command);
        println!("{}", &command);

        std::fs::write(case_dir.join("failing"), "");
        let success = snapshot_test_with_folder(binary_folder, binary, arguments, &case_dir,
                                  adjust.as_ref().map(AsRef::as_ref), interactive,
                                  coeffs_path_from_run_dir,
                                  book_path_from_run_dir.as_ref(),
                                  book_path, timeout as _, &thor_files, "thor");
        if !success {
            std::fs::remove_file(case_dir.join("failing"));
            std::fs::write(case_dir.join("timeout"), "");
            continue;
        }
        let binary_folder = "../../../test-target/release/";

        let success = snapshot_test_with_folder(binary_folder, binary, arguments, &case_dir,
                                  adjust.as_ref().map(AsRef::as_ref), interactive,
                                  coeffs_path_from_run_dir, book_path_from_run_dir.as_ref(),
                                                book_path, timeout as _, &thor_files, "thor");
        std::fs::remove_file(case_dir.join("failing"));
        if !success {
            std::fs::write(case_dir.join("timeout"), "");
            continue;
        }
        std::fs::read_dir(&run_dir).unwrap().for_each(|dir| {
            let name = dir.unwrap().file_name().to_str().unwrap().into();
            if name == "book.bin" {
                book_i += 1;
                let new_path = format!("./fuzzer-data/books/book-{}.bin", book_i);
                std::fs::copy(run_dir.join("book.bin"), &new_path).unwrap();
                books.push(new_path);
            }
            if name == "default.profraw" {
                return;
            }
            used_filenames.push(name);
        });
        used_filenames.sort();
        used_filenames.dedup();

        std::fs::remove_file("all-tests-with-fuzz.profdata");

        Command::new("bash") //
            .arg("-c")
            .arg("cargo-profdata -- merge -sparse ./tests/snapshot-tests/*/*/default.profraw ./fuzzer/*/run_dir/*.profraw  -o all-tests-with-fuzz.profdata")
            .output().unwrap();
        let coverage = Command::new("cargo")
            .args("cov -- report test-target/release/zebra -instr-profile all-tests-with-fuzz.profdata -ignore-filename-regex /home/matyas/.cargo/".split_whitespace())
            .output().unwrap().stdout;
        std::str::from_utf8(&coverage)
            .unwrap()
            .lines()
            .for_each(|line| {
                if line.starts_with("TOTAL") {
                    if last_coverage_line == line {
                        last_coverage_not_changed += 1;
                        if last_coverage_not_changed > 20 {
                            last_coverage_not_changed = 0;
                            timeout *= 2;
                            println!("Coverage didn't change in 30 iterations. Increasing timeout.")
                        }
                    } else {
                        last_coverage_not_changed = 0;
                        last_coverage_line = line.to_owned();
                    }

                    println!("{}", line);
                }

            });
        case += 1;
    }

}
fn test_case_coverage() {
    let cases = Command::new("bash")
        .arg("-c")
        .arg("echo ./tests/snapshot-tests/*/run_dir/default.profraw ./fuzzer/*/run_dir/*.profraw")
        .output()
        .unwrap()
        .stdout
        ;
    #[derive(Debug, Clone)]
    struct CoverageLine {
        file_id: usize,
        regions:i32,
        covered_regions: i32
    }
    #[derive(Debug, Clone)]
    struct Case {
        profile: String,
        total_covered_regions: i32,
        coverage: Vec<CoverageLine>
    }
    let mut id_map = RwLock::new(HashMap::<String, usize>::new());
    fn case_coverage(case: &str, dir: &str, report: &mut dyn Write, use_cache: bool, id_map: &RwLock<HashMap<String, usize>>) -> Case {
        let coverage_file_path = Path::new(dir).join("case-coverage.txt");

        use std::fmt::Write;
        let case_coverage = if let Some(file) = use_cache.then(|| fs::read_to_string(&coverage_file_path).ok()).flatten()  {
            writeln!(report, "loading already computed coverage for case: {}" ,case);
            file
        } else {
            writeln!(report, "Computing new coverage for case: {}" ,case);
            // TODO --Xdemangler=rustfilt ?
            let arg = format!("cargo-profdata -- merge -sparse -num-threads=1 {}  -o {dir}case.profdata && \
        cargo cov -- report test-target/release/zebra -instr-profile {dir}case.profdata -ignore-filename-regex /home/matyas/.cargo/", case, dir = dir);

            let cov = String::from_utf8(Command::new("bash")
                .arg("-c")
                .arg(arg)
                .output().unwrap().stdout).unwrap();
            if use_cache {
                std::fs::write(&coverage_file_path, &cov).unwrap();
            }
            cov
        };

        let mut total_covered_regions = 0;

        let mut coverage: Vec<_> = case_coverage
            .lines()
            .filter_map(|line| {
                if line.starts_with("---") || line.starts_with("Filename") {
                    return None;
                }
                let mut split = line.split_whitespace();
                let filename = split.next().unwrap();
                let total_regions: i32 = split.next().unwrap().parse().unwrap();
                let missing_regions: i32 = split.next().unwrap().parse().unwrap();
                let covered_regions = total_regions - missing_regions;
                // TOTAL                                        7590              4357    42.60%         729               393    46.09%       29060             19824    31.78%           0                 0         -
                if filename == "TOTAL" {
                    write!(report, "{}\n", line);
                    total_covered_regions = covered_regions;
                    return None;
                }
                let option = id_map.read().unwrap().get(filename).cloned();
                return Some(CoverageLine {
                    file_id: option.unwrap_or_else(|| {
                        let mut id_map = id_map.write().unwrap();
                        // We need to check again, because someone else could add the value in
                        // the meantime, because we only held read lock
                        if let Some(v) = id_map.get(filename) {
                            return *v;
                        }
                        let val = (id_map.len());
                        id_map.insert(filename.to_owned(), val);
                        val
                    }),
                    regions: total_regions,
                    covered_regions,
                })
            }).collect();
        coverage.sort_by_key(|c| c.file_id);
        let computed_covered_regions = coverage.iter().map(|x| x.covered_regions).sum();
        assert_eq!(total_covered_regions, computed_covered_regions);
        Case {
            profile: case.to_string(),
            total_covered_regions,
            coverage
        }
    }
    use rayon::prelude::*;
    let all_cases = String::from_utf8(cases).unwrap();
    let mut cases = all_cases
        .split_whitespace()
        .par_bridge()
        .map(|case| {
            let dir = case.strip_suffix("run_dir/default.profraw").unwrap();
            let mut out = String::new();
            let coverage = case_coverage(case, dir, &mut out, true, &id_map);
            println!("{}", out);
            coverage
        })
        .collect::<Vec<Case>>();
    cases.sort_unstable_by_key(|case| -case.total_covered_regions);

    // println!("{:?}", cases);

    let base_test_case_dir = "test-case-finder/base/";
    std::fs::create_dir_all(base_test_case_dir);
    std::fs::create_dir_all("test-case-finder/current/");

    let mut report = String::new();
    let all_cases_coverage = case_coverage("./tests/snapshot-tests/*/run_dir/default.profraw ./fuzzer/*/run_dir/*.profraw",
                                           base_test_case_dir, &mut report, false, &mut id_map);
    println!("{}", report);
    let mut winners = vec![&cases[0]];
    let mut winners_coverage = winners[0].clone();

    #[derive(Clone)]
    struct ConsideredCase<'a> {
        case: &'a Case,
        potential: i32,
    }
    fn potential(winners: &Case, candidate: &Case, all_cases_coverage: &Case) -> i32 {
        assert_eq!(winners.coverage.len(), candidate.coverage.len());
        winners.coverage.iter()
            .zip(candidate.coverage.iter())
            .zip(all_cases_coverage.coverage.iter())
            .map(|((win, cand), all)| {
                assert_eq!(win.file_id, cand.file_id);
                assert_eq!(win.file_id, all.file_id);
                assert!(all.regions >= cand.regions);
                assert!(all.regions >= win.regions);

                let remaining_regions_to_cover = all.regions - win.covered_regions;
                let potential_addition = (remaining_regions_to_cover).min(cand.covered_regions);
                potential_addition
            }).sum()
    }
    let mut considered: Vec<_> = cases.iter()
        .skip(1)
        .map(|case| ConsideredCase { case, potential: potential(&winners_coverage, case, &all_cases_coverage) })
        .collect();
    loop {
        std::fs::create_dir("test-case-finder/current/_0/");
        let mut rep = String::new();
        let candidate = std::sync::RwLock::new((
            considered[0].case,
            // FIXME this is incorrect, but this will be recalculated so it doesn't matter
            considered[0].case.clone()
        ));
        println!("{}", rep);
        std::fs::remove_dir_all("test-case-finder/current/");
        let considered_size = considered.len();
        let maximum_potential = considered.iter().map(|c|c.potential).max().unwrap();
        let max_possible_covered_regions_in_this_cycle = winners_coverage.total_covered_regions + maximum_potential;
        considered = considered.into_iter().enumerate().par_bridge().filter_map(|(i, mut considered_case)| {
            let case = considered_case.case;

            struct Report(String);
            impl Drop for Report { fn drop(&mut self) { println!("{}", self.0) } }
            let mut report = Report (format!("Testing {}/{}: ", i, considered_size));

            let read_guard = candidate.read().unwrap();
            let coverage_with_candidate = &read_guard.1;

            assert!(coverage_with_candidate.total_covered_regions <= max_possible_covered_regions_in_this_cycle);
            // FIXME this is still sub optimal because we don't filter the previous best case from the data
            //  we need to keep track of not only max potential, but also second max potential. That's probably also why we never hit it

            if coverage_with_candidate.total_covered_regions == max_possible_covered_regions_in_this_cycle {
                report.0.push_str("Skipping case because we already reached maximum potential in this cycle.");
                return Some(considered_case);
            }
            if winners_coverage.total_covered_regions + case.total_covered_regions <= coverage_with_candidate.total_covered_regions
                || winners_coverage.total_covered_regions + considered_case.potential <= coverage_with_candidate.total_covered_regions
            {
                report.0.push_str("Skipping case for current iteration because it can't add more regions than candidate.");
                return Some(considered_case);
            }


            report.0.push_str("\n");
            let coverage_with_candidate = read_guard.1.total_covered_regions; // TODO use the computed potential
            drop(read_guard);
            // TODO don't do this in every iteration (we can just concat winners once and than add)
            let combined_case = winners.iter()
                .chain(std::iter::once(&case))
                .map(|c| c.profile.replace("run_dir/default.profraw", "case.profdata"))
                .collect::<Vec<_>>()
                .join(" ")
                ;
            let dir = format!("test-case-finder/current/{}/", i);
            std::fs::create_dir_all(&dir);
            let current_case = case_coverage(&combined_case, &dir, &mut report.0, false, &id_map);

            // let's update the potential - this one will be lower than the initial one, so we
            //  can filter out more cases in the future
            considered_case.potential = (winners_coverage.coverage.iter()
                .zip(current_case.coverage.iter())
                .map(|(winner, current)| current.covered_regions - winner.covered_regions)
                .sum());
            // let's first check the value of coverage_with_candidate
            // we have read at the begining of the loop
            // it might have changed in the meantime, but it can only increase,
            // so if this if fails, we know it will fail for the new value too and
            // we don't need to lock to read the new value
            if current_case.total_covered_regions > coverage_with_candidate {
                let mut guard = candidate.write().unwrap();
                let (candidate_case, coverage_with_candidate) = guard.deref_mut();
                if current_case.total_covered_regions > coverage_with_candidate.total_covered_regions {
                    *candidate_case = &case;
                    *coverage_with_candidate = current_case;
                    return Some(considered_case);
                }
            }

            if current_case.total_covered_regions <= winners_coverage.total_covered_regions {
                // TODO or when current  max_potential with potential of this case is still lower than winners coverage??
                use std::fmt::Write;
                write!(report.0, "Excluding {}", i );
                return None;
            }
            return Some(considered_case);
        }).collect();
        let (candidate_case, coverage_with_candidate) = candidate.into_inner().unwrap();
        if coverage_with_candidate.total_covered_regions == winners_coverage.total_covered_regions {
            println!("Total coverage didn't change in a single cycle. Ending");
            break;
        }
        winners.push(candidate_case);
        winners_coverage = coverage_with_candidate;
        if considered.is_empty() {
            //TODO at the end, this is not hit and we just keep cycling - what's the deal with that?
            break;
        };
    }
    println!("winners {:?}", winners.iter().map(|win| &win.profile).collect::<Vec<_>>())
}
//
//./fuzzer/2565/run_dir/default.profraw
//./fuzzer/2668/run_dir/default.profraw
//./fuzzer/988/run_dir/default.profraw
//./fuzzer/2022/run_dir/default.profraw
//./fuzzer/2317/run_dir/default.profraw
//./fuzzer/718/run_dir/default.profraw
//./fuzzer/2778/run_dir/default.profraw
//./fuzzer/251/run_dir/default.profraw
//./fuzzer/3133/run_dir/default.profraw
//./fuzzer/1963/run_dir/default.profraw
//./fuzzer/2118/run_dir/default.profraw
//./fuzzer/1168/run_dir/default.profraw
//./fuzzer/1847/run_dir/default.profraw
//./fuzzer/312/run_dir/default.profraw
//./fuzzer/981/run_dir/default.profraw

fn new_seq(rng: &mut ThreadRng, fuzz_dir: &Path, case: usize) -> String {
    let mut arg = loop {
        let seq_to_steal = rng.gen_range(0..case + 1);
        if seq_to_steal == case {
            break String::from("e6f6f5f4e3d6g4d3c3h3c4g3g5g6c7c6c5b6d7b5f7f3b4f8h4h5f2f1h2h1");
        }
        let mut move_path = fuzz_dir.join(seq_to_steal.to_string());
        move_path.push("run_dir/current.mov");
        if let Ok(file) = fs::read_to_string(move_path) {
            let file = file.split_whitespace().filter_map(|item| {
                let first = *item.as_bytes().get(0)?;
                let second = *item.as_bytes().get(1)?;
                match (item.len(), first, second) {
                    (2, b'a'..=b'h', b'1'..=b'8') => {
                        Some(item)
                    }
                    _ => None
                }
            }).collect::<String>();
            break file;
        };
    };
    let slice_to = if arg.len() == 0 { 0 } else { rng.gen_range(0..arg.len()) };
    arg.truncate(slice_to);
    arg
}

mod tests {
    use flate2_coeff_source::Flate2Source;
    use std::ffi::CStr;
    use zlib_coeff_source::{ZLibSource};
    use std::process::{Command, Stdio, ChildStdin, ExitStatus, Child};
    use std::path::{Path, PathBuf};
    use std::fs::{File, DirEntry, read_dir};
    use std::io::{Write};
    use std::iter::FromIterator;
    use std::convert::TryFrom;
    use wait_timeout::ChildExt;
    use std::time::{Duration, SystemTime};
    use std::ops::{Add, Deref};
    use std::sync::{Arc, Mutex};

    #[test]
    fn coeff_source_test() {
        use engine_traits::CoeffSource;
        let file_name: &CStr = CStr::from_bytes_with_nul(b"./../coeffs2.bin\x00").unwrap();

        let mut z_lib_source = ZLibSource::try_new(file_name).unwrap();

        let mut flate2_source = Flate2Source::new_from_data(&std::fs::read("./../coeffs2.bin").unwrap());

        while let Some(word) = z_lib_source.try_next_word() {
            let flate_word = flate2_source.try_next_word().unwrap();
            assert_eq!(word, flate_word)
        }

        assert!(flate2_source.try_next_word().is_none());
    }

    macro_rules! snap_test {
        ($id:ident, $args:literal) => {
            snap_test!($id, $args, with_adjust: true);
        };

        ($binary:literal, $func:ident, $suffix:literal, $id:ident, $args:literal, $has_adjust:expr) => {
            snap_test!($binary, $func, $suffix, $id, $args,  $has_adjust, interactive: None);
        };

        ($binary:literal, $func:ident, $suffix:literal, $id:ident, $args:literal, $has_adjust:expr, interactive: $interactive:ident) => {
                #[test]
                fn $func() {
                    use crate::tests::*;
                    snapshot_test(
                        $binary,
                        $args,
                        &("./snapshot-tests/".to_owned() + stringify!($id) + $suffix),
                        $has_adjust,
                        Interactive::$interactive
                    );
                }
        };

        ($id:ident, $args:literal, with_adjust: true) => {
            mod $id {
                snap_test!("zebra", basic, "-basic", $id, $args, false);
                snap_test!("zebra", with_adjust, "-with-adjust", $id, $args, true);
            }
        };
        ($binary:literal, $id:ident, $args:literal, with_adjust: false) => {
            mod $id {
                snap_test!($binary, basic, "-basic", $id, $args, false);
            }
        };

        ($binary:literal, $id:ident, $args:literal) => {
            mod $id {
                snap_test!($binary, basic, "-basic", $id, $args,  false);
            }
        };
        ($binary:literal, $id:ident, $args:literal, $has_err:literal) => {
            mod $id {
                snap_test!($binary, basic, "-basic", $id, $args, false);
            }
        };

        ($id:ident, $args:literal, with_adjust: false) => {
            mod $id {
                snap_test!("zebra", basic, "-basic", $id, $args, false);
            }
        };

        ($id:ident, $args:literal, interactive: Dumb) => {
            mod $id {
                snap_test!("zebra", basic, "-basic", $id, $args, false, interactive: Dumb);
            }
        };
        ($binary:literal, $id:ident, $args:literal, interactive: $interactive:ident) => {
            mod $id {
                snap_test!($binary, basic, "-basic", $id, $args, false, interactive: $interactive);
            }
        };

        ($id:ident, $args:literal) => {
            snap_test!($id, $args, with_adjust: true);
        };
    }

    snap_test!(minus_p_zero, "-l 6 6 6 6 6 6 -r 0 -p 0");

    snap_test!(with_seq, "-seq f5d6c3 -l 6 6 6 6 6 6 -r 0");

    snap_test!(analyze_basic, "-analyze -seq e6f6f5f4e3d6g4d3c3h3c4g3g5g6c7c6c5b6d7b5f7f3b4f8h4h5f2f1h2h1 -l 7 7 7 7 7 7 -r 0");

    snap_test!(analyze_invalid, "-analyze -seq f1h2h1 -l 7 7 7 7 7 7 -r 0", with_adjust: false);

    snap_test!(with_seq_invalid, "-seq f5d6h1 -l 6 6 6 6 6 6 -r 0");

    snap_test!(with_no_echo, "-l 6 6 6 6 6 6 -r 0 -e 0");

    snap_test!(with_some_slack, "-l 6 6 6 6 6 6 -r 0 -slack 8");

    snap_test!(with_hash_twelve, "-l 6 6 6 6 6 6 -r 0 -h 12");

    snap_test!(with_repeat, "-l 6 6 6 6 6 6 -r 0 -repeat 2");

    snap_test!(with_repeat_and_log, "-l 6 6 6 6 6 6 -r 0 -repeat 2 -log zebra.log");

    snap_test!(no_wld, "-l 6 6 0 6 6 0 -r 0 -repeat 2");

    /*
    run this to verify
     cargo test --release --package tests "tests::no_exact_no_wld::no_exact_no_wld" -- --test-threads 1 --nocapture
    */
    snap_test!(no_exact_no_wld, "-l 6 0 0 6 0 0 -r 0 -repeat 2");

    snap_test!(minus_p_zero_without_book, "-l 6 6 6 6 6 6 -r 0 -p 0 -b 0");

    snap_test!(small_game_test_without_book, "-l 6 6 6 6 6 6 -r 0 -b 0", with_adjust: false);

    snap_test!(full_game_test, "-l 16 16 16 16 16 16 -r 0", with_adjust: false);

    snap_test!(small_game_test, "-l 6 6 6 6 6 6 -r 0", with_adjust: false);

    snap_test!(micro_game, "-l 1 1 1 1 1 1 -r 0", with_adjust: false);

    snap_test!(thor_five, "-l 2 2 2 2 2 2 -r 0 -thor 5", with_adjust: false);

    snap_test!(uneven, "-l 1 1 1 8 8 8 -r 0", with_adjust: false);

    snap_test!(rand_move, "-l 6 6 6 6 6 6 -r 0 -randmove 3", with_adjust: false);

    snap_test!(rand_move_without_book, "-l 6 6 6 6 6 6 -r 0 -randmove 3 -b 0", with_adjust: false);

    snap_test!(rand_move_one, "-l 6 6 6 6 6 6 -r 0 -randmove 1", with_adjust: false);

    snap_test!(basic_interactive, "-l 6 6 6 0 -r 0 -b 0 -repeat 2", interactive: Dumb);

    snap_test!(basic_interactive_flipped, "-l 0 6 6 6 -r 0 -b 0 -repeat 1", interactive: Dumb);

    snap_test!(two_players, "-l 0 0 -r 0 -b 0 -repeat 2", interactive: Dumb);

    snap_test!(two_players_with_log, "-l 0 0 -r 0 -b 0 -repeat 2  -log zebra.log", interactive: Dumb);

    snap_test!(learn, "-l 2 2 2 2 2 2 -r 0 -learn 3 5", with_adjust: false);

    snap_test!(
        seqfile,
         "-l 2 2 2 2 2 2 -r 0 -seqfile ../../../resources/seq-file.txt -log zebra.log",
        with_adjust: false
    );

    snap_test!(
        seqfile_too_long,
         "-l 2 2 2 2 2 2 -r 0 -seqfile ../../../resources/seq-file-too-long.txt -log zebra.log",
        with_adjust: false
    );

    snap_test!(
        seqfile_invalid,
         "-l 2 2 2 2 2 2 -r 0 -seqfile ../../../resources/seq-file-invalid.txt -log zebra.log",
        with_adjust: false
    );

    snap_test!(
        board_source,
         "-l 2 2 2 2 2 2 -r 0 -g ../../../resources/board.txt -log zebra.log",
        with_adjust: false
    );

    snap_test!(
        learn_board_source_segv,
         "-r 0 -l 0 0 -thor 16 -b 1 -h 20 -learn 1 3  -g ../../../resources/board-fuzzer-1 -e 1 -p 1",
        interactive: Dumb
    );

    // TODO test all these parameters at once: -g, -seq and -seqfile, how they interact??
    //  what if they conflict??

    snap_test!("practice", practice, "./../../../../book.bin", interactive: Practice);
    snap_test!("practice", practice_help, "./../../../../book.bin dd dd");

    #[derive(Copy, Clone)]
    pub enum Interactive {
        Dumb,
        Practice,
        None
    }
    fn interact_practice(input: &mut ChildStdin) {
        let moves = "e6  f6 f5  f4 e3  d6 g4  d3 c3  h3 c4  g3 g5  g6 c7  c6 c5  b6 7  b5 f7 \
         f3 b4  f8 h4  h5 f2  f1 g2  e8 e7  a3 h2  h1 h6  d8 c8  b8 g7  b7 a4  g8 a8  a7 h8  h7 \
         g1  e1 e2  d1 a5  c2 d2  a6 b3  c1 a2  b2 a1  b1 quit";
        moves.split_whitespace().for_each(|move_| {
            let _ = input.write(move_.as_bytes());
            let _ = input.write("\n".as_bytes());
            let _ = input.flush();
        });
    }
    fn interact_basically(until: SystemTime, input: &mut ChildStdin, mut record_file: File) {
        loop {
            let mut written = 0;
            for char in b'a'..=b'h' {
                for num in b'1'..=b'8' {
                    if std::time::SystemTime::now() > until {
                        return;
                    }
                    let move_ = &[char, num, b'\n'];
                    written = input.write(move_).unwrap_or(0);
                    record_file.write(move_).unwrap_or(0);
                    let _ = input.flush();
                    if written == 0 {
                        break;
                    }
                }
                if written == 0 {
                    break;
                }
            }
            if written == 0 {
                break;
            }
            written = input.write(&[b'\n']).unwrap_or(0);
            record_file.write(&[b'\n']).unwrap_or(0);
            let _ = input.flush();
            if written == 0 {
                break;
            }
        }
    }
    const BINARY_FOLDER :&str =
    // "./../../../../../zebra-1/"
        "./../../../../test-target/release/"
    // "./../../../../../bisection/target/release/"
    ;
    macro_rules! from_fuzz {
        ($($id:ident, $args:literal, $book_path:literal, $thor_files:expr);+) => {
            $(
                #[test]
                fn $id() {
                    snapshot_test_from_fuzzer(
                        $args,
                        &("./snapshot-tests/".to_owned() + stringify!($id)),
                        $book_path,
                        $thor_files
                    );
                }
            )+
        };
    }

    from_fuzz!(
f1_1160 , "-l 6 7 5 6 7 7 -r 1 -e 0 -seqfile ../seq.txt -draw2black -t 2 4 4 2 5 2 1 -p 1 -learn 2 3 " , "../../../resources/books/book-34.bin", &[];
f1_1307 , "-l 11 24 1 15 23 10 -b 1 -randmove 1 -thor 7 -p 0 -r 1" , "book.bin", &["wth_1980.wtb","wth_1990.wtb"];
f1_1331 , "-l 15 11 18 17 9 18 -test -p 0 -keepdraw -b 0" ,  "../../../resources/books/book-633.bin", &[];
f1_1354 , "-l 5 19 23 4 19 24 -b 1 -r 0 -p 0 -randmove 7" ,  "book.bin", &[];
f1_1364 , "-l 2 3 1 2 4 1 -slack 88.27771 -r 0 -learn 1 26 -seq c4c5c6e3f2b4c3d2d3d6e2f3f4f5f6g1g2g3g4g5g6g7h1h2h3h4h5h6h7b3b5b6b7b8c1c2c7c8d1e1e6e7f1f7g8h8 -b 1 -analyze -h 6" ,  "../../../resources/books/book-140.bin", &[];
f1_1371 , "-l 1 5 5 1 6 7 -dev 83 41 97.18826 -p 0 -e 1 -t 2 1 1 0 2 1 1" ,  "book.bin", &[];
f1_1382 , "-l 2 0 4 2 1 4 -w 0 -b 1 -g ../board.txt" ,  "book.bin", &[];
f1_1399 , "-l 1 4 5 1 5 2 -time 435 31 352 340 -thor 19 -dev 69 57 209.51326 -h 8 -slack 7.4069858 -p 0" ,  "book.bin", &[];
f1_1455 , "-l 1 0 0 2 1 2 -t 2 1 1 0 2 0 0 -dev 35 12 146.69566 -p 1 -e 1 -r 0 -b 0" ,  "book.bin",&[];
f1_1571 , "-l 1 2 2 2 2 2 -learn 1 15 -log Mp9JapERVJ -b 0 -p 0" ,  "../../../resources/books/book-72.bin", &[];
f1_1638 , "-l 2 5 6 3 7 4 -thor 11 -h 10 -w 1 -p 1 -learn 0 21 " ,  "../../../resources/books/book-37.bin", &["wth_1980.wtb", "wthor.trn"];
f1_1692 , "-l 1 6 5 1 6 6 -seq c4c5c6e3f2b4c3d2d3d6e2f3f4f5f6g1g2g3g4f1h1e1b6b3h2b5e6c7a5a4c2d1c1a6a2g5h6h5g6d7d8h7c8f7g8e7e8f8g7h8a7a8a3b8b7b1a1b2h -seqfile ../seq.txt -e 0 -log f08O5Xd1kZ -dev 93 34 74.776405" , "book.bin", &[];
f1_1794 , "-l 0 0 -time 304 286 149 221 -p 1 -log ItB5HG1vdJ -thor 17 -w 0 -test -draw2black -b 0" ,  "book.bin", &["wthor.jou","wthor.trn"];
f1_1887 , "-l 0 0 -e 0 -r 1 -b 1 -t 1 3 1 2 -seqfile ../seq.txt -draw2black -randmove 2 -thor 17 -dev 56 15 166.48924" , "../../../resources/books/book-673.bin", &["wth_1981.wtb","wthor.jou","wthor.trn",];
f1_1892 , "-l 4 8 6 5 6 6 -g ../board.txt -r 1 -seq c4c5c6e3f2b4 -p 1 -seqfile ../seq.txt" , "../../../resources/books/book-555.bin", &[];
f1_1896 , "-l 2 2 5 1 4 6 -b 1 -analyze -e 0 -randmove 9 -seq c4c5c6e3f2b4c3d2d3d6e2f3f4f5f6g1g2g3g4g5g6g7h1h2h3h4h5h6h7b3b5b6b7b8c1c2c7 -draw2none" ,  "../../../resources/books/book-32.bin", &[];
f1_1936 , "-l 0 0 -learn 4 11 -e 1 -public -dev 55 17 32.211823 -g ../board.txt -thor 1" , "../../../resources/books/book-197.bin", &[];
f1_1980 , "-l 4 17 2 14 20 4" , "../../../resources/books/book-695.bin", &[];
f1_1989 , "-l 15 3 12 14 1 13 -b 1 -p 1 -keepdraw -seqfile ../seq.txt -seq d3e3f4c3f5g4f3f6e6g5c4g6g3f2h3h5h4h2f7d7e7c6d6c7c5b4b6b5c8a5b3d8e8a2e2f1d2c1h7h6h1c" ,  "book.bin",&[];
f1_2054 , "-l 0 0 -time 81 467 105 320 -analyze -draw2black -p 0 -dev 71 70 85.1679 -e 0 -b 1" ,  "book.bin", &[];
f1_2179 , "-l 5 5 8 5 3 8 -b 1 -h 0 -learn 2 0 -r 1" ,  "../../../resources/books/book-219.bin", &[];
f1_2230 , "-l 14 15 13 17 17 2 -r 0 -g ../board.txt -log current.mov" , "book.bin", &[];
f1_2446 , "-l 0 0 -r 0 -e 0 -g ../board.txt" ,  "book.bin",&[];
f1_247 , "-l 4 3 0 4 2 2 -r 0 -thor 6 -repeat 0 -log 7jVcoP348A -seqfile ../seq.txt" ,  "../../../resources/books/book-511.bin", &["wth_1980.wtb",];
f1_2535 , "-l 3 1 3 3 1 2 -learn 1 13 -dev 4 93 61.99677 -w 1 -r 0 -thor 4" ,  "../../../resources/books/book-414.bin", &["wthor.jou","wthor.trn",];
f1_2575 , "-l 0 0 -e 0 -t 2 2 1 3 4 4 4 -b 1 -repeat 1 -r 1 -thor 4" ,  "book.bin", &["wth_1990.wtb","wthor.jou","wthor.trn",];
f1_2588 , "-l 2 4 6 3 4 6 -log DQFDCEKN9c -e 1 -draw2white -dev 94 46 160.96391" ,  "../../../resources/books/book-531.bin", &[];
f1_2749 , "-l 3 6 1 3 7 1 -randmove 6 -repeat 0 -p 0 -h 3 -learn 1 16 -seq e6f6f5f4e3d6g4d3c3h3c4g3g5g6c7c6c5b6d7b5f7f3b4f8h4h5f2f1g7e7h7e8d8 -keepdraw -b 1" ,  "../../../resources/books/book-73.bin", &[];
f1_2773 , "-l 4 7 9 4 6 9 -learn 4 5 -r 1 -draw2none -dev 59 44 11.6894455" ,  "../../../resources/books/book-652.bin", &[];
f1_2788 , "-l 1 2 6 2 1 7 -b 0 -g ../board.txt -seq e6f6f5f4e3d6g4d3c3h3c4g3g5g6c7c6c5b6d7b5f7f3b4f8h4h5f2f1a6a4h6h7a3a2h2h1g2b8g7d8g1h8e7b2e8c2 -p 0 -learn 0 1 -time 298 327 94 27 -e 1 -h 15" , "../../../resources/books/book-463.bin", &[];
f1_2796 , "-l 0 0 -randmove 1 -thor 16 -h 17 -learn 1 11 -t 2 2 0 1 2 1 1 -r 0 -dev 79 62 160.49995 -p 0 -e 1" , "../../../resources/books/book-113.bin", &[];
f1_2879 , "-l 0 0 -public -p 1 -private" ,  "../../../resources/books/book-649.bin", &[];
f1_2926 , "-l 1 3 4 2 4 3 -e 1 -keepdraw -w 1 -dev 38 38 124.93513 -p 0 -time 320 403 188 492 -seq " ,  "book.bin", &[];
f1_2934 , "-l 11 5 7 10 5 6 -log CybGtA8mUF -thor 3 -b 1 -e 1" ,  "book.bin", &["wth_1990.wtb","wthor.jou","wthor.trn",];
f1_2944 , "-l 3 2 1 4 1 1 -h 20 -public -draw2none -t 2 2 1 1 2 1 1 -r 0 -p 1" ,  "book.bin",&[];
f1_344 , "-l 0 0 -e 0 -b 1 -wld 0 -r 1" ,  "book.bin",&[];
f1_363 , "-l 2 5 6 3 4 5 -t 3 2 1 0 1 1 1 2 1 0 -repeat 0 -e 0 -b 1 -r 0 -thor 13 -keepdraw" ,  "book.bin", &["wth_1981.wtb","wthor.trn",];
f1_427 , "-l 1 3 3 1 4 1 -time 475 68 380 304 -thor 7 -r 1 -p 1 -seq f5f6d3g5e6d7f7d6e7f4c6e8f8d8c8c7c5e3b8c -e 1 -g ../board.txt -analyze" ,  "../../../resources/books/book-255.bin", &[];
f1_515 , "-l 7 4 3 1 17 9 -draw2white -repeat 5 -thor 12 -p 1 -r 1" ,  "../../../resources/books/book-447.bin", &[];
f1_517 , "-l 5 1 6 5 0 6 -r 1 -learn 4 12 -slack 46.394203 -h 6 -b 1" ,  "../../../resources/books/book-10.bin", &[];
f1_545 , "-l 11 22 9 11 24 9 -time 107 472 369 392 -seqfile ../seq.txt -e 1 -p 0 -r 0 -dev 40 36 101.57625" ,  "../../../resources/books/book-331.bin", &[];
f1_577 , "-l 0 0 -draw2none -r 0 -log adjust.txt -slack 77.36907 -private -p 1 -thor 9" ,  "../../../resources/books/book-636.bin", &["wthor.trn",];
f1_585 , "-l 4 1 4 2 1 5 -t 3 1 2 1 2 1 2 2 2 0 -e 1 -w 1 -draw2none -seqfile ../seq.txt -keepdraw -p 0 -repeat 1 -public -learn 2 3 " ,  "../../../resources/books/book-711.bin", &[];
f1_60 , "-l 2 6 3 1 6 3 -p 1 -t 2 1 0 0 2 0 1 -g ../board.txt -draw2white -b 1 -r 1"  , "book.bin",&[];
f1_733 , "-l 7 12 7 9 12 7 -e 1 -learn 8 1 -slack 0.9908676 -dev 83 22 203.86996 -r 1" ,  "../../../resources/books/book-764.bin", &[];
f1_738 , "-l 4 6 2 8 11 8 -repeat 3 -b 1 -h 13 -seq d3c3e6e3d2f6f4g4f5d6d7c6e7d8 -time 424 200 3 341 -keepdraw -p 1" , "../../../resources/books/book-375.bin", &[];
f1_76 , "-l 2 7 6 3 6 5 -p 1 -keepdraw -test -e 1 -g ../board.txt -draw2none -learn 1 56 " , "../../../resources/books/book-456.bin", &[];
f1_770 , "-l 6 3 20 7 3 22 -thor 19 -r 1 -h 20 -p 0 -b 1 -repeat 3" , "book.bin", &["wth_1980.wtb", ]

    );

    fn snapshot_test_from_fuzzer(arguments: &str, snapshot_test_dir: &str, book_path: &str, thor_files:&[&str]) {
        let adjust = std::fs::read_to_string(Path::new(snapshot_test_dir).join("snapshots/adjust.txt")).ok();
        let thor_files: Vec<_> = thor_files.iter().map(Deref::deref).map(String::from).collect();
        let coeffs_path_from_run_dir = "./../../../../coeffs2.bin" ;
        let book_path_from_run_dir = if book_path == "book.bin" {
            "./../../../../book.bin"
        }  else {
            book_path
        };
        let interactive = if arguments.contains("-l 0 0") { Interactive::Dumb } else { Interactive::None };
        snapshot_test_with_folder(BINARY_FOLDER, "zebra", arguments, Path::new(snapshot_test_dir), adjust.as_ref().map(String::as_str),
                                  interactive, coeffs_path_from_run_dir, book_path_from_run_dir,
                                  if book_path == "book.bin" {
                                      "resources/book-tmp.bin"
                                  } else {

                                      book_path.strip_prefix("../../../").unwrap()
                                  },
                                  30, thor_files.as_slice(), "../thor");
    }

    fn snapshot_test(binary: &str, arguments: &str, snapshot_test_dir: &str, with_adjust: bool, interactive: Interactive) {

        let coeffs_path_from_run_dir = "./../../../../coeffs2.bin" ;
        let book_path_from_run_dir = "./../../../../book.bin" ;
        let with_adjust = if with_adjust {
            Some("3.5 2.8 5.1 12.3\n".as_ref())
        } else {
            None
        };
        snapshot_test_with_folder(BINARY_FOLDER, binary, arguments, Path::new(snapshot_test_dir), with_adjust,
                                  interactive, coeffs_path_from_run_dir, book_path_from_run_dir,
                                  "resources/book-tmp.bin", 30, &[], "../thor");
    }

    pub fn snapshot_test_with_folder(binary_folder: &str,
                                     binary: &str,
                                     arguments: &str,
                                     snapshot_test_dir: &Path,
                                     adjust: Option<&str>,
                                     interactive: Interactive,
                                     coeffs_path_from_run_dir: &str,
                                     book_path_from_run_dir: &str,
                                     swap_book_path: &str,
                                     timeout: u64,
                                     thor_files: &[String], thor_base: &str
    ) -> bool {

        let snapshot_test_dir = Path::new(snapshot_test_dir);
        if !snapshot_test_dir.exists() {
            std::fs::create_dir_all(snapshot_test_dir).unwrap();
        }
        let snapshots_dir = snapshot_test_dir.join("snapshots");
        if !snapshots_dir.exists() {
            std::fs::create_dir_all(&snapshots_dir).unwrap();
        }

        let run_directory = snapshot_test_dir.join("run_dir");
        let _ = std::fs::remove_dir_all(&run_directory);
        std::fs::create_dir_all(&run_directory).unwrap();
        if !thor_files.is_empty() {
            let thor_dir = run_directory.join("thor");
            std::fs::create_dir_all(&thor_dir).unwrap();
            for file in thor_files {
                std::fs::copy(Path::new(thor_base).join(file), thor_dir.join(file)).unwrap();
            }
        }

        if let Some(s) = adjust {
            let path = run_directory.join("adjust.txt");
            File::create(path)
                .unwrap()
                .write(s.as_bytes())
                .unwrap();
        }
        let binpath = run_directory
            .join(binary_folder)
            .join(binary)
            .canonicalize()
            .unwrap();

        let coeffs_path = run_directory.join(coeffs_path_from_run_dir).canonicalize().unwrap();
        let mut book_path = run_directory.join(book_path_from_run_dir).canonicalize().unwrap();
        let canon_run_dir = run_directory.canonicalize().unwrap();

        let compare_books = arguments.contains("-learn");
        if compare_books {
            let buf = canon_run_dir.join("book.bin");
            std::fs::copy(swap_book_path, &buf).unwrap();
            book_path = buf;
        }

        let mut child = Command::new(binpath)
            .current_dir(&canon_run_dir)
            .args(arguments.split_whitespace())
            .env("RUST_BACKTRACE", "1")
            // TODO enable leak san. again
            //  for now, it just reports some minor leaks from thor that seems like non-problem
            .env("ASAN_OPTIONS", "fast_unwind_on_malloc=0,detect_leaks=0")
            .env("COEFFS_PATH", coeffs_path.to_str().unwrap())

            // we probably don't need this when -learn parameter is set, because we copy the
            // investigate that
            .env("BOOK_PATH", book_path.to_str().unwrap())
            .stdin(Stdio::piped())
            .stderr(Stdio::from(File::create(canon_run_dir.join("zebra-stderr")).unwrap()))
            .stdout(Stdio::from(File::create(canon_run_dir.join("zebra-stdout")).unwrap()))
            .spawn()
            .unwrap();
        let start = std::time::SystemTime::now();
        let end = start.add(Duration::from_secs(timeout));
        let t = match interactive {
            Interactive::Dumb => {
                let mut input = child.stdin.take().unwrap();
                let stdin = File::create(canon_run_dir.join("zebra-stdin")).unwrap();
                Some(std::thread::spawn(move || {
                    interact_basically(end, &mut input, stdin);
                }))
            }
            Interactive::Practice => {
                let mut input = child.stdin.take().unwrap();
                interact_practice(&mut input);
                None
            }
            _ => None,
        };
        let remaining = Duration::from_secs(timeout)
            .checked_sub(std::time::SystemTime::now().duration_since(start).unwrap_or(Duration::from_secs(0)))
            .unwrap_or(Duration::from_secs(0));


        let exit_status = match child
            .wait_timeout(remaining)
            .unwrap() {
            Some(exit_status) => exit_status,
            None => {
                println!("{}s timeout expired, killing child process.", timeout);
                child.kill().unwrap();
                println!("Kill sent");
                child.wait().unwrap();
                println!("Killed, waiting for input thread to end");
                t.map(|t| t.join().unwrap());
                println!("Thread ended");
                return false;
            }
        };
        std::fs::write(
            run_directory.join("__snapshot_test_exit_status"),
            format!("exit status: {}", exit_status.code().unwrap())
        );
        let mut file_set = std::fs::read_dir(&snapshots_dir)
            .unwrap()
            .into_iter()
            .chain(std::fs::read_dir(&run_directory)
                .unwrap()
                .into_iter())
            .filter_map(|dir| {
                let entry = dir.unwrap();
                entry.metadata().unwrap().is_file().then(|| entry.file_name().into_string().unwrap())
            })
            .collect::<Vec<_>>();
        file_set.sort();
        file_set.dedup();
        for file in file_set {
            if file == "default.profraw" || file == "zebra-stdin" {
                continue; // ignore coverage data and stdin snapshot
            }
            assert_snapshot(
                snapshots_dir.join(&file).as_ref(),
                run_directory.join(&file).as_ref());
        }
        t.map(|t| t.join().unwrap());
        true
    }

    fn assert_snapshot(snapshot_path: &Path, result_path: &Path) {

        if result_path.exists() {
            if !snapshot_path.exists() {
                if std::env::var("BLESS").map(|v| v == "true").unwrap_or(false) {
                    std::fs::copy(result_path, snapshot_path).unwrap();
                } else {
                    panic!(
                        "\n\nWARNING: Snapshot for \n{:?}\n ... doesn't exist. Rerun the tests with BLESS=true environment variable to make them green again.\n\n\n",
                        result_path
                    );
                }
            }
        } else {
            if !snapshot_path.exists() {
                return // this means that this run doesn't have output any snapshot
            }
        }
        // If the file is not valid text, compare binary content
        let snapshot = std::fs::read(snapshot_path).unwrap();
        let snapshot_str = std::str::from_utf8(snapshot.as_ref());
        if snapshot_str.is_err() {
            let result = std::fs::read(result_path).unwrap();
            assert!(result == snapshot, "{} == {}" , snapshot_path.to_str().unwrap(), result_path.to_str().unwrap());
            return;
        }


        fn variable_lines(line: &&str) -> bool {
            !(line.starts_with("Engine compiled")
                || line.starts_with("Zebra (c) 1997-2005 Gunnar Andersson, compile date")
                || line.starts_with("Gunnar Andersson"))
        }

        let snapshot = snapshot_str.unwrap();
        let output = std::fs::read_to_string(result_path).expect(result_path.as_os_str().to_str().unwrap());

        let mut first = snapshot.lines().filter(variable_lines);
        let mut second = output.lines().filter(variable_lines);

        while let Some(expected) = first.next() {
            assert_eq!(expected, second.next().unwrap())
        }
        assert!(first.next().is_none());
        assert!(second.next().is_none());
    }

    snap_test!(help, "?");
    from_fuzz!(
        d1 , "-r 0 -l 1 3 8 8 1 7 -repeat 4 -h 1" , "book.bin", &[];
        d2 , "-r 0 -l 1 3 8 8 1 7 -repeat 4 -h 1" , "book.bin", &[];
        d3 , "-r 0 -l 9 4 4 7 4 8 -public -draw2none -repeat 4 -slack 0.33416033" , "book.bin", &[];
        d4, "-r 0 -l 9 3 7 5 2 6 -e 1" , "book.bin", &[];
        d5, "-r 0 -l 7 5 0 8 8 7 -repeat 3 -e 1" , "book.bin", &[];
        d6, "-r 0 -l 5 7 6 5 1 2 -slack 8.41116" , "book.bin", &[];
        d7, "-r 0 -l 8 9 1 1 4 2 -draw2black -draw2none -repeat 3 -e 1" , "book.bin", &[];
        d8, "-r 0 -l 2 4 7 9 9 6 -public -private -draw2black -draw2white -repeat 4 -slack 4.9125338" , "book.bin", &[];
        d9, "-r 0 -l 5 6 6 6 6 4 -private -slack 0.28241396 -e 0 -h 7" , "book.bin", &[];
        d10, "-r 0 -l 9 1 9 12 4 9 -keepdraw -test -slack 8.871107 -p 0 -b 0 -thor 2" , "book.bin", &[];
        d11, "-r 0 -l 9 16 11 8 18 6 -draw2none -p 0 -w 0 -thor 9 -h 4" , "book.bin", &[];
        d12, "-r 0 -l 15 19 7 9 2 19 -keepdraw -draw2black -draw2white -randmove 1 -p 0 -thor 4" , "book.bin", &[];
        d13, "-r 0 -l 0 0 -public -analyze -p 0 -g ../../tests/resources/board.txt -seq e6f6f5f4e3d6g4d3c3h3c4g3g5g6c7c6c5b6d7b5f7f3b4f8h4h5f2f1h2h1" , "book.bin", &[];

        // TODO following two tests are failing. not sure why but seems related to float precision
        d14, "-r 0 -l 0 0 -draw2black -repeat 0 -p 1 -b 1 -time 1.6256914422442514 4.570592846993699 2.5283251123785977 3.7189676525522044" , "book.bin", &[];
        d15, "-r 0 -l 8 7 2 8 3 7 -b 0 -w 1 -time 4.529340827768474 4.712636376864859 4.990958221355872 4.91143980476709" , "book.bin", &[];
        d16, "-r 0 -l 4 17 1 7 14 17 -p 0 -e 0 -time 2 0 0 2" , "book.bin", &[];
        d17, "-r 0 -l 3 4 14 9 8 19 -repeat 0 -slack 5.739609 -p 1 -e 0 -b 0 -w 0 -h 3 -time 4 3 2 1" , "book.bin", &[];
        d18, "-r 0 -l 6 10 4 8 6 4 -randmove 5 -p 0 -e 0 -h 17 -time 1 0 2 3" , "book.bin", &[];
        d19, "-r 0 -l 5 10 3 3 19 8 -private -p 0 -b 1 -dev 89 93 211.67601 -t 2 4 9 5 8 9 1 -time 2 4 5 40" , "book.bin", &[]
    );

//     creating adjust.txt
// testing args '-r 0 -l 5 17 9 1 3 11 -draw2none -time 2 0 1 3'

    // Note(matyas): I have no idea what's wrong with this one
    //  It's buffer overflow in original and panic out of bounds in rust,
    //   because disks_played is 61 roughly after midgame. Very weird bug.
    //   There's probably some place in the code where unmake_move call is missing or smth like that?
// testing args '-r 0 -l 9 6 3 5 19 0 -repeat 4 -p 1 -b 1 -w 0 -h 19 -dev 17 75 94.33498 -g ../../tests/resources/board.txt -time 40 48 6 24'
// thread 'main' panicked at 'assertion failed: second.next().is_none()', tests/src/main.rs:537:9

//similar:     testing args '-r 0 -l 7 0 14 9 4 18 -repeat 0 -randmove 5 -p 1 -e 1 -h 9 -g ../../tests/resources/board.txt -time 13 47 5 16'
//     testing args '-r 0 -l 3 6 0 4 13 12 -repeat 4 -e 0 -b 0 -g ../../tests/resources/board.txt -time 12 23 5 24'
// testing args '-r 0 -l 9 8 7 8 19 14 -randmove 4 -p 1 -e 0 -h 5 -dev 8 8 123.677826 -t 3 4 17 10 7 3 7 8 1 10 -seq e6f6f5f4e3d6g4d3c3h3c4g3g5g6c7c6c5b6d7b5f7f3b4f8h4h5f2f1 -time 4 24 24 22'

    //  index out of bounds
//      ./target/release/zebra -r 0 -l 0 0 -thor 16 -b 1 -h 20 -learn 1 3  -g fuzzer-data/seqfile-fuzzer-1 -e 1 -p 1

    // index out of bounds on weird board
    // RUST_BACKTRACE=1 BOOK_PATH=./../../fuzzer-data/books/book-343.bin COEFFS_PATH=./../../coeffs2.bin ../../target/release/zebra -l 4 3 1 3 4 1 -g ../../fuzzer-data/board-fuzzer-1 -r 1 -w 0 -b 1
//     OXOXOO--OXXOO-OOOXXXX-OXO-----O-XO-X--XXOOX-O---XXXXOXXO-OXO-XO-
// White to move
// This file was automatically generated
}
