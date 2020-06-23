mod commands;

use commands::Subcommand;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcmd: Option<Subcommand>,
}


fn main() {
	let cli = Cli::from_args();
	println!("{:#?}", cli);

	if let Some(ref subcmd) = cli.subcmd {
		subcmd.run();
	} else {
		println!("{:#?}", cli);
	}
}

/***

Cli {
    subcmd: Some(
        Contract(
            ContractCmd {
                cmd: Call {
                    from: "0000000000000000000000000000000000000001",
                    value: "0",
                    to: "0000000000000000000000000000000000000002",
                    gas: 100000,
                    gas_price: "0",
                    data: Some(
                        "000000",
                    ),
                    data_file: Some(
                        "abc.txt",
                    ),
                },
            },
        ),
    ),
}

 ***/




