/*
 * Author Gaurav Sablok
 * Universitat Potsdam
 * Date 2024-10-20
 *
 * A command line rust crate to access the ena repositories and download the data from the ena.
 * It will interact directly with the ENA archives and will give you the fastfiles address so that
 * you can download easily from the ENA without even searching on the ENA.
 *
 *
 *
 * */


mod args;
use std::env::Args;
use cmd_lib::run_cmd;
use std::fs::File;
use std::io::Read;
use std::fmt::Display;
use args::EnaArgs;
use clap::Parser;
use std::io::{BufReader, BufRead};

fn main() {
    let args:EnaArgs = EnaArgs::parse();
let mut start:String = String::from("https://www.ebi.ac.uk/ena/portal/api/filereport?accession=");
let mut middle:String = args.project_arg.to_string();
let mut end:String = String::from("&result=read_run&fields=study_accession,sample_accession,experiment_accession,run_accession,tax_id,scientific_name,fastq_ftp,submitted_ftp,sra_ftp,bam_ftp&format=tsv&download=true&limit=0");
let finals:String = format!("{}{}{}",start,middle,end);
let mut project:String = args.project_arg;
run_cmd!(wget $finals -O $project);
let f = File::open(&project).expect("file not present");
let read = BufReader::new(f);
for i in read.lines() {
    let line = i
               .expect("line not present");
            let linecapture: Vec<&str> = line.split('\t').collect();
            for i in &linecapture {
                if i.contains("ftp") && i.contains("gz"){
                let intermediate:Vec<&str> = i.split(";").collect();
                for i in &intermediate {
                    println!("wget {}", i)
            }
            }
        }
}
}
