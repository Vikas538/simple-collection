#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, mint_to,MintTo,TokenAccount, TokenInterface};
use anchor_spl::metadata::{Metadata};

declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[constant]
pub const NAME: &str = "Simple Collection";
#[constant]
pub const URI: &str = "https://raw.githubusercontent.com/Vikas538/token-command-line/main/metadata.json";
#[constant]
pub const SYMBOL: &str = "SCOL";


#[program]
pub mod simplecollection {



    use anchor_spl::metadata::{CreateMasterEditionV3, CreateMetadataAccountsV3, SignMetadata, create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::{CollectionDetails, Creator, DataV2}, sign_metadata};

    use super::*;

    pub fn create_simple_collection(ctx:Context<InitializeCollection>,end_date:u64,nft_config_uri:String,max_nfts:u64)->Result<()>{

        ctx.accounts.simple_collection.end_date=end_date;
        ctx.accounts.simple_collection.nft_config_uri=nft_config_uri;
        ctx.accounts.simple_collection.max_nfts=max_nfts;
        ctx.accounts.simple_collection.nfts_created=0;

        let signer_seeds:&[&[&[u8]]]  =&[&[
            b"collection_mint".as_ref(),
            &[ctx.bumps.collection_mint],
        ]];

        mint_to(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), MintTo { mint: ctx.accounts.collection_mint.to_account_info(), to: ctx.accounts.collection_token_account.to_account_info(), authority: ctx.accounts.collection_mint.to_account_info() }, signer_seeds), 1)?;

        create_metadata_accounts_v3(CpiContext::new_with_signer(ctx.accounts.token_metadata_program.to_account_info(), CreateMetadataAccountsV3{
            metadata:ctx.accounts.metadata.to_account_info(),
            mint:ctx.accounts.collection_mint.to_account_info(),
            payer:ctx.accounts.payer.to_account_info(),
            rent:ctx.accounts.rent.to_account_info(),
            mint_authority:ctx.accounts.collection_mint.to_account_info(),
            system_program:ctx.accounts.system_program.to_account_info(),
            update_authority:ctx.accounts.collection_mint.to_account_info()

        }, &signer_seeds), DataV2{
            name: NAME.to_string(),
            symbol: SYMBOL.to_string(),
            uri: URI.to_string(),
            seller_fee_basis_points: 0,
            creators: Some(vec![Creator {
                address: ctx.accounts.collection_mint.key(),
                verified: false,
                share: 100,
            }]),
            collection: None,
            uses: None,
        }, true, true, Some(CollectionDetails::V1 { size: 0 }))?;



        create_master_edition_v3(CpiContext::new_with_signer(ctx.accounts.token_metadata_program.to_account_info(), CreateMasterEditionV3{
            metadata:ctx.accounts.metadata.to_account_info(),
            edition:ctx.accounts.master_edition.to_account_info(),
            mint:ctx.accounts.collection_mint.to_account_info(),
            mint_authority:ctx.accounts.collection_mint.to_account_info(),
            payer:ctx.accounts.payer.to_account_info(),
            rent:ctx.accounts.rent.to_account_info(),
            system_program:ctx.accounts.system_program.to_account_info(),
            update_authority:ctx.accounts.collection_mint.to_account_info(),
            token_program:ctx.accounts.token_program.to_account_info(),
        }, signer_seeds), Some(0))?;


        sign_metadata(CpiContext::new_with_signer(ctx.accounts.token_metadata_program.to_account_info(), SignMetadata{
            creator:ctx.accounts.collection_mint.to_account_info(),
            metadata:ctx.accounts.metadata.to_account_info(),
        }, signer_seeds))?;

        Ok(())


    }

    

    
}


#[derive(Accounts)]
#[instruction(id:u64)]
pub struct InitializeCollection<'info>{

    #[account(mut)]
    pub payer:Signer<'info>,

    #[account(init,payer=payer,space=8+SimpleCollection::INIT_SPACE,seeds=[b"simple_collection".as_ref(),id.to_le_bytes().as_ref()],bump)]
    pub simple_collection:Account<'info,SimpleCollection>,

    #[account(init,payer=payer,mint::decimals=0,mint::authority=collection_mint,mint::freeze_authority=collection_mint,seeds=[b"collection_mint"],bump)]
    pub collection_mint:InterfaceAccount<'info,Mint>,

    #[account(init,payer=payer,token::mint=collection_mint,token::authority=collection_token_account,seeds=[b"token_collection_mint_account"],bump)]
    pub collection_token_account:InterfaceAccount<'info,TokenAccount>,


    #[account(mut,
        seeds=[b"metadata",token_metadata_program.key().as_ref(),collection_mint.key().as_ref(),b"edition"],
        bump,
        seeds::program=token_metadata_program.key()
    )]
    /// CHECK: this account is checked by the metadata smart contract
    pub metadata:UncheckedAccount<'info>,


    #[account(mut,
        seeds=[b"master",token_metadata_program.key().as_ref(),collection_mint.key().as_ref(),b"edition"],
        bump,
        seeds::program=token_metadata_program.key()
    )]
    /// CHECK: this account is checked by the metadata smart contract
    pub master_edition:UncheckedAccount<'info>,

    pub token_metadata_program:Program<'info,Metadata> ,
    pub token_program:Interface<'info,TokenInterface>,

    pub system_program:Program<'info,System>,
    pub rent:Sysvar<'info,Rent>
}


#[account]
#[derive(InitSpace)]
pub struct  SimpleCollection{

    id:u64,
    authority:Pubkey,
    max_nfts:u64,
    end_date:u64,
    #[max_len(100)]
    nft_config_uri:String,
    nfts_created:u64,

}
