# enafetch

<img src="https://github.com/IBCHgenomic/enafetch/blob/main/enafetch.png" width="100" />


- access ena and download files asynchronously.
- made for high performance computing where you can put the project numbers and links will be ready.
- just specify the project number and your files will be ready.

```
cargo install --path . 
or 
cargo build 

╭─gauavsablok@gauravsablok ~/Desktop/rust/ena-rust on main ✘
╰$ ./target/release/rust-ena PRJEB55343
PRJEB55343           100% [====================>]     688     --.-KB/s
                          [Files: 1  Bytes: 688 ]
wget ftp.sra.ebi.ac.uk/vol1/fastq/ERR100/071/ERR10084071/ERR10084071_1.fastq.gz
wget ftp.sra.ebi.ac.uk/vol1/fastq/ERR100/071/ERR10084071/ERR10084071_2.fastq.gz
wget ftp.sra.ebi.ac.uk/vol1/fastq/ERR100/064/ERR10077564/ERR10077564.fastq.gz

```

Gaurav Sablok \
Instytut Chemii Bioorganicznej Polskiej \
Akademii Nauk ul. Noskowskiego 12/14 | \
61-704, Poznań Poland
