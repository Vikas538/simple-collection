import { Button } from '@/components/ui/button'
import { UiWalletAccount } from '@wallet-ui/react'

import { useSimplecollectionInitializeMutation } from '@/features/simplecollection/data-access/use-simplecollection-initialize-mutation'

export function SimplecollectionUiButtonInitialize({ account }: { account: UiWalletAccount }) {
  const mutationInitialize = useSimplecollectionInitializeMutation({ account })

  return (
    <Button onClick={() => mutationInitialize.mutateAsync()} disabled={mutationInitialize.isPending}>
      Initialize Simplecollection {mutationInitialize.isPending && '...'}
    </Button>
  )
}
