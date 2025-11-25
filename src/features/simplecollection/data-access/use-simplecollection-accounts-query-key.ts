import { useSolana } from '@/components/solana/use-solana'

export function useSimplecollectionAccountsQueryKey() {
  const { cluster } = useSolana()

  return ['simplecollection', 'accounts', { cluster }]
}
