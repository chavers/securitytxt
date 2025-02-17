use anchor_lang::prelude::*;
#[cfg(not(feature = "no-entrypoint"))]
use  solana_security_txt::security_txt;

declare_id!("798shHqbnsSvY6odSdQJXaV2UEf1pmcBE7jsWyBV7Pc4");

#[program]
pub mod securitytxt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    #[cfg(not(feature = "no-entrypoint"))]
    security_txt! {
        name: "verified-builds",
        project_url: "https://myproject.com",
        contacts: "email:security@myproject.com",
        policy: "https://myproject.com/security-policy",

        // Optional Fields
        preferred_languages: "en,de",
        source_code: "https://github.com/chavers/securitytxt",
        source_revision: "",
        source_release: "",
        encryption: "",
        auditors: " ",
        acknowledgements: "Thank you to our bug bounty hunters!"
    }
}

#[derive(Accounts)]
pub struct Initialize {}
