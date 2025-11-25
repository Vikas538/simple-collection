import { SimplecollectionAccount, getIncrementInstruction } from '@project/anchor'
import { UiWalletAccount, useWalletUiSigner } from '@wallet-ui/react'
import { useWalletUiSignAndSend } from '@wallet-ui/react-gill'
import { useMutation } from '@tanstack/react-query'
import { toastTx } from '@/components/toast-tx'
import { useSimplecollectionAccountsInvalidate } from './use-simplecollection-accounts-invalidate'

export function useSimplecollectionIncrementMutation({
  account,
  simplecollection,
}: {
  account: UiWalletAccount
  simplecollection: SimplecollectionAccount
}) {
  const invalidateAccounts = useSimplecollectionAccountsInvalidate()
  const signAndSend = useWalletUiSignAndSend()
  const signer = useWalletUiSigner({ account })

  return useMutation({
    mutationFn: async () => await signAndSend(getIncrementInstruction({ simplecollection: simplecollection.address }), signer),
    onSuccess: async (tx) => {
      toastTx(tx)
      await invalidateAccounts()
    },
  })
}
