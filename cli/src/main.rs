use std::{collections::HashMap, fs::File, io::{Write, Read}};
use prettytable::{Table, row};
use serde::{Deserialize, Serialize};
use wca::{ Args, Props, Type, Result, Command, Routine };
use serde_json;

pub( crate ) fn run_benchmarks_command() -> wca::Command
{
  wca::Command::former()
  .hint( "Run the benchmark tests" )
  .property( "crate", "specify the crate of functions to benchmark", Type::String, true )
  .phrase( "benchmarks.run" )
  .form()
}

pub( crate ) fn get_results() -> wca::Command
{
  wca::Command::former()
  .hint( "Generate a table with results" )
  .property( "function", "specify the function to get the table with results", Type::String, true )
  .phrase( "results.get" )
  .form()
}

fn parse_benchmark_output(output: &str) -> HashMap<String, String> {
  let mut results = HashMap::new();
  let lines: Vec<&str> = output.split('\n').collect();
  let mut names = Vec::new();
  let mut times = Vec::new();

  let mut current_test_name;

  for line in lines {
    if line.starts_with("Benchmarking ") {
      if let Some(test_name) = line.split_whitespace().nth(1) {
        current_test_name = test_name.to_string();
        if !names.contains(&current_test_name) {
          names.push(current_test_name)
        }
        
      }
    } 
    if line.contains("time:   [") {
      let time_str = line.split_whitespace().collect::<Vec<_>>();
      for i in 0..time_str.len() {
        if time_str[i].contains(".") {
          let mut res = time_str[i + 2].to_string();
          res.push(' ');
          res.push_str(time_str[i + 3]);
          times.push(res);
          break;
        } 
      }
    }
  }

  for i in 0..times.len() {
    results.insert(names[i * 2].clone(), times[i].clone());
  }

  results
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    map: HashMap<String, String>,
}

fn write_to_file(map: &HashMap<String, String>, file_path: &str) -> std::io::Result<()> {
  let data = Data { map: map.clone() };
  let serialized = serde_json::to_string(&data)?;

  let mut file = File::create(file_path)?;
  file.write_all(serialized.as_bytes())?;
  Ok(())
}

fn read_from_file(file_path: &str) -> std::io::Result<HashMap<String, String>> {
  let mut file = File::open(file_path)?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;

  let data: Data = serde_json::from_str(&content)?;
  Ok(data.map)
}

pub( crate ) fn benchmarks( ( _, props ) : ( Args, Props ) ) -> Result< () >
{
  let output = match props.get_owned( "crate" ) 
  {
    Some( "ndarray" ) => {
      std::process::Command::new("cargo")
      .arg("bench")
      .arg("--package")
      .arg("ndarray_example")
      .output()
      .expect("Error while running tests")
    },
    Some( "opencl" ) => {
      std::process::Command::new("cargo")
      .arg("bench")
      .arg("--package")
      .arg("opencl_example")
      .output()
      .expect("Error while running tests")
    },
    Some( "rayon" ) => {
      std::process::Command::new("cargo")
      .arg("bench")
      .arg("--package")
      .arg("rayon-example")
      .output()
      .expect("Error while running tests")
    },
    Some( "tch" ) => {
      std::process::Command::new("cargo")
      .arg("bench")
      .arg("--package")
      .arg("tch-example")
      .output()
      .expect("Error while running tests")
    },
    Some( "wgsl" ) => {
      std::process::Command::new("cargo")
      .arg("bench")
      .arg("--package")
      .arg("wgsl-example")
      .output()
      .expect("Error while running tests")
    },
    _ => {
      let r = std::process::Command::new("cargo")
      .arg("bench")
      .output()
      .expect("Error while running tests");

      let res = parse_benchmark_output(&String::from_utf8_lossy(&r.stdout));

      write_to_file(&res, "results.json").unwrap();

      r
    },
  };

  let res = parse_benchmark_output(&String::from_utf8_lossy(&output.stdout));



  if output.status.success() {
    println!("Benchmark tests were successfully executed");

    println!("stdout: ");
    for (key, value) in &res {
      println!("{}: {}", key, value);
    }
  } else {
    eprintln!("Error while executing tests");

    eprintln!("stderr: {:?}", String::from_utf8_lossy(&output.stderr));
  }
  
  Ok( () )
}

fn draw_table(results: HashMap<String, String>, variant: i32) {
  match variant {
      1 => {
        //add arrays
        let mut vectors_add_table = Table::new();
        vectors_add_table.add_row( row![ "Function name", "Device", "Dry Run time", "Single operation time", "Batch operations(1.00e+03) time", "Batch operations(1.00e+06) time" ] );
        vectors_add_table.add_row( row![ 
          "ndarray", 
          "CPU", 
          results.get("dry_run_add_arrays_ndarray").unwrap_or(&"-".to_string()), 
          results.get("add_arrays_ndarray_one").unwrap_or(&"-".to_string()), 
          results.get("add_arrays_ndarray_batch_1000").unwrap_or(&"-".to_string()), 
          results.get("add_arrays_ndarray_batch_100000").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_add_table.add_row( row![ 
          "rayon", 
          "CPU", 
          results.get("dry_run_add_arrays_rayon").unwrap_or(&"-".to_string()), 
          results.get("add_arrays_rayon_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_add_arrays_rayon").unwrap_or(&"-".to_string()), 
          results.get("batch1000000_add_arrays_rayon").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_add_table.add_row( row![ 
          "tch", 
          "CPU", 
          results.get("dry_run_add_arrays_tch").unwrap_or(&"-".to_string()), 
          results.get("add_arrays_tch_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_add_arrays_tch").unwrap_or(&"-".to_string()), 
          results.get("batch1000000_add_arrays_tch").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_add_table.add_row( row![ 
          "tch", 
          "CPU", 
          results.get("dry_run_add_arrays_tch").unwrap_or(&"-".to_string()), 
          results.get("add_arrays_tch_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_add_arrays_tch").unwrap_or(&"-".to_string()), 
          results.get("batch1000000_add_arrays_tch").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_add_table.add_row( row![ 
          "opencl", 
          "GPU", 
          results.get("dry_run_add_vectors_opencl").unwrap_or(&"-".to_string()), 
          results.get("add_vectors_opencl_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_add_vectors_opencl").unwrap_or(&"-".to_string()), 
          results.get("batch1000000_add_vectors_opencl").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_add_table.add_row( row![ 
          "wgsl", 
          "GPU", 
          results.get("dry_run_add_arrays_wgsl").unwrap_or(&"-".to_string()), 
          results.get("add_arrays_wgsl_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_add_arrays_wgsl").unwrap_or(&"-".to_string()), 
          results.get("batch100000_add_arrays_wgsl").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_add_table.add_row( row![ 
          "rust", 
          "CPU", 
          results.get("dry_run_add_arrays_rust").unwrap_or(&"-".to_string()), 
          results.get("add_arrays_rust_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_add_arrays_rust").unwrap_or(&"-".to_string()), 
          results.get("batch100000_add_arrays_rust").unwrap_or(&"-".to_string()), 
          ] 
        );

        println!("Sum of two vectors");
        vectors_add_table.printstd();
      }
      2 => {
        //sum arrays
        let mut vectors_sum_table = Table::new();
        vectors_sum_table.add_row( row![ "Function name", "Device", "Dry Run time", "Single operation time", "Batch operations(1.00e+03) time", "Batch operations(1.00e+06) time" ] );
        vectors_sum_table.add_row( row![ 
          "ndarray", 
          "CPU", 
          results.get("dry_run_sum_arrays_ndarray").unwrap_or(&"-".to_string()), 
          results.get("sum_arrays_ndarray_one").unwrap_or(&"-".to_string()), 
          results.get("sum_arrays_ndarray_batch_1000").unwrap_or(&"-".to_string()), 
          results.get("sum_arrays_ndarray_batch_1000000").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_sum_table.add_row( row![ 
          "rayon", 
          "CPU", 
          results.get("dry_run_sum_arrays_rayon").unwrap_or(&"-".to_string()), 
          results.get("sum_arrays_rayon_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_sum_arrays_rayon").unwrap_or(&"-".to_string()), 
          results.get("batch1000000_sum_arrays_rayon").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_sum_table.add_row( row![ 
          "tch", 
          "CPU", 
          results.get("dry_run_sum_arrays_tch").unwrap_or(&"-".to_string()), 
          results.get("sum_arrays_tch_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_sum_arrays_tch").unwrap_or(&"-".to_string()), 
          results.get("batch1000000_sum_arrays_tch").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_sum_table.add_row( row![ 
          "tch", 
          "CPU", 
          results.get("dry_run_sum_arrays_tch").unwrap_or(&"-".to_string()), 
          results.get("sum_arrays_tch_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_sum_arrays_tch").unwrap_or(&"-".to_string()), 
          results.get("batch1000000_sum_arrays_tch").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_sum_table.add_row( row![ 
          "opencl", 
          "GPU", 
          results.get("dry_run_sum_vectors_opencl").unwrap_or(&"-".to_string()), 
          results.get("sum_vectors_opencl_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_sum_vectors_opencl").unwrap_or(&"-".to_string()), 
          results.get("batch1000000_sum_vectors_opencl").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_sum_table.add_row( row![ 
          "wgsl", 
          "GPU", 
          results.get("dry_run_sum_arrays_wgsl").unwrap_or(&"-".to_string()), 
          results.get("sum_arrays_wgsl_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_sum_arrays_wgsl").unwrap_or(&"-".to_string()), 
          results.get("batch100000_sum_arrays_wgsl").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_sum_table.add_row( row![ 
          "rust", 
          "CPU", 
          results.get("dry_run_sum_arrays_rust").unwrap_or(&"-".to_string()), 
          results.get("sum_arrays_rust_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_sum_arrays_rust").unwrap_or(&"-".to_string()), 
          results.get("batch100000_sum_arrays_rust").unwrap_or(&"-".to_string()), 
          ] 
        );

        println!("Sum of vec elements");
        vectors_sum_table.printstd();
      }
      3 => {
        //optimized sum
        let mut vectors_optimized_sum_table = Table::new();
        vectors_optimized_sum_table.add_row( row![ "Function name", "Device", "Dry Run time", "Single operation time", "Batch operations(1.00e+03) time", "Batch operations(1.00e+06) time" ] );
        vectors_optimized_sum_table.add_row( row![ 
          "ndarray", 
          "CPU", 
          results.get("dry_run_optimized_sum_arrays_ndarray").unwrap_or(&"-".to_string()), 
          results.get("optimized_sum_arrays_ndarray_one").unwrap_or(&"-".to_string()), 
          results.get("optimized_sum_arrays_ndarray_batch_1000").unwrap_or(&"-".to_string()), 
          results.get("optimized_sum_arrays_ndarray_batch_1000000").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_optimized_sum_table.add_row( row![ 
          "rayon", 
          "CPU", 
          results.get("dry_run_optimized_sum_arrays_rayon").unwrap_or(&"-".to_string()), 
          results.get("optimized_sum_arrays_rayon_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_optimized_sum_arrays_rayon").unwrap_or(&"-".to_string()), 
          results.get("batch1000000_optimized_sum_arrays_rayon").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_optimized_sum_table.add_row( row![ 
          "tch", 
          "CPU", 
          results.get("dry_run_optimized_sum_arrays_tch").unwrap_or(&"-".to_string()), 
          results.get("optimized_sum_arrays_tch_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_optimized_sum_arrays_tch").unwrap_or(&"-".to_string()), 
          results.get("batch1000000_optimized_sum_arrays_tch").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_optimized_sum_table.add_row( row![ 
          "tch", 
          "CPU", 
          results.get("dry_run_optimized_sum_arrays_tch").unwrap_or(&"-".to_string()), 
          results.get("optimized_sum_arrays_tch_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_optimized_sum_arrays_tch").unwrap_or(&"-".to_string()), 
          results.get("batch1000000_optimized_sum_arrays_tch").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_optimized_sum_table.add_row( row![ 
          "opencl", 
          "GPU", 
          results.get("dry_run_optimized_sum_vectors_opencl").unwrap_or(&"-".to_string()), 
          results.get("optimized_sum_vectors_opencl_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_optimized_sum_vectors_opencl").unwrap_or(&"-".to_string()), 
          results.get("batch1000000_optimized_sum_vectors_opencl").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_optimized_sum_table.add_row( row![ 
          "wgsl", 
          "GPU", 
          results.get("dry_run_optimized_sum_arrays_wgsl").unwrap_or(&"-".to_string()), 
          results.get("optimized_sum_arrays_wgsl_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_optimized_sum_arrays_wgsl").unwrap_or(&"-".to_string()), 
          results.get("batch100000_optimized_sum_arrays_wgsl").unwrap_or(&"-".to_string()), 
          ] 
        );
        vectors_optimized_sum_table.add_row( row![ 
          "rust", 
          "CPU", 
          results.get("dry_run_optimized_sum_arrays_rust").unwrap_or(&"-".to_string()), 
          results.get("optimized_sum_arrays_rust_one").unwrap_or(&"-".to_string()), 
          results.get("batch1000_optimized_sum_arrays_rust").unwrap_or(&"-".to_string()), 
          results.get("batch100000_optimized_sum_arrays_rust").unwrap_or(&"-".to_string()), 
          ] 
        );

        println!("Sum of vec elements(optimized)");
        vectors_optimized_sum_table.printstd();
      }
      _ => {}
  }
}

pub( crate ) fn table( ( _, props ) : ( Args, Props ) ) -> Result< () >
{
  let temp_map = read_from_file("results.json").unwrap();
  match props.get_owned( "function" ) 
  {
    Some( "vec_add" ) => {
      draw_table(temp_map, 1);
    }
    Some("vec_sum") => {
      draw_table(temp_map, 2);
    }
    Some("optimized_vec_sum") => {
      draw_table(temp_map, 3);
    }
    _ => {
      draw_table(temp_map.clone(), 1);
      println!("");
      draw_table(temp_map.clone(), 2);
      println!("");
      draw_table(temp_map.clone(), 3);
      println!("");
    }
  }

  Ok( () )
}

/// A command that runs a command
pub fn grammar_form() -> Vec< Command >
{
  vec!
  [
    run_benchmarks_command(),
    get_results()
  ]
}

/// A command that creates a command
pub fn executor_form() -> HashMap< String, Routine >
{
  HashMap::from
  ([
    ( "benchmarks.run".to_owned(), Routine::new( benchmarks ) ),
    ( "results.get".to_owned(), Routine::new( table ) ),
  ])
}

fn main()
{
  let ca = wca::CommandsAggregator::former()
  .grammar( grammar_form() )
  .executor( executor_form() )
  .build();

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();

  ca.perform( args.join( " " ).as_str() ).unwrap();
}