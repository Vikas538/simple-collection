import { useQueryClient } from '@tanstack/react-query'
import { useSimplecollectionAccountsQueryKey } from './use-simplecollection-accounts-query-key'

export function useSimplecollectionAccountsInvalidate() {
  const queryClient = useQueryClient()
  const queryKey = useSimplecollectionAccountsQueryKey()

  return () => queryClient.invalidateQueries({ queryKey })
}
