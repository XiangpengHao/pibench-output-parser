use regex::{self, Regex};
use serde::Serialize;
use serde_json;
use std::error;
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
mod c_ffi;

#[cfg(not(target_arch = "wasm32"))]
pub use c_ffi::*;

#[wasm_bindgen]
#[derive(PartialOrd, PartialEq, Debug, Serialize, Copy, Clone)]
pub enum KeyDistribution {
    UNIFORM = 0,
    ZIPFAN = 1,
}

#[wasm_bindgen]
#[derive(PartialEq, PartialOrd, Debug, Serialize, Copy, Clone)]
pub struct BenchmarkOptions {
    sampling: i32,
    latency: f32,
    key_size: i32,
    value_size: i32,
    random_seed: i32,
    read: f32,
    insert: f32,
    update: f32,
    delete: f32,
    scan: f32,
    key_distribution: KeyDistribution,
    records: i32,
    operations: i32,
    threads: i32,
}

impl BenchmarkOptions {
    pub fn from_text(text: &str) -> Result<BenchmarkOptions, Box<dyn error::Error>> {
        let re = Regex::new(
            "# Records: (?P<records>\\d+)\\s*\n\
                \\s*# Operations: (?P<operations>\\d+)\\s*\n\
               \\s*# Threads: (?P<threads>\\d+)\\s*\n\
               \\s*Sampling: (?P<sampling>\\d+) ms\\s*\n\
               \\s*Latency: (?P<latency>\\d*\\.?\\d*)\\s*\n\
               .*\n\
               \\s*Key size: (?P<key_size>\\d+)\\s*\n\
               \\s*Value size: (?P<value_size>\\d+)\\s*\n\
               \\s*Random seed: (?P<random_seed>\\d+)\\s*\n\
               \\s*Key distribution: (?P<key_distribution>\\w+)\\s*\n\
               \\s*Scan size: (?P<scan_size>\\d+)\\s*\n\
               .*\n\
               \\s*Read: (?P<read>\\d*\\.?\\d*)\\s*\n\
               \\s*Insert: (?P<insert>\\d*\\.?\\d*)\\s*\n\
               \\s*Update: (?P<update>\\d*\\.?\\d*)\\s*\n\
               \\s*Delete: (?P<delete>\\d*\\.?\\d*)\\s*\n\
               \\s*Scan: (?P<scan>\\d*\\.?\\d*)\\s*",
        )
        .unwrap();
        let caps = re.captures(text).unwrap();

        Ok(BenchmarkOptions {
            sampling: caps["sampling"].parse::<i32>()?,
            records: caps["records"].parse::<i32>()?,
            threads: caps["threads"].parse::<i32>()?,
            operations: caps["operations"].parse::<i32>()?,
            latency: caps["latency"].parse::<f32>()?,
            key_size: caps["key_size"].parse::<i32>()?,
            key_distribution: KeyDistribution::UNIFORM,
            value_size: caps["value_size"].parse::<i32>()?,
            random_seed: caps["random_seed"].parse::<i32>()?,
            read: caps["read"].parse::<f32>()?,
            insert: caps["insert"].parse::<f32>()?,
            delete: caps["delete"].parse::<f32>()?,
            update: caps["update"].parse::<f32>()?,
            scan: caps["scan"].parse::<f32>()?,
        })
    }
}

#[wasm_bindgen]
#[derive(Eq, PartialEq, PartialOrd, Debug, Serialize, Copy, Clone)]
pub struct LatencyResults {
    min: i32,
    p_50: i32,
    p_90: i32,
    p_99: i32,
    p_99_9: i32,
    p_99_99: i32,
    p_99_999: i32,
    max: i32,
}

impl LatencyResults {
    pub fn from_text(text: &str) -> Result<Option<LatencyResults>, Box<dyn error::Error>> {
        let re = Regex::new(
            "Latencies .*\n\
            \\s*min: (?P<min>\\d+)\\s*\n\
            \\s*50%: (?P<p_50>\\d+)\\s*\n\
            \\s*90%: (?P<p_90>\\d+)\\s*\n\
            \\s*99%: (?P<p_99>\\d+)\\s*\n\
            \\s*99.9%: (?P<p_99_9>\\d+)\\s*\n\
            \\s*99.99%: (?P<p_99_99>\\d+)\\s*\n\
            \\s*99.999%: (?P<p_99_999>\\d+)\\s*\n\
            \\s*max: (?P<max>\\d+)\\s*",
        )?;
        let caps = match re.captures(text) {
            Some(caps) => caps,
            None => return Ok(None),
        };
        Ok(Some(LatencyResults {
            min: caps["min"].parse::<i32>()?,
            p_50: caps["p_50"].parse::<i32>()?,
            p_90: caps["p_90"].parse::<i32>()?,
            p_99: caps["p_99"].parse::<i32>()?,
            p_99_9: caps["p_99_9"].parse::<i32>()?,
            p_99_99: caps["p_99_99"].parse::<i32>()?,
            p_99_999: caps["p_99_999"].parse::<i32>()?,
            max: caps["max"].parse::<i32>()?,
        }))
    }
}

#[wasm_bindgen]
#[derive(PartialOrd, Eq, PartialEq, Debug, Clone, Serialize)]
pub struct PCMResults {
    l3_misses: u64,
    dram_reads: u64,
    dram_writes: u64,
    nvm_reads: u64,
    nvm_writes: u64,
}

impl PCMResults {
    pub fn from_text(text: &str) -> Result<Option<PCMResults>, Box<dyn error::Error>> {
        let regex_raw = "\\s*L3 misses: (?P<l3_misses>\\d+)\\s*\n\
            \\s*DRAM Reads \\(bytes\\): (?P<dram_reads>\\d+)\\s*\n\
            \\s*DRAM Writes \\(bytes\\): (?P<dram_writes>\\d+)\\s*\n\
            \\s*NVM Reads \\(bytes\\): (?P<nvm_reads>\\d+)\\s*\n\
            \\s*NVM Writes \\(bytes\\): (?P<nvm_writes>\\d+)\\s*";
        let caps = match Regex::new(&regex_raw)?.captures(text) {
            Some(caps) => caps,
            None => return Ok(None),
        };

        Ok(Some(PCMResults {
            l3_misses: caps["l3_misses"].parse::<u64>()?,
            dram_reads: caps["dram_reads"].parse::<u64>()?,
            dram_writes: caps["dram_writes"].parse::<u64>()?,
            nvm_reads: caps["nvm_reads"].parse::<u64>()?,
            nvm_writes: caps["nvm_writes"].parse::<u64>()?,
        }))
    }
}

#[wasm_bindgen]
#[derive(PartialEq, PartialOrd, Debug, Serialize, Clone)]
pub struct BenchmarkResults {
    load_time: f32,
    run_time: f32,
    throughput: f32,
    pcm: Option<PCMResults>,
    latency: Option<LatencyResults>,
    samples: Option<Vec<u32>>,
}

impl BenchmarkResults {
    pub fn from_text(text: &str) -> Result<BenchmarkResults, Box<dyn error::Error>> {
        const FLOATING_REGEX: &str = "[+\\-]?(?:0|[1-9]\\d*)(?:\\.\\d*)?(?:[eE][+\\-]?\\d+)?";
        let regex_raw = format!(
            "Load time: (?P<load_time>{floating}) milliseconds\\s*\n\
            \\s*Run time: (?P<run_time>{floating}) milliseconds\\s*\n\
            \\s*Throughput: (?P<throughput>{floating}) ops/s\n",
            floating = FLOATING_REGEX
        );
        let re = Regex::new(&regex_raw)?;
        let caps = re.captures(text).unwrap();

        let latency_results = LatencyResults::from_text(text)?;
        let pcm_results = PCMResults::from_text(text)?;
        let samples = BenchmarkResults::capture_samples(text)?;

        Ok(BenchmarkResults {
            load_time: caps["load_time"].parse::<f32>()?,
            run_time: caps["run_time"].parse::<f32>()?,
            throughput: caps["throughput"].parse::<f32>()?,
            pcm: pcm_results,
            latency: latency_results,
            samples,
        })
    }

    pub fn capture_samples(text: &str) -> Result<Option<Vec<u32>>, Box<dyn error::Error>> {
        let regex_raw = "\\s*Samples:\\n(?P<samples>[\\s\\d+\\n?]*)";
        let caps = match Regex::new(&regex_raw)?.captures(text) {
            Some(caps) => caps,
            None => return Ok(None),
        };
        let samples = caps["samples"]
            .split("\n")
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
            .map(|item| item.parse::<u32>())
            .collect::<Result<Vec<_>, _>>();
        Ok(Some(samples?))
    }
}
#[wasm_bindgen]
#[derive(Serialize)]
pub struct PiBenchData {
    options: BenchmarkOptions,
    results: BenchmarkResults,
}

impl PiBenchData {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[wasm_bindgen]
impl PiBenchData {
    pub fn from_text(input: &str) -> Option<PiBenchData> {
        let benchmark_options = BenchmarkOptions::from_text(input);
        let benchmark_results = BenchmarkResults::from_text(input);
        if benchmark_options.is_err() || benchmark_results.is_err() {
            return None;
        }
        return Some(PiBenchData {
            options: benchmark_options.unwrap(),
            results: benchmark_results.unwrap(),
        });
    }
    pub fn to_js_value(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn parse_benchmark_results() {
        let sample_string = "Overview:
                                    Load time: 90801.3 milliseconds
                                    Run time: 79192.3672 milliseconds
                                    Throughput: 126274.7969 ops/s
                                PCM Metrics:
                                    L3 misses: 133342466
                                    DRAM Reads (bytes): 4197345472
                                    DRAM Writes (bytes): 3685394624
                                    NVM Reads (bytes): 60347831872
                                    NVM Writes (bytes): 11408209856
                                Samples:
                                	135452
	                                126077
	                                109243
                                ";
        let gt = BenchmarkResults {
            load_time: 90801.3,
            run_time: 79192.3672,
            throughput: 126274.7969,
            pcm: Some(PCMResults {
                l3_misses: 133342466,
                dram_reads: 4197345472,
                dram_writes: 3685394624,
                nvm_reads: 60347831872,
                nvm_writes: 11408209856,
            }),
            latency: None,
            samples: Some(vec![135452, 126077, 109243]),
        };
        let result = BenchmarkResults::from_text(sample_string);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), gt);
    }

    #[test]
    fn parse_latency_results() {
        let sample_string = "Latencies (998141 operations observed):
                                    min: 882
                                    50%: 7481
                                    90%: 9121
                                    99%: 43233
                                    99.9%: 51150
                                    99.99%: 69460
                                    99.999%: 16985300
                                    max: 22247728
                                ";
        let gt = LatencyResults {
            min: 882,
            p_50: 7481,
            p_90: 9121,
            p_99: 43233,
            p_99_9: 51150,
            p_99_99: 69460,
            p_99_999: 16985300,
            max: 22247728,
        };
        let latency = LatencyResults::from_text(sample_string);
        assert!(latency.is_ok());
        assert_eq!(latency.unwrap().unwrap(), gt);
    }

    #[test]
    fn parse_benchmark_options() {
        let sample_string = " Environment:
                                    Time: Sat May  9 13:39:56 2020
                                    CPU: 96 * Intel(R) Xeon(R) Gold 6252 CPU @ 2.10GHz
                                    CPU Cache: 36608 KB
                                    Kernel: Linux 5.5.4-arch1-1
                                    Benchmark Options:
                                    Target: /home/hao/coding/bztree/release/libbztree_pibench_wrapper.so
                                    # Records: 10000000
                                    # Operations: 10000000
                                    # Threads: 1
                                    Sampling: 1000 ms
                                    Latency: 0.1
                                    Key prefix: 
                                    Key size: 8
                                    Value size: 8
                                    Random seed: 1729
                                    Key distribution: UNIFORM
                                    Scan size: 100
                                    Operations ratio:
                                        Read: 0.2
                                        Insert: 0.8
                                        Update: 0
                                        Delete: 0
                                        Scan: 0
                                creating new tree on pool.
                                ";
        let gt = BenchmarkOptions {
            records: 10000000,
            operations: 10000000,
            threads: 1,
            sampling: 1000,
            latency: 0.1,
            key_size: 8,
            value_size: 8,
            random_seed: 1729,
            key_distribution: KeyDistribution::UNIFORM,
            scan: 0.,
            read: 0.2,
            insert: 0.8,
            update: 0.,
            delete: 0.,
        };
        let options = BenchmarkOptions::from_text(sample_string);
        assert!(options.is_ok());
        assert_eq!(options.unwrap(), gt);
    }
}
