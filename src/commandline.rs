use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct CommandlineOptions {
    #[structopt(long, short)]
    pub layout: Option<String>,

    #[structopt(long, short = "n")]
    pub project_name: Option<String>,

    #[structopt(long, short)]
    pub path: Option<String>,

    #[structopt(long, short, use_delimiter = true)]
    pub keys: Option<Vec<String>>,

    #[structopt(long, short)]
    pub hide_key: Option<bool>
}
