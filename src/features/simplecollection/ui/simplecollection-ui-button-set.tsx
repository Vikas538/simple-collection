import { SimplecollectionAccount } from '@project/anchor'
import { UiWalletAccount } from '@wallet-ui/react'
import { Button } from '@/components/ui/button'

import { useSimplecollectionSetMutation } from '@/features/simplecollection/data-access/use-simplecollection-set-mutation'

export function SimplecollectionUiButtonSet({ account, simplecollection }: { account: UiWalletAccount; simplecollection: SimplecollectionAccount }) {
  const setMutation = useSimplecollectionSetMutation({ account, simplecollection })

  return (
    <Button
      variant="outline"
      onClick={() => {
        const value = window.prompt('Set value to:', simplecollection.data.count.toString() ?? '0')
        if (!value || parseInt(value) === simplecollection.data.count || isNaN(parseInt(value))) {
          return
        }
        return setMutation.mutateAsync(parseInt(value))
      }}
      disabled={setMutation.isPending}
    >
      Set
    </Button>
  )
}
