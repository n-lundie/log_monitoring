# Log Monitoring Tool 
A Rust command line application for generating reports based on process logs.

## Getting Started 
The easiest way to build and test the application is with Cargo. This guide assumes you have Cargo and rustc 1.84 or higher installed. If not, follow [this guide](https://www.rust-lang.org/tools/install) and then come back here.
1. Clone the repository
```bash
git clone https://github.com/n-lundie/log_monitoring.git 
```
2. Navigate into the repository
```bash
cd log_monitoring 
```
3. Run unit tests 
```bash
cargo test 
```
4. Build the application
```bash
cargo build --release 
```
5. Copy the binary into the top level directory
```bash
cp target/release/log_monitoring ./log_monitoring
```
6. Run the application on the log file 
```bash
./log_monitoring ./logs.log
```
7. The report will be generated in the same directory that you run the command from. An example of this output can be seen in the [report.log](https://github.com/n-lundie/log_monitoring/blob/main/report.log) file in this repo. Each line in the report contains the **PID**, **alert level** and **duration** of the job that triggered the alert.

## Areas For Improvement
* Currently the application does not include jobs that started but did not finish in the report. A flag to enable a strict mode which would include such jobs should be added.
* Reports are always generated within the current working directory. The user should be able to specify an output path.
* Reports are always generated with the same name, causing existing reports to be overwritten. Report file names should automatically increment to prevent this.
* The order that alerts appear in the report is determined by the order in which alerts are generated. Options for sorting the report by level, PID or duration should be added.
* The summary printed to the standard output should include the number of warnings and errors detected.