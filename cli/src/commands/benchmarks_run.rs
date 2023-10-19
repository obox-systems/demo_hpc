use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};
use wca::{Args, Props, Result, Type};

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

pub(crate) fn read_from_file(file_path: &str) -> std::io::Result<HashMap<String, String>> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let data: Data = serde_json::from_str(&content)?;
    Ok(data.map)
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

pub(crate) fn run_benchmarks_command() -> wca::Command {
    wca::Command::former()
        .hint("Run the benchmark tests")
        .property(
            "crate",
            "specify the crate of functions to benchmark",
            Type::String,
            true,
        )
        .phrase("benchmarks.run")
        .form()
}

pub(crate) fn benchmarks((_, props): (Args, Props)) -> Result<()> {
    let output = match props.get_owned("crate") {
        Some("ndarray") => std::process::Command::new("cargo")
            .arg("bench")
            .arg("--package")
            .arg("ndarray_example")
            .output()
            .expect("Error while running tests"),
        Some("opencl") => std::process::Command::new("cargo")
            .arg("bench")
            .arg("--package")
            .arg("opencl_example")
            .output()
            .expect("Error while running tests"),
        Some("rayon") => std::process::Command::new("cargo")
            .arg("bench")
            .arg("--package")
            .arg("rayon-example")
            .output()
            .expect("Error while running tests"),
        Some("tch") => std::process::Command::new("cargo")
            .arg("bench")
            .arg("--package")
            .arg("tch-example")
            .output()
            .expect("Error while running tests"),
        Some("wgsl") => std::process::Command::new("cargo")
            .arg("bench")
            .arg("--package")
            .arg("wgsl-example")
            .output()
            .expect("Error while running tests"),
        _ => {
            let r = std::process::Command::new("cargo")
                .arg("bench")
                .output()
                .expect("Error while running tests");

            let res = parse_benchmark_output(&String::from_utf8_lossy(&r.stdout));

            write_to_file(&res, "results.json").unwrap();

            r
        }
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

    Ok(())
}
