import {
  Blockhash,
  createSolanaClient,
  createTransaction,
  generateKeyPairSigner,
  Instruction,
  isSolanaError,
  KeyPairSigner,
  signTransactionMessageWithSigners,
} from 'gill'
import {
  fetchSimplecollection,
  getCloseInstruction,
  getDecrementInstruction,
  getIncrementInstruction,
  getInitializeInstruction,
  getSetInstruction,
} from '../src'
// @ts-ignore error TS2307 suggest setting `moduleResolution` but this is already configured
import { loadKeypairSignerFromFile } from 'gill/node'

const { rpc, sendAndConfirmTransaction } = createSolanaClient({ urlOrMoniker: process.env.ANCHOR_PROVIDER_URL! })

describe('simplecollection', () => {
  let payer: KeyPairSigner
  let simplecollection: KeyPairSigner

  beforeAll(async () => {
    simplecollection = await generateKeyPairSigner()
    payer = await loadKeypairSignerFromFile(process.env.ANCHOR_WALLET!)
  })

  it('Initialize Simplecollection', async () => {
    // ARRANGE
    expect.assertions(1)
    const ix = getInitializeInstruction({ payer: payer, simplecollection: simplecollection })

    // ACT
    await sendAndConfirm({ ix, payer })

    // ASSER
    const currentSimplecollection = await fetchSimplecollection(rpc, simplecollection.address)
    expect(currentSimplecollection.data.count).toEqual(0)
  })

  it('Increment Simplecollection', async () => {
    // ARRANGE
    expect.assertions(1)
    const ix = getIncrementInstruction({
      simplecollection: simplecollection.address,
    })

    // ACT
    await sendAndConfirm({ ix, payer })

    // ASSERT
    const currentCount = await fetchSimplecollection(rpc, simplecollection.address)
    expect(currentCount.data.count).toEqual(1)
  })

  it('Increment Simplecollection Again', async () => {
    // ARRANGE
    expect.assertions(1)
    const ix = getIncrementInstruction({ simplecollection: simplecollection.address })

    // ACT
    await sendAndConfirm({ ix, payer })

    // ASSERT
    const currentCount = await fetchSimplecollection(rpc, simplecollection.address)
    expect(currentCount.data.count).toEqual(2)
  })

  it('Decrement Simplecollection', async () => {
    // ARRANGE
    expect.assertions(1)
    const ix = getDecrementInstruction({
      simplecollection: simplecollection.address,
    })

    // ACT
    await sendAndConfirm({ ix, payer })

    // ASSERT
    const currentCount = await fetchSimplecollection(rpc, simplecollection.address)
    expect(currentCount.data.count).toEqual(1)
  })

  it('Set simplecollection value', async () => {
    // ARRANGE
    expect.assertions(1)
    const ix = getSetInstruction({ simplecollection: simplecollection.address, value: 42 })

    // ACT
    await sendAndConfirm({ ix, payer })

    // ASSERT
    const currentCount = await fetchSimplecollection(rpc, simplecollection.address)
    expect(currentCount.data.count).toEqual(42)
  })

  it('Set close the simplecollection account', async () => {
    // ARRANGE
    expect.assertions(1)
    const ix = getCloseInstruction({
      payer: payer,
      simplecollection: simplecollection.address,
    })

    // ACT
    await sendAndConfirm({ ix, payer })

    // ASSERT
    try {
      await fetchSimplecollection(rpc, simplecollection.address)
    } catch (e) {
      if (!isSolanaError(e)) {
        throw new Error(`Unexpected error: ${e}`)
      }
      expect(e.message).toEqual(`Account not found at address: ${simplecollection.address}`)
    }
  })
})

// Helper function to keep the tests DRY
let latestBlockhash: Awaited<ReturnType<typeof getLatestBlockhash>> | undefined
async function getLatestBlockhash(): Promise<Readonly<{ blockhash: Blockhash; lastValidBlockHeight: bigint }>> {
  if (latestBlockhash) {
    return latestBlockhash
  }
  return await rpc
    .getLatestBlockhash()
    .send()
    .then(({ value }) => value)
}
async function sendAndConfirm({ ix, payer }: { ix: Instruction; payer: KeyPairSigner }) {
  const tx = createTransaction({
    feePayer: payer,
    instructions: [ix],
    version: 'legacy',
    latestBlockhash: await getLatestBlockhash(),
  })
  const signedTransaction = await signTransactionMessageWithSigners(tx)
  return await sendAndConfirmTransaction(signedTransaction)
}
