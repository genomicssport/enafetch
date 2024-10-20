/*
 * Author Gaurav Sablok
 * Universitat Potsdam
 * Date 2024-10-20
 *
 * A command line rust crate to access the ena repositories and download the data from the ena.
 *
 *
 *
 * */


mod args;
use std::env::Args;
use cmd_lib::run_cmd;
use std::fs;
use args::EnaArgs;
use clap::Parser;

fn main() {


    let args:EnaArgs = EnaArgs::parse();
     let mut start:String = String::from("https://www.ebi.ac.uk/ena/portal/api/filereport?accession=");
     let mut middle:String = args.project_arg.to_string();
     let mut end:String = String::from("&result=read_run&fields=study_accession,sample_accession,experiment_accession,run_accession,tax_id,scientific_name,fastq_ftp,submitted_ftp,sra_ftp,bam_ftp&format=tsv&download=true&limit=0");
     let finals:String = format!("{}{}{}",start,middle,end);
     let mut project:String = args.project_arg;
     run_cmd!(wget $finals -O $project.txt);
     /* implementing a download option also lateron so that you dont have to visit ena repository
     run_cmd!(bash -c "for i in $(cat file.txt | grep -w gz ); do echo $i; done | grep ^ftp");
     run_cmd!{bash -c "awk '{print $8}' file.txt > enaccess.sh"};
     run_cmd!(cat enaccess.sh);
     run_cmd!(rm -rf file.txt);
     */
}
