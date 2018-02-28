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
            let acronyms = get_acronyms(&db);
            let regset = get_regexes(&db).expect("Failed to get regexes");
            let comments = fetch_comments(&reddit);
            for comment in comments {
                let mut matches = 0;
                for acronym_id in regset.matches(&comment.body) {
                    matches += 1;
                    let acronym = &acronyms[acronym_id];
                    println!("Match: {}: {}", acronym.key, acronym.value);
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
    }
}
