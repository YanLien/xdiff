use anyhow::Result;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input};
use std::fmt::Write as _;
use std::io::Write as _;
use xdiff_live::{
    cli::{Action, Args, RunArgs},
    get_body_text, get_headers_text, get_status_text, highlight_text, LoadConfig, RequestConfig,
    RequestProfile, process_error_output,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    // println!("{:?}", args);
    let result = match args.action {
        Action::Run(args) => run(args).await,
        Action::Parse => parse().await,
        _ => panic!("Not implemented"),
    };
    
    process_error_output(result)
}

async fn run(args: RunArgs) -> Result<()> {
    let config_file = args.config.unwrap_or_else(|| "./xreq_test.yml".to_string());
    // println!("Using config file: {}", config_file);
    let config = RequestConfig::load_yaml(&config_file).await?;
    // println!("{:?}", config);
    let profile = config.get_profile(&args.profile).ok_or_else(|| {
        anyhow::anyhow!(
            "Profile {} not found in config file {}",
            args.profile,
            config_file
        )
    })?;

    let extra_args = args.extra_params.into();

    let url = profile.get_url(&extra_args)?;

    let res = profile.send(&extra_args).await?.into_inner();

    let status = get_status_text(&res)?;
    let headers = get_headers_text(&res, &[])?;
    let body = get_body_text(res, &[]).await?;

    let mut output = String::new();

    if atty::is(atty::Stream::Stdout) {
        write!(&mut output, "Url: {}\n", url)?;
        write!(&mut output, "{}", status)?;
        write!(
            &mut output,
            "{}",
            highlight_text(&headers, "yaml", Some("Solarized (dark)"))?
        )?;

        write!(&mut output, "{}", highlight_text(&body, "json", None)?)?;
    } else {
        write!(&mut output, "{}", body)?;
    }

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    writeln!(stdout, "{}", output)?;

    Ok(())
}

async fn parse() -> Result<()> {
    // println!("Parse not implemented");
    let theme = ColorfulTheme::default();
    let url = Input::<String>::with_theme(&theme)
        .with_prompt("Url1")
        .interact_text()?;

    let profile: RequestProfile = url.parse()?;

    let name = Input::<String>::with_theme(&theme)
        .with_prompt("Profile")
        .interact_text()?;

    let config = RequestConfig::new(vec![(name, profile)].into_iter().collect());
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


