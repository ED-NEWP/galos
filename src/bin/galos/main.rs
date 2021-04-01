use structopt::StructOpt;
use galos_db::{Error, Database};
use galos::Run;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short = "d", long = "database", help = "override default (.env) database URL")]
    database_url: Option<String>,
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    #[structopt(about = "Search for systems, bodies, stations, factions, etc")]
    Search(search::Cli),
    #[structopt(about = "Plot routes between to and from many systems")]
    Route(route::Cli),
}

impl Run for Subcommand {
    fn run(&self, db: &Database) {
        match self {
            Subcommand::Search(cli) => cli.run(db),
            Subcommand::Route(cli)  => cli.run(db),
        }
    }
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::from_args();
    let db = if let Some(url) = cli.database_url {
        Database::from_url(&url).await?
    } else {
        Database::new().await?
    };

    cli.subcommand.run(&db);
    Ok(())
}

mod search;
mod route;


#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use predicates::str;
    use std::process::Command;

    #[test]
    fn help() {
        let mut cmd = Command::cargo_bin("galos").unwrap();
        cmd.arg("--help");
        cmd.assert().success();
    }

    #[test]
    fn version() {
        let mut cmd = Command::cargo_bin("galos").unwrap();
        cmd.arg("-V");
        let assert = cmd.assert();
        assert.stdout(str::starts_with("galos"))
              .stdout(str::contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn search() {
        let mut cmd = Command::cargo_bin("galos").unwrap();
        cmd.arg("search").args(vec!["-s", "Sol"]);
        cmd.assert().stdout(predicate::str::contains("population"));
    }

    #[test]
    fn search_count() {
        let mut cmd = Command::cargo_bin("galos").unwrap();
        cmd.arg("search").args(vec!["-s", "Sothis", "-c"]);
        cmd.assert().stdout(predicate::str::contains("1 system"));
    }
}
