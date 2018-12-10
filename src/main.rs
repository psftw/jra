#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_derive;

extern crate dirs;
extern crate serde;
extern crate serde_json;
extern crate goji;

use std::fs::File;
use std::error::Error;
use std::collections::BTreeMap;

use goji::{Credentials, Jira};
use prettytable::{format, Table};
use structopt::StructOpt;

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(flatten)]
    configs: BTreeMap<String, JiraConfig>,
}

impl Config {
    fn lookup_jira(&self, query: &String) -> Result<JiraConfig, Box<dyn Error>> {
        let mut result = Err(format!("unknown query name: {}", query).into());
        for config in self.configs.clone().values() {
            for q in config.queries.keys() {
                if q == query {
                    result = match result {
                        Err(_) => { Ok(config.clone()) }
                        Ok(_) => { Err(format!("ambiguous jira for query: {}", &query).into()) }
                    }
                }
            }
        }
        result
    }
}

#[derive(Clone, Debug, Deserialize)]
struct JiraConfig {
    host: String,
    user: String,
    pass: String,
    queries: BTreeMap<String, String>,
}

fn linkify(text: String, url: String) -> String {
    format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, text)
}

/// query jira api from cli
#[derive(StructOpt, Debug)]
#[structopt(name = "jeera", author = "")]
enum Opt {
    /// list queries
    #[structopt(name = "ls", author = "")]
    List {},

    /// run query
    #[structopt(name = "q", author = "")]
    Query {
        /// query name
        query: String,
    },
}

fn run() -> Result<(), Box<Error>> {

    let mut config_path = dirs::config_dir().ok_or("unable to find a config dir")?;
    config_path.push("jeera.json");
    let config_file = File::open(config_path)?;
    let config: Config = serde_json::from_reader(config_file)?;
    let opt = Opt::from_args();

    match opt {
        Opt::List {} => {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_CLEAN);
            table.add_row(row!["JIRA_NAME", "QUERY_NAME", "QUERY"]);
            for (jira_name, config) in config.configs {
                for (q_name, q) in config.queries {
                    table.add_row(row![jira_name, q_name, q]);
                }
            }
            table.printstd();
        }
        Opt::Query { query } => {
            let jc = config.lookup_jira(&query)?;
            let jira = Jira::new(jc.host.clone(), Credentials::Basic(jc.user.clone(), jc.pass.clone())).unwrap();
            let query_text = jc.queries.get(&query).unwrap().to_string();
            match jira.search().iter(query_text, &Default::default()) {
                Ok(results) => {
                    let mut table = Table::new();
                    table.set_format(*format::consts::FORMAT_CLEAN);
                    // put LINK at the end otherwise prettytable gets confused about the cell
                    // length due to the escape codes
                    table.add_row(row!["SUMMARY", "STATUS", "REPORTER", "ASSIGNEE", "LINK"]);
                    for issue in results {
                        table.add_row(row![
                            issue.summary().unwrap_or("???".to_owned()),
                            issue.status().map(|value| value.name).unwrap_or("".to_owned()),
                            issue.reporter().map(|value| value.display_name).unwrap_or("".to_owned()),
                            issue.assignee().map(|value| value.display_name,).unwrap_or("".to_owned()),
                            linkify(issue.key.clone(), format!("{}/browse/{}", &jc.host, &issue.key)),
                        ]);
                    }
                    table.printstd();
                }
                Err(err) => panic!("{:#?}", err),
            }
        }
    }
    Ok(())

}

fn main() {
    if let Err(err) = run() {
        println!("error: {}", err);
    }
}
