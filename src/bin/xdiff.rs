use anyhow::Result;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use std::io::Write;
use xdiff_live::{
    cli::{Action, Args, RunArgs},
    config::LoadConfig,
    highlight_text, process_error_output, DiffConfig, DiffProfile, ExtraArgs, RequestProfile,
    ResponseProfile,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    // println!("{:?}", args);
    let result = match args.action {
        Action::Run(arg) => run(arg).await,
        Action::Parse => parse().await,
        _ => panic!("Not implemented"),
    };

    process_error_output(result)
}

async fn run(args: RunArgs) -> Result<()> {
    let config_file = args
        .config
        .unwrap_or_else(|| "fixtures/test.yml".to_string());
    // println!("Using config file: {}", config_file);
    let config = DiffConfig::load_yaml(&config_file).await?;
    // println!("{:?}", config);
    let profile = config.get_profile(&args.profile).ok_or_else(|| {
        anyhow::anyhow!(
            "Profile {} not found in config file {}",
            args.profile,
            config_file
        )
    })?;

    let extra_args = args.extra_params.into();
    // into()是Rust中的一个通用方法，它用于执行转换（conversion）或转移（move）操作。
    let output = profile.diff(extra_args).await?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    write!(stdout, "{}", output)?;

    Ok(())
}

async fn parse() -> Result<()> {
    // println!("Parse not implemented");
    let theme = ColorfulTheme::default();
    let url1 = Input::<String>::with_theme(&theme)
        .with_prompt("Url1")
        .interact_text()?;
    let url2 = Input::<String>::with_theme(&theme)
        .with_prompt("Url2")
        .interact_text()?;

    let req1: RequestProfile = url1.parse()?;
    let req2: RequestProfile = url2.parse()?;

    let name = Input::<String>::with_theme(&theme)
        .with_prompt("Profile")
        .interact_text()?;

    let res = req1.send(&ExtraArgs::default()).await?;

    let headers = res.get_header_keys();

    let chosen = MultiSelect::with_theme(&theme)
        .with_prompt("Select headers to skip")
        .items(&headers)
        .interact()?;

    let skip_headers = chosen.iter().map(|i| headers[*i].to_string()).collect();

    let res = ResponseProfile::new(skip_headers, vec![]);
    let profile = DiffProfile::new(req1, req2, res);
    let config = DiffConfig::new(vec![(name, profile)].into_iter().collect());
    let result = serde_yaml::to_string(&config)?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    if atty::is(atty::Stream::Stdout) {
        write!(stdout, "---\n{}", highlight_text(&result, "yaml", None)?)?;
    } else {
        write!(stdout, "---\n{}", result)?;
    }
    Ok(())
}
