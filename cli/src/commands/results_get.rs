use prettytable::{row, Table};
use std::collections::HashMap;
use wca::{Args, Props, Result, Type};

use crate::read_from_file;

pub(crate) fn get_results() -> wca::Command {
    wca::Command::former()
        .hint("Generate a table with results")
        .property(
            "function",
            "specify the function to get the table with results",
            Type::String,
            true,
        )
        .phrase("results.get")
        .form()
}

fn draw_table(results: HashMap<String, String>, variant: i32) -> Table {
    let mut table = Table::new();
    table.add_row(row![
        "Function name",
        "Device",
        "Dry Run time",
        "Single operation time",
        "Batch operations(1.00e+03) time",
        "Batch operations(1.00e+06) time"
    ]);
    match variant {
        1 => {
            //add arrays
            table.add_row(row![
                "ndarray",
                "CPU",
                results
                    .get("dry_run_add_arrays_ndarray")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("add_arrays_ndarray_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("add_arrays_ndarray_batch_1000")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("add_arrays_ndarray_batch_100000")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "rayon",
                "CPU",
                results
                    .get("dry_run_add_arrays_rayon")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("add_arrays_rayon_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_add_arrays_rayon")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000000_add_arrays_rayon")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "tch",
                "CPU",
                results
                    .get("dry_run_add_arrays_tch")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("add_arrays_tch_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_add_arrays_tch")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000000_add_arrays_tch")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "opencl",
                "GPU",
                results
                    .get("dry_run_add_vectors_opencl")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("add_vectors_opencl_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_add_vectors_opencl")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000000_add_vectors_opencl")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "wgsl",
                "GPU",
                results
                    .get("dry_run_add_arrays_wgsl")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("add_arrays_wgsl_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_add_arrays_wgsl")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch100000_add_arrays_wgsl")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "rust",
                "CPU",
                results
                    .get("dry_run_add_arrays_rust")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("add_arrays_rust_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_add_arrays_rust")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch100000_add_arrays_rust")
                    .unwrap_or(&"-".to_string()),
            ]);
        }
        2 => {
            //sum arrays
            table.add_row(row![
                "ndarray",
                "CPU",
                results
                    .get("dry_run_sum_arrays_ndarray")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("sum_arrays_ndarray_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("sum_arrays_ndarray_batch_1000")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("sum_arrays_ndarray_batch_1000000")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "rayon",
                "CPU",
                results
                    .get("dry_run_sum_arrays_rayon")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("sum_arrays_rayon_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_sum_arrays_rayon")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000000_sum_arrays_rayon")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "tch",
                "CPU",
                results
                    .get("dry_run_sum_arrays_tch")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("sum_arrays_tch_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_sum_arrays_tch")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000000_sum_arrays_tch")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "opencl",
                "GPU",
                results
                    .get("dry_run_sum_vectors_opencl")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("sum_vectors_opencl_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_sum_vectors_opencl")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000000_sum_vectors_opencl")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "wgsl",
                "GPU",
                results
                    .get("dry_run_sum_arrays_wgsl")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("sum_arrays_wgsl_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_sum_arrays_wgsl")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch100000_sum_arrays_wgsl")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "rust",
                "CPU",
                results
                    .get("dry_run_sum_arrays_rust")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("sum_arrays_rust_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_sum_arrays_rust")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch100000_sum_arrays_rust")
                    .unwrap_or(&"-".to_string()),
            ]);
        }
        3 => {
            //optimized sum
            table.add_row(row![
                "ndarray",
                "CPU",
                results
                    .get("dry_run_optimized_sum_arrays_ndarray")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("optimized_sum_arrays_ndarray_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("optimized_sum_arrays_ndarray_batch_1000")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("optimized_sum_arrays_ndarray_batch_1000000")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "rayon",
                "CPU",
                results
                    .get("dry_run_optimized_sum_arrays_rayon")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("optimized_sum_arrays_rayon_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_optimized_sum_arrays_rayon")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000000_optimized_sum_arrays_rayon")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "tch",
                "CPU",
                results
                    .get("dry_run_optimized_sum_arrays_tch")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("optimized_sum_arrays_tch_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_optimized_sum_arrays_tch")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000000_optimized_sum_arrays_tch")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "opencl",
                "GPU",
                results
                    .get("dry_run_optimized_sum_vectors_opencl")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("optimized_sum_vectors_opencl_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_optimized_sum_vectors_opencl")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000000_optimized_sum_vectors_opencl")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "wgsl",
                "GPU",
                results
                    .get("dry_run_optimized_sum_arrays_wgsl")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("optimized_sum_arrays_wgsl_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_optimized_sum_arrays_wgsl")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch100000_optimized_sum_arrays_wgsl")
                    .unwrap_or(&"-".to_string()),
            ]);
            table.add_row(row![
                "rust",
                "CPU",
                results
                    .get("dry_run_optimized_sum_arrays_rust")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("optimized_sum_arrays_rust_one")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch1000_optimized_sum_arrays_rust")
                    .unwrap_or(&"-".to_string()),
                results
                    .get("batch100000_optimized_sum_arrays_rust")
                    .unwrap_or(&"-".to_string()),
            ]);
        }
        _ => {}
    }

    table
}

pub(crate) fn table((_, props): (Args, Props)) -> Result<()> {
    let temp_map = read_from_file("results.json").unwrap();
    match props.get_owned("function") {
        Some("vec_add") => {
            println!("Sum of vec elements");
            draw_table(temp_map, 1).printstd();
        }
        Some("vec_sum") => {
            println!("Sum of vec elements");
            draw_table(temp_map, 2).printstd();
        }
        Some("optimized_vec_sum") => {
            println!("Sum of vec elements(optimized)");
            draw_table(temp_map, 3).printstd();
        }
        _ => {
            println!("Sum of vec elements");
            draw_table(temp_map.clone(), 1).printstd();
            println!("");
            println!("Sum of vec elements");
            draw_table(temp_map.clone(), 2).printstd();
            println!("");
            println!("Sum of vec elements(optimized)");
            draw_table(temp_map.clone(), 3).printstd();
            println!("");
        }
    }

    Ok(())
}
