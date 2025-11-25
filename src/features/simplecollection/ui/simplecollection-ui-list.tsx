import { SimplecollectionUiCard } from './simplecollection-ui-card'
import { useSimplecollectionAccountsQuery } from '@/features/simplecollection/data-access/use-simplecollection-accounts-query'
import { UiWalletAccount } from '@wallet-ui/react'

export function SimplecollectionUiList({ account }: { account: UiWalletAccount }) {
  const simplecollectionAccountsQuery = useSimplecollectionAccountsQuery()

  if (simplecollectionAccountsQuery.isLoading) {
    return <span className="loading loading-spinner loading-lg"></span>
  }

  if (!simplecollectionAccountsQuery.data?.length) {
    return (
      <div className="text-center">
        <h2 className={'text-2xl'}>No accounts</h2>
        No accounts found. Initialize one to get started.
      </div>
    )
  }

  return (
    <div className="grid lg:grid-cols-2 gap-4">
      {simplecollectionAccountsQuery.data?.map((simplecollection) => (
        <SimplecollectionUiCard account={account} key={simplecollection.address} simplecollection={simplecollection} />
      ))}
    </div>
  )
}
