import { SimplecollectionAccount, getDecrementInstruction } from '@project/anchor'
import { useMutation } from '@tanstack/react-query'
import { UiWalletAccount, useWalletUiSigner } from '@wallet-ui/react'
import { useWalletUiSignAndSend } from '@wallet-ui/react-gill'
import { toastTx } from '@/components/toast-tx'
import { useSimplecollectionAccountsInvalidate } from './use-simplecollection-accounts-invalidate'

export function useSimplecollectionDecrementMutation({
  account,
  simplecollection,
}: {
  account: UiWalletAccount
  simplecollection: SimplecollectionAccount
}) {
  const invalidateAccounts = useSimplecollectionAccountsInvalidate()
  const signer = useWalletUiSigner({ account })
  const signAndSend = useWalletUiSignAndSend()

  return useMutation({
    mutationFn: async () => await signAndSend(getDecrementInstruction({ simplecollection: simplecollection.address }), signer),
    onSuccess: async (tx) => {
      toastTx(tx)
      await invalidateAccounts()
    },
  })
}
