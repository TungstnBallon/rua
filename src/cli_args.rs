use clap::*;

pub fn build_cli() -> App<'static, 'static> {
	App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.setting(AppSettings::SubcommandRequired)
		.subcommand(SubCommand::with_name("install")
			.about("Download a package by name and build it.")
			.arg(Arg::with_name("offline").long("offline").short("o")
				.help("forbid internet access while building packages (sources are downloaded using .SRCINFO)"))
			.arg(Arg::with_name("TARGET").help("target package").required(true).index(1)))
		.subcommand(SubCommand::with_name("jailbuild")
			.about("Build package, using PKGBUILD and wrapping in jail")
			.arg(Arg::with_name("offline").long("offline").short("o")
				.help("forbid internet access while building (sources are downloaded using .SRCINFO)"))
			.arg(Arg::with_name("TARGET").help("directory to build")
				.required(false).default_value(".").index(1)))
		.subcommand(SubCommand::with_name("tarcheck")
			.about("Check *.tar or *.tar.xz archive")
			.arg(Arg::with_name("TARGET").help("archive to check").required(true).index(1)))
		.subcommand(SubCommand::with_name("search")
			.about("Opens AUR web search page")
			.arg(Arg::with_name("TARGET").help("target to search for").required(true).index(1)))
}
