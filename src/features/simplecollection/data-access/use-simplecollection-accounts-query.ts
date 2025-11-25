import { useSolana } from '@/components/solana/use-solana'
import { useQuery } from '@tanstack/react-query'
import { getSimplecollectionProgramAccounts } from '@project/anchor'
import { useSimplecollectionAccountsQueryKey } from './use-simplecollection-accounts-query-key'

export function useSimplecollectionAccountsQuery() {
  const { client } = useSolana()

  return useQuery({
    queryKey: useSimplecollectionAccountsQueryKey(),
    queryFn: async () => await getSimplecollectionProgramAccounts(client.rpc),
  })
}
