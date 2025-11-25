import { SimplecollectionAccount, getCloseInstruction } from '@project/anchor'
import { useMutation } from '@tanstack/react-query'
import { UiWalletAccount, useWalletUiSigner } from '@wallet-ui/react'
import { useWalletUiSignAndSend } from '@wallet-ui/react-gill'
import { toastTx } from '@/components/toast-tx'
import { useSimplecollectionAccountsInvalidate } from './use-simplecollection-accounts-invalidate'

export function useSimplecollectionCloseMutation({ account, simplecollection }: { account: UiWalletAccount; simplecollection: SimplecollectionAccount }) {
  const invalidateAccounts = useSimplecollectionAccountsInvalidate()
  const signAndSend = useWalletUiSignAndSend()
  const signer = useWalletUiSigner({ account })

  return useMutation({
    mutationFn: async () => {
      return await signAndSend(getCloseInstruction({ payer: signer, simplecollection: simplecollection.address }), signer)
    },
    onSuccess: async (tx) => {
      toastTx(tx)
      await invalidateAccounts()
    },
  })
}
