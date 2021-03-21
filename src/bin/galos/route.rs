use async_std::task;
use structopt::StructOpt;
use itertools::Itertools;
use galos_db::{Database, systems::{System, Jump}};
use galos::Run;

#[derive(StructOpt, Debug)]
pub struct Cli {
    // #[structopt(parse(lalrpop(Route)))]
    range: f64,
    start: String,
    end: String,

    #[structopt(long = "insert")]
    insert: bool,
}

impl Run for Cli {
    fn run(&self, db: &Database) {
        let (start, end) = task::block_on(async {
            let start = System::fetch_by_name(db, &self.start).await.unwrap();
            let end   = System::fetch_by_name(db, &self.end).await.unwrap();
            (start, end)
        });

        // TODO: add progress bar.
        let (route, cost) = start.route_to(db, &end, self.range).unwrap().unwrap();

        let mut prev: Option<Jump> = None;
        let mut start = true;
        for (a, b) in route[..].into_iter().tuple_windows() {
            // TODO INSERT routes
            // println!("Route starting from {:#?}", &pair[0]);

            if start {
                println!("{:?}", &a.name);
                start = false;
            }
            println!("-> {}\n{:?}", a.distance(&b), &b.name);

            if self.insert {
                let current = if let Some(ref p) = prev {
                    task::block_on(async {
                        Jump::create(db, &b, Some(p), None).await.expect("start jump sql")
                    })
                } else {
                    task::block_on(async {
                        Jump::create(db, &a, None, None).await.expect("start jump sql")
                    })
                };
                prev = Some(current);
            }
        }
        println!("-----");
        println!("total jumps: {}, cost: {}", route.len() - 1, cost);
    }
}

// enum Route {
//     End,
//     Stop(String),
//     // `A -> B` specifies a direct path from A to B
//     Path(Box<Route>, Box<Route>),
//     // `A + B` specifies a path to both A and B, where the route could either visit
//     // A or B first
//     Both(Box<Route>, Box<Route>),
//     // `A | B` specifies a path to either A or B
//     Either(Box<Route>, Box<Route>),
// }
