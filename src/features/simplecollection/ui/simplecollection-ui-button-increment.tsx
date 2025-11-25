import { SimplecollectionAccount } from '@project/anchor'
import { UiWalletAccount } from '@wallet-ui/react'
import { Button } from '@/components/ui/button'
import { useSimplecollectionIncrementMutation } from '../data-access/use-simplecollection-increment-mutation'

export function SimplecollectionUiButtonIncrement({ account, simplecollection }: { account: UiWalletAccount; simplecollection: SimplecollectionAccount }) {
  const incrementMutation = useSimplecollectionIncrementMutation({ account, simplecollection })

  return (
    <Button variant="outline" onClick={() => incrementMutation.mutateAsync()} disabled={incrementMutation.isPending}>
      Increment
    </Button>
  )
}
