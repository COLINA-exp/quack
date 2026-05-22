use std::io::Result;
use std::io::{BufRead, BufReader};
use std::fs::{read_to_string, File};
use std::str::FromStr;

use ndarray::Array2;

//use rayon::prelude::*;

pub struct DaqEvent {
    pub number   : u32,
    pub time     : u64,
    pub sampling : f32,
    pub waveforms: Array2<f32>,
}

fn get_token<T: FromStr>(line: &str, index: usize, label: &str) -> T {
    line.trim()
        .split_whitespace()
        .nth(index)
        .and_then(|x| x.parse().ok())
        .expect(&format!("Could not parse {label}"))
}

fn get_channels(line: &str, channels: &Vec<usize>) -> impl Iterator<Item=f32> {
    line.trim()
        .split_whitespace()
        .skip(1)
        .enumerate()
        .filter(|(i,_)| channels.contains(i))
        .map(|t| t.1)
        .map(|token| token.parse().expect(&format!("Could not parse token: {token}")))
}

pub fn read_waveform_length(filename: &str) -> usize {
    let reader = BufReader::new(File::open(filename).expect("Could not open first file"));
    for line in reader.lines() {
        let line = line.expect("Failure reading file header");
        if line.starts_with("Samples") {
            return line.trim()
                       .split_whitespace()
                       .nth(1)
                       .and_then(|x| x.parse().ok())
                       .expect("Could not read waveform length");
        }
    }
    unreachable!();
}

pub fn read_event(chunk: &str, channels: &Vec<usize>) -> DaqEvent {
    let mut meta : Vec<&str> = chunk.split("\n").collect();
    let     waves            = meta.split_off(5);

    let nchannels = channels.len();
    let number    = get_token::<u32>  (&meta[0], 0, "event number");
    let time      = get_token::<u64>  (&meta[1], 1, "time stamp");
    let nsamples  = get_token::<usize>(&meta[2], 1, "number of samples");
    let sampling  = get_token::<f32>  (&meta[3], 3, "sampling time");
    let waveforms = Array2::from_shape_vec((nchannels, nsamples),
        waves.into_iter()
             .flat_map(|line| get_channels(&line, &channels))
             .collect::<Vec<f32>>()
    ).expect("Could not create waveform array");

    DaqEvent{number, time, sampling, waveforms}
}

pub fn read_daq_file(filename: &str, channels: &Vec<usize>) -> Result<Vec<DaqEvent>> {
    Ok(
        read_to_string(filename)?
            .split("Event n. ")
            .skip(1)
            // .par_bridge()
            // .into_par_iter()
            .map(|chunk| read_event(chunk, channels))
            .collect()
    )
}
