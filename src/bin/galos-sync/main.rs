use structopt::StructOpt;
use galos_db::{Error, Database};
use galos::Run;

mod journal;
mod eddn;
mod edsm;
mod eddb;

#[derive(StructOpt, Debug)]
pub enum Cli {
    #[structopt(about = "Import local journal files")]
    Journal(journal::Cli),
    #[structopt(about = "Subscribes to EDDN to continuously sync from incoming events")]
    Eddn(eddn::Cli),
    #[structopt(about = "Sync from EDSM's nightly dumps")]
    Edsm(edsm::Cli),
    #[structopt(about = "Sync from EDDB's nightly dumps")]
    Eddb(eddb::Cli),
}

impl Run for Cli {
    fn run(&self, db: &Database) {
        match self {
            Cli::Journal(cli) => cli.run(db),
            Cli::Eddn(cli) => cli.run(db),
            Cli::Edsm(cli) => cli.run(db),
            Cli::Eddb(cli) => cli.run(db),
        }
    }
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::from_args();
    let db = Database::new().await?;
    cli.run(&db);

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::sync::Once;
    use std::process::Command;
    use async_std::task;
    use assert_cmd::prelude::*;
    use predicates::str;
    use galos_db::{Database, systems::System};

    const TEST_DB_URL: &'static str = "postgresql://localhost/elite_test";

    static INIT: Once = Once::new();
    fn init() {
        INIT.call_once(|| {
            Command::new("sqlx")
                .arg("database").arg("reset").args(vec!["--source", "galos_db/migrations"])
                .env("DATABASE_URL", TEST_DB_URL)
                .status()
                .expect("reset failed");
        });
    }

    fn database() -> Database {
        init();
        task::block_on(async {
            Database::from_url(TEST_DB_URL).await.expect("init db")
        })
    }

    #[test]
    fn help() {
        let mut cmd = Command::cargo_bin("galos-sync").unwrap();
        cmd.arg("--help");
        cmd.assert().success();
    }

    #[test]
    fn version() {
        let mut cmd = Command::cargo_bin("galos-sync").unwrap();
        cmd.arg("-V");
        let assert = cmd.assert();
        assert.stdout(str::starts_with("galos"))
              .stdout(str::contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn journal() {
        let db = database();
        let count = task::block_on(async { System::count(&db).await.expect("count row") });
        let mut cmd = Command::cargo_bin("galos-sync").unwrap();
        cmd.arg("journal")
            .arg("elite_journal/dumps/Journal.200730230952.01.log")
            .env("DATABASE_URL", TEST_DB_URL);
        cmd.assert().success();
        assert_eq!(count + 5, task::block_on(async { System::count(&db).await.expect("count row") }));
    }

    #[test]
    #[ignore]
    fn eddn() {
        let db = database();
        let count = task::block_on(async { System::count(&db).await.expect("count row") });
        let mut cmd = Command::cargo_bin("galos-sync").unwrap();
        // TODO: Limit run length?
        cmd.arg("eddn")
            .env("DATABASE_URL", TEST_DB_URL);
        cmd.assert().success();
        assert_eq!(count + 2, task::block_on(async { System::count(&db).await.expect("count row") }));
    }

    #[test]
    #[ignore]
    fn edsm() {
        let db = database();
        let count = task::block_on(async { System::count(&db).await.expect("count row") });
        let mut cmd = Command::cargo_bin("galos-sync").unwrap();
        cmd.arg("edsm")
           .arg("file")
           .arg("edsm/tests/systemsPopulated.json")
           .env("DATABASE_URL", TEST_DB_URL);
        cmd.assert().success();
        assert_eq!(count + 500, task::block_on(async { System::count(&db).await.expect("count row") }));
    }

    #[test]
    fn eddb() {
        let db = database();
        let count = task::block_on(async { System::count(&db).await.expect("count row") });
        let mut cmd = Command::cargo_bin("galos-sync").unwrap();
        cmd.arg("eddb")
           .arg("eddb/tests/systems_recently.csv")
           .env("DATABASE_URL", TEST_DB_URL);
        cmd.assert().success();
        assert_eq!(count + 500, task::block_on(async { System::count(&db).await.expect("count row") }));
    }
}
