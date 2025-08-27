mod args;
use args::EnaArgs;
use clap::Parser;
use cmd_lib::run_cmd;
use figlet_rs::FIGfont;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

/*
 * Author Gaurav Sablok
 * Date 2024-10-20
 *
 * A command line rust crate to access the ena repositories and download the data from the ena.
 * It will interact directly with the ENA archives and will give you the fastfiles address so that
 * you can download easily from the ENA without even searching on the ENA.
 * */

#[tokio::main]
async fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("REPGENERATE");
    println!("{}", repgenerate.unwrap());
    let args: EnaArgs = EnaArgs::parse();
    let start: String = String::from("https://www.ebi.ac.uk/ena/portal/api/filereport?accession=");
    let middle: String = args.project_arg.to_string();
    let end:String = String::from("&result=read_run&fields=study_accession,sample_accession,experiment_accession,run_accession,tax_id,scientific_name,fastq_ftp,submitted_ftp,sra_ftp,bam_ftp&format=tsv&download=true&limit=0");
    let finals: String = format!("{}{}{}", start, middle, end);
    let project: String = args.project_arg;
    run_cmd!(wget $finals -O $project).expect("failed");
    let f = File::open(&project).expect("file not present");
    let read = BufReader::new(f);
    let mut filewrite = File::create("FTPlinks.sh").expect("file not present");
    for i in read.lines() {
        let line = i.expect("line not present");
        let linecapture: Vec<&str> = line.split('\t').collect();
        for i in &linecapture {
            if i.contains("ftp") && i.contains("gz") {
                let intermediate: Vec<&str> = i.split(";").collect();
                for i in &intermediate {
                    writeln!(filewrite, "wget {}", i).expect("file not present");
                }
            }
        }
    }
    let _ = Command::new("sh")
        .arg("FTPlinks.sh")
        .output()
        .expect("command failed");
}
