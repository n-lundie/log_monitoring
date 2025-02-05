# Log Monitoring Tool 
A Rust command line application for generating reports based on process logs.

## Getting Started 
The easiest way to build and test the application is with Cargo. This guide assumes you have Cargo and rustc 1.84 or higher installed. If not, follow [this guide](https://www.rust-lang.org/tools/install) and then come back here.
1. Clone the repository
```bash
git clone https://github.com/n-lundie/log_monitoring.git 
```
2. Navigate into the repository
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
cp target/release/log_monitoring log_monitoring
```
6. Run the application on the log file 
```bash
./log_monitoring ./logs.log
```
7. The report will be generated in the same directory that you run the command from. An example of this output can be in the `report.log` file in this repo