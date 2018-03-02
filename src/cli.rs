#[derive(StructOpt)]
#[structopt(name = "rcronym")]
pub enum Cli {
    #[structopt(name = "add")]
    AddAcronym {
        #[structopt(long = "key")]
        key: String,
        #[structopt(long = "regex")]
        regex: Option<String>,
    },
    #[structopt(name = "edit")]
    EditAcronym {
        #[structopt(long = "id")]
        id: i32,
        #[structopt(long = "key")]
        key: Option<String>,
        #[structopt(long = "regex")]
        regex: Option<String>,
        #[structopt(long = "value")]
        value: Option<String>,
    },
    #[structopt(name = "rm")]
    RemoveAcronym {
        #[structopt(long = "id")]
        id: i32,
    },
    #[structopt(name = "list")]
    ListAcronyms,
    #[structopt(name = "candidates")]
    RecentCandidates,
    #[structopt(name = "recent")]
    ParseRecent,
    #[structopt(name = "expand")]
    ExpandThread {
        #[structopt(long = "id")]
        thread_id: String,
    },
}
