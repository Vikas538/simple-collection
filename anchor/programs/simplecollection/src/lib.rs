#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{mint_to, MintTo, Mint, TokenAccount, TokenInterface};
use anchor_spl::metadata::{self, CreateMetadataAccountsV3, CreateMasterEditionV3, SetAndVerifySizedCollectionItem, create_metadata_accounts_v3, create_master_edition_v3, set_and_verify_sized_collection_item, Metadata};

declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[constant]
pub const NAME: &str = "Simple Collection";
#[constant]
pub const URI: &str = "https://raw.githubusercontent.com/Vikas538/token-command-line/main/metadata.json";
#[constant]
pub const SYMBOL: &str = "SCOL";

#[program]
pub mod simplecollection {
    use anchor_spl::metadata::mpl_token_metadata::types::{DataV2,CollectionDetails,Creator,Collection};


    use super::*;


    pub fn initialize_collection(
        ctx: Context<InitializeCollection>,
        id: u64,
        end_date: u64,
        nft_config_uri: String,
        max_nfts: u64,
    ) -> Result<()> {
        // initialize collection state
        let sc = &mut ctx.accounts.simple_collection;
        sc.id = id;
        sc.authority = ctx.accounts.payer.key();
        sc.end_date = end_date;
        sc.nft_config_uri = nft_config_uri;
        sc.max_nfts = max_nfts;
        sc.nfts_created = 0;

        // signer for collection_mint PDA (we are the PDA)
        let collection_bump = ctx.bumps.collection_mint;
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"collection_mint".as_ref(),
            &id.to_le_bytes(),
            &[collection_bump],
        ]];

        // 1) mint 1 token into collection_token_account (make it a 1/1 NFT)
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    to: ctx.accounts.collection_token_account.to_account_info(),
                    authority: ctx.accounts.collection_mint.to_account_info(),
                },
                signer_seeds,
            ),
            1u64,
        )?;

        // 2) create metadata for the collection mint (DataV2 + CollectionDetails::V1 size 0)
        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                signer_seeds,
            ),
            DataV2 {
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
            },
            true,  // is_mutable
            true,  // update_authority_is_signer (we sign with PDA)
            Some(CollectionDetails::V1 { size: 0 }),
        )?;

        // 3) create master edition for the collection mint (max supply = 0)
        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: ctx.accounts.master_edition.to_account_info(),
                    metadata: ctx.accounts.metadata.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                signer_seeds,
            ),
            Some(0),
        )?;

        // 4) sign metadata as a creator (so the collection mint is a verified creator)
        // optional but often used; fine to call (collection_mint signs)
        metadata::sign_metadata(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                metadata::SignMetadata {
                    creator: ctx.accounts.collection_mint.to_account_info(),
                    metadata: ctx.accounts.metadata.to_account_info(),
                },
                signer_seeds,
            )
        )?;

        Ok(())
    }

    pub fn create_nft_under_collection(ctx: Context<CreateNft>, id: u64) -> Result<()> {
        // short refs
        let sc = &mut ctx.accounts.simple_collection;

        // ensure we haven't exceeded max
        require!(sc.nfts_created < sc.max_nfts, ErrorCode::MaxNftsReached);

        // collection PDA signer seeds (include id)
        let collection_bump = ctx.bumps.collection_mint;
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"collection_mint".as_ref(),
            &id.to_le_bytes(),
            &[collection_bump],
        ]];

        // 1) mint 1 token of the new nft_mint to payer's ATA
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    to: ctx.accounts.nft_mint_token_account.to_account_info(),
                    authority: ctx.accounts.collection_mint.to_account_info(),
                },
                signer_seeds,
            ),
            1u64,
        )?;

        // 2) create metadata for the NEW NFT (collection reference left unverified)
        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.nft_mint_metadata.to_account_info(),
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                signer_seeds,
            ),
            DataV2 {
                // you can make per-NFT names using the counter if you want
                name: format!("{} #{}", NAME, sc.nfts_created),
                symbol: SYMBOL.to_string(),
                uri: URI.to_string(), // if you have unique URIs per NFT, pass them in instruction args
                seller_fee_basis_points: 0,
                creators: None,
                collection:Some(Collection{ key: ctx.accounts.collection_mint.key(), verified: false }),
                uses: None,
            },
            false, // is_mutable
            false, // update_authority_is_signer (we use PDA)
            None,
        )?;

        // 3) create master edition for the NEW NFT (supply = 0 -> 1/1)
        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: ctx.accounts.nft_mint_master_edition.to_account_info(),
                    metadata: ctx.accounts.nft_mint_metadata.to_account_info(),
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                signer_seeds,
            ),
            Some(0),
        )?;

        // 4) verify & add the NFT to the sized collection (this increments the collection on-chain)
        set_and_verify_sized_collection_item(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                SetAndVerifySizedCollectionItem {
                    metadata: ctx.accounts.nft_mint_metadata.to_account_info(),
                    collection_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    collection_mint: ctx.accounts.collection_mint.to_account_info(),
                    collection_metadata: ctx.accounts.collection_mint_metadata.to_account_info(),
                    collection_master_edition: ctx.accounts.collection_mint_master_edition.to_account_info(),
                },
                signer_seeds,
            ),
            None, // collection_authority_record (none for PDA)
        )?;

        // 5) increment counter only after successful verification
        sc.nfts_created = sc.nfts_created.checked_add(1).ok_or(ErrorCode::Overflow)?;

        Ok(())
    }
}

// ---------- ACCOUNTS ----------

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct InitializeCollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// collection state (per id)
    #[account(
        init,
        payer = payer,
        space = 8 + SimpleCollection::INIT_SPACE,
        seeds = [b"simple_collection".as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub simple_collection: Account<'info, SimpleCollection>,

    /// collection mint (unique per id)
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint,
        seeds = [b"collection_mint".as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    /// token account owned by the collection PDA which will hold the 1-of-1 collection NFT
    #[account(
        init,
        payer = payer,
        token::mint = collection_mint,
        token::authority = collection_mint,
        seeds = [b"collection_token_account".as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub collection_token_account: InterfaceAccount<'info, TokenAccount>,

    /// metadata PDA for collection mint
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: checked by metaplex
    pub metadata: UncheckedAccount<'info>,

    /// master edition PDA for collection mint
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: checked by metaplex
    pub master_edition: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// the collection state account (mutable because we'll increment nfts_created)
    #[account(
        mut,
        seeds = [b"simple_collection".as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub simple_collection: Account<'info, SimpleCollection>,

    /// collection mint PDA (must be mutable for some CPIs)
    #[account(mut, seeds = [b"collection_mint".as_ref(), id.to_le_bytes().as_ref()], bump)]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    /// NEW NFT mint (unique per collection id + counter)
    #[account(
        init,
        payer = payer,
        seeds = [
            b"nft_mint".as_ref(),
            id.to_le_bytes().as_ref(),
            simple_collection.nfts_created.to_le_bytes().as_ref()
        ],
        bump,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint
    )]
    pub nft_mint: InterfaceAccount<'info, Mint>,

    /// Associated token account for payer to receive the NFT
    #[account(
        init,
        payer = payer,
        associated_token::mint = nft_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub nft_mint_token_account: InterfaceAccount<'info, TokenAccount>,

    /// NFT metadata PDA (will be created by Metaplex)
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: Metaplex will validate
    pub nft_mint_metadata: UncheckedAccount<'info>,

    /// NFT master edition PDA
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: Metaplex will validate
    pub nft_mint_master_edition: UncheckedAccount<'info>,

    /// Collection metadata PDA (for verification)
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK:
    pub collection_mint_metadata: UncheckedAccount<'info>,

    /// Collection master edition PDA (for verification)
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK:
    pub collection_mint_master_edition: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// ---------- STATE ----------

#[account]
#[derive(InitSpace)]
pub struct SimpleCollection {
    pub id: u64,
    pub authority: Pubkey,
    pub max_nfts: u64,
    pub end_date: u64,
    #[max_len(200)]
    pub nft_config_uri: String,
    pub nfts_created: u64,
}

impl SimpleCollection {
    // estimate space: id(8) + authority(32) + max_nfts(8) + end_date(8) + nft_config_uri (4 + up to 200) + nfts_created(8)
    pub const INIT_SPACE: usize = 8 + 32 + 8 + 8 + (4 + 200) + 8;
}

// ---------- ERRORS ----------
#[error_code]
pub enum ErrorCode {
    #[msg("Max NFTs reached for this collection")]
    MaxNftsReached,
    #[msg("Overflow incrementing counter")]
    Overflow,
}
