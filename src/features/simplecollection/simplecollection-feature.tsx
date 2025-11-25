import { useSolana } from '@/components/solana/use-solana'
import { WalletDropdown } from '@/components/wallet-dropdown'
import { AppHero } from '@/components/app-hero'
import { SimplecollectionUiButtonInitialize } from './ui/simplecollection-ui-button-initialize'
import { SimplecollectionUiList } from './ui/simplecollection-ui-list'
import { SimplecollectionUiProgramExplorerLink } from './ui/simplecollection-ui-program-explorer-link'
import { SimplecollectionUiProgramGuard } from './ui/simplecollection-ui-program-guard'

export default function SimplecollectionFeature() {
  const { account } = useSolana()

  return (
    <SimplecollectionUiProgramGuard>
      <AppHero
        title="Simplecollection"
        subtitle={
          account
            ? "Initialize a new simplecollection onchain by clicking the button. Use the program's methods (increment, decrement, set, and close) to change the state of the account."
            : 'Select a wallet to run the program.'
        }
      >
        <p className="mb-6">
          <SimplecollectionUiProgramExplorerLink />
        </p>
        {account ? (
          <SimplecollectionUiButtonInitialize account={account} />
        ) : (
          <div style={{ display: 'inline-block' }}>
            <WalletDropdown />
          </div>
        )}
      </AppHero>
      {account ? <SimplecollectionUiList account={account} /> : null}
    </SimplecollectionUiProgramGuard>
  )
}
