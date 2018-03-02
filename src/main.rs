//! A reddit bot shamelessly reimplementing [Decronym](http://decronym.xyz/) but in
//! [Rust](https://www.rust-lang.org/) and for the
//! [/r/rust subreddit](https://www.reddit.com/r/rust/).
//!
//! # Installation
//!
//!     cargo install diesel
//!     git clone https://github.com/brookst/rcronym.git
//!     cd rcronym
//!     echo "DATABASE_URL=postgres://rcronym@localhost/rcronym" > .env
//!     diesel setup
//!
//! # Usage
//!
//! The command line interface can be used to add and remove acronyms with the `add` and `rm`
//! commands. The `add` command requires a `--key` argument for the acronym, an optional `--regex`
//! argument to customise the matching, and consumes the explanation on `stdin`.
//!
//! Acronyms in the database can be listed with the `list` command. The id number of each acronym
//! can be used to remove it with `rm --id`.
//!
//! The most recent comments in the `/r/rust` subreddit are fetched, and any matches with acronyms
//! in the database are printed.
//!
//! A blanket upper-case regex can be used to look for acronyms in use with the `candidates`
//! command.

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate orca;
extern crate regex;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use orca::App;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io::{stdin, Read};

mod schema;
mod models;
mod cli;

use self::cli::Cli;

fn connect_reddit() -> App {
    let name = "Rcronym";
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_VERSION");
    App::new(name, version, author).unwrap()
}

fn fetch_comments<'a>(reddit: &'a App) -> orca::data::Comments<'a> {
    reddit.create_comment_stream("rust")
}

fn get_acronyms(connection: &PgConnection) -> Vec<models::Acronym> {
    use schema::acronyms;
    use models::Acronym;
    acronyms::dsl::acronyms
        .load::<Acronym>(connection)
        .expect("Error loading acronyms")
}

fn get_regexes(connection: &PgConnection) -> Result<regex::RegexSet, regex::Error> {
    use self::schema::*;
    let results = acronyms::dsl::acronyms
        .select(acronyms::regex)
        .load::<String>(connection)
        .expect("Error loading acronyms");
    let builder = regex::RegexSetBuilder::new(results);
    builder.build()
}

fn connect_db() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use structopt::StructOpt;
    let matches = Cli::from_args();

    let reddit = connect_reddit();
    let db = connect_db();

    match matches {
        Cli::ListAcronyms => {
            let acronyms = get_acronyms(&db);

            for acronym in acronyms {
                println!(
                    "[{}] {}: {} {}",
                    acronym.id, acronym.key, acronym.value, acronym.regex
                );
            }
        }
        Cli::AddAcronym { key, regex } => {
            use schema::acronyms;
            use models::Acronym;
            let mut input = String::new();
            stdin().read_to_string(&mut input).unwrap();
            let regex = regex.unwrap_or(format!(r"\b{}\b", key));
            diesel::insert_into(acronyms::table)
                .values((
                    acronyms::key.eq(key),
                    acronyms::regex.eq(regex),
                    acronyms::value.eq(input.trim_right()),
                ))
                .get_result::<Acronym>(&db)
                .expect("Error inserting acronym");
        }
        Cli::RemoveAcronym { id } => {
            use schema::acronyms;
            diesel::delete(acronyms::dsl::acronyms.filter(acronyms::id.eq(id)))
                .execute(&db)
                .expect("Error deleting acronym");
        }
        Cli::RecentCandidates => {
            let comments = fetch_comments(&reddit);
            let regex = regex::Regex::new(r"\b[A-Z]{3,6}\b").unwrap();
            for comment in comments {
                for candidate in regex.find_iter(&comment.body) {
                    println!(
                        "{} https://www.reddit.com/r/rust/comments/{}//{}/",
                        candidate.as_str(),
                        comment.link_id.trim_left_matches("t3_"),
                        comment.id
                    );
                }
            }
        }
        Cli::ParseRecent => {
            use schema::occurrences;
            let acronyms = get_acronyms(&db);
            let regset = get_regexes(&db).expect("Failed to get regexes");
            let comments = fetch_comments(&reddit);
            for comment in comments {
                let mut matches = 0;
                let thread_id = comment.link_id.trim_left_matches("t3_");
                for acronym_id in regset.matches(&comment.body) {
                    matches += 1;
                    let acronym = &acronyms[acronym_id];
                    println!("Match: {}: {}", acronym.key, acronym.value);
                    diesel::insert_into(occurrences::table)
                        .values((
                            occurrences::thread_id.eq(thread_id),
                            occurrences::comment_id.eq(&comment.id),
                            occurrences::acronym_id.eq(acronym.id),
                        ))
                        .on_conflict_do_nothing()
                        .execute(&db)
                        .expect("Error inserting occurrence to db");
                }
                if matches > 0 {
                    println!(
                        "{} matches in https://www.reddit.com/r/rust/comments/{}//{}/ by {}",
                        matches,
                        comment.link_id.trim_left_matches("t3_"),
                        comment.id,
                        comment.author
                    );
                }
            }
        }
        Cli::ExpandThread { thread_id } => {
            use schema::{occurrences, acronyms};
            use models::Acronym;
            println!("Thread: https://www.reddit.com/r/rust/comments/{}", thread_id);

            let results = occurrences::table
                .inner_join(acronyms::table)
                .filter(occurrences::thread_id.eq(thread_id))
                .select(acronyms::all_columns)
                .load::<Acronym>(&db)
                .expect("Thread query failed");
            for acronym in results {
                println!("  {}: {}", acronym.key, acronym.value);
            }
        }
    }
}
