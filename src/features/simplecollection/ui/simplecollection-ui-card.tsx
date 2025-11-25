import { SimplecollectionAccount } from '@project/anchor'
import { ellipsify, UiWalletAccount } from '@wallet-ui/react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { AppExplorerLink } from '@/components/app-explorer-link'
import { SimplecollectionUiButtonClose } from './simplecollection-ui-button-close'
import { SimplecollectionUiButtonDecrement } from './simplecollection-ui-button-decrement'
import { SimplecollectionUiButtonIncrement } from './simplecollection-ui-button-increment'
import { SimplecollectionUiButtonSet } from './simplecollection-ui-button-set'

export function SimplecollectionUiCard({ account, simplecollection }: { account: UiWalletAccount; simplecollection: SimplecollectionAccount }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Simplecollection: {simplecollection.data.count}</CardTitle>
        <CardDescription>
          Account: <AppExplorerLink address={simplecollection.address} label={ellipsify(simplecollection.address)} />
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className="flex gap-4 justify-evenly">
          <SimplecollectionUiButtonIncrement account={account} simplecollection={simplecollection} />
          <SimplecollectionUiButtonSet account={account} simplecollection={simplecollection} />
          <SimplecollectionUiButtonDecrement account={account} simplecollection={simplecollection} />
          <SimplecollectionUiButtonClose account={account} simplecollection={simplecollection} />
        </div>
      </CardContent>
    </Card>
  )
}
