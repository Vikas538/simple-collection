import { SimplecollectionAccount } from '@project/anchor'
import { UiWalletAccount } from '@wallet-ui/react'
import { Button } from '@/components/ui/button'

import { useSimplecollectionCloseMutation } from '@/features/simplecollection/data-access/use-simplecollection-close-mutation'

export function SimplecollectionUiButtonClose({ account, simplecollection }: { account: UiWalletAccount; simplecollection: SimplecollectionAccount }) {
  const closeMutation = useSimplecollectionCloseMutation({ account, simplecollection })

  return (
    <Button
      variant="destructive"
      onClick={() => {
        if (!window.confirm('Are you sure you want to close this account?')) {
          return
        }
        return closeMutation.mutateAsync()
      }}
      disabled={closeMutation.isPending}
    >
      Close
    </Button>
  )
}
