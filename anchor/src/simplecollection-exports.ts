// Here we export some useful types and functions for interacting with the Anchor program.
import { Account, getBase58Decoder, SolanaClient } from 'gill'
import { getProgramAccountsDecoded } from './helpers/get-program-accounts-decoded'
import { Simplecollection, SIMPLECOLLECTION_DISCRIMINATOR, SIMPLECOLLECTION_PROGRAM_ADDRESS, getSimplecollectionDecoder } from './client/js'
import SimplecollectionIDL from '../target/idl/simplecollection.json'

export type SimplecollectionAccount = Account<Simplecollection, string>

// Re-export the generated IDL and type
export { SimplecollectionIDL }

export * from './client/js'

export function getSimplecollectionProgramAccounts(rpc: SolanaClient['rpc']) {
  return getProgramAccountsDecoded(rpc, {
    decoder: getSimplecollectionDecoder(),
    filter: getBase58Decoder().decode(SIMPLECOLLECTION_DISCRIMINATOR),
    programAddress: SIMPLECOLLECTION_PROGRAM_ADDRESS,
  })
}
