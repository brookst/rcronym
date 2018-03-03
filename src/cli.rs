#[derive(StructOpt)]
#[structopt(name = "rcronym")]
pub enum Cli {
    #[structopt(name = "add")]
    /// Add a new acronym to the database
    AddAcronym {
        #[structopt(long = "key")]
        /// Acronym/initialism
        key: String,
        #[structopt(long = "regex")]
        /// Regular expression to match. '\bKEY\b' by default
        regex: Option<String>,
    },
    #[structopt(name = "edit")]
    /// Edit an acronym in-place
    EditAcronym {
        #[structopt(long = "id")]
        /// ID number in database (See rcronym list)
        id: i32,
        /// Acronym/initialism
        #[structopt(long = "key")]
        key: Option<String>,
        #[structopt(long = "regex")]
        /// Regular expression to match.
        regex: Option<String>,
        #[structopt(long = "value")]
        /// Description of acronym
        value: Option<String>,
    },
    #[structopt(name = "rm")]
    /// Remove an acronym from the database
    RemoveAcronym {
        #[structopt(long = "id")]
        /// ID number in database (See rcronym list)
        id: i32,
    },
    #[structopt(name = "list")]
    /// Print acronyms stored in database
    ListAcronyms,
    #[structopt(name = "candidates")]
    /// Look for WORDs in recent Reddit comments
    RecentCandidates,
    #[structopt(name = "recent")]
    /// Look for acronyms in recent Reddit comments
    ParseRecent,
    #[structopt(name = "expand")]
    /// Print all acronyms detected in a thread
    ExpandThread {
        #[structopt(long = "id")]
        /// Reddit thread ID
        thread_id: String,
    },
}
