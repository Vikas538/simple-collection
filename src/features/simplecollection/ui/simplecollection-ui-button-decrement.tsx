import { SimplecollectionAccount } from '@project/anchor'
import { UiWalletAccount } from '@wallet-ui/react'
import { Button } from '@/components/ui/button'

import { useSimplecollectionDecrementMutation } from '../data-access/use-simplecollection-decrement-mutation'

export function SimplecollectionUiButtonDecrement({ account, simplecollection }: { account: UiWalletAccount; simplecollection: SimplecollectionAccount }) {
  const decrementMutation = useSimplecollectionDecrementMutation({ account, simplecollection })

  return (
    <Button variant="outline" onClick={() => decrementMutation.mutateAsync()} disabled={decrementMutation.isPending}>
      Decrement
    </Button>
  )
}
