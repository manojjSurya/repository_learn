use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_m1atches(matches)
}

pub struct ConfigurationParameters {
    pub I1_file:String,
    pub I2_file:String,
    pub I3_file:String,
    pub I4_file:String,
    pub I5_file:String,
    pub I6_file:String,
    pub I7_file:String,
    pub Output_file:String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());


        info!(logger,"The path of I1 file is:{}",self.I1_file());
        info!(logger,"the path of I2 file is:{}",self.I2_file());
        info!(logger,"The path of I3 file is:{}",self.I3_file());
        info!(logger,"The path of I4 file is:{}",self.I4_file());
        info!(logger,"The path of I5 file is:{}",self.I5_file());
        info!(logger,"The path of I6 file is:{}",self.I6_file());
        info!(logger,"The path of I7 file is:{}",self.I7_file());
        info!(logger,"output_file_path:{}",self.Output_file());

    }
}

impl ConfigurationParameters {
    fn new_from_m1atches(matches: clap::ArgMatches) -> ConfigurationParameters {
         let mut I1_file=matches.value_of("I1-file").unwrap().to_string();
        //let mut output_file_path=matches.value_of("output_file_path").unwrap().to_string();
        let mut I2_file=matches.value_of("I2-file").unwrap().to_string();
        let mut I3_file=matches.value_of("I3-file").unwrap().to_string();
        let mut I4_file=matches.value_of("I4-file").unwrap().to_string();
        let mut I5_file=matches.value_of("I5-file").unwrap().to_string();
        let mut I6_file=matches.value_of("I6-file").unwrap().to_string();
        let mut I7_file=matches.value_of("I7-file").unwrap().to_string();
        let mut Output_file=matches.value_of("Output-file").unwrap().to_string();
        
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            I1_file,
            I2_file,
            I3_file,
            I4_file,
            I5_file,
            I6_file,
            I7_file,
            log_file_path,
            log_level,
            diagnostics_file_path,
            is_perf_diagnostics_enabled,
            Output_file,

        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn I1_file(&self)-> &str{
        &self.I1_file
    }
    pub fn I2_file(&self) -> &str{
        &self.I2_file
    }
    pub fn I3_file(&self) -> &str{
        &self.I3_file
    }
    pub fn I4_file(&self) -> &str{
        &self.I4_file
    }
    pub fn I5_file(&self) -> &str{
        &self.I5_file
    }
    pub fn I6_file(&self) -> &str{
        &self.I6_file
    }
    pub fn I7_file(&self) -> &str{
        &self.I7_file
    }
    pub fn Output_file(&self) -> &str{
        &self.Output_file
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)    
    .about("generating command line arguements for arguements!!")
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("log-file")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("diagnostics-log-file")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("I1-file")
                .long("I1-file")
                .value_name("I1-file")
                .help("Helps to get contents of I1 file.")
                .required(true)
        )
        .arg(
            Arg::with_name("I2-file")
                .long("I2-file")
                .value_name("I2-file")
                .help("Help get the contents of the I2 file.")
                .required(true)
        )
        .arg(
            Arg::with_name("I3-file")
                .long("I3-file")
                .value_name("I3-file")
                .help("Help get the contents of the I3 file.")
                .required(true)
        )
        .arg(
            Arg::with_name("I4-file")
                .long("I4-file")
                .value_name("I4-file")
                .help("Help get the contents of the I4 file.")
                .required(true)
        )
        .arg(
            Arg::with_name("I5-file")
                .long("I5-file")
                .value_name("I5-file")
                .help("Help get the contents of the I5 file.")
                .required(true)
        )
        .arg(
            Arg::with_name("I6-file")
                .long("I6-file")
                .value_name("I6-file")
                .help("Help get the contents from I6 file.")
                .required(true)
        )
        .arg(
            Arg::with_name("I7-file")
                .long("I7-file")
                .value_name("I7-file")
                .help("Help get the contents of the master1 file.")
                .required(true)
        )
        
        .arg(
            Arg::with_name("Output-file")
                .long("output-file")
                .value_name("Output-file")
                .help("Helps get the output file path to write contents to text file")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .required(false)
        )
        .get_matches()
}
