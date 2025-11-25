import { SIMPLECOLLECTION_PROGRAM_ADDRESS } from '@project/anchor'
import { AppExplorerLink } from '@/components/app-explorer-link'
import { ellipsify } from '@wallet-ui/react'

export function SimplecollectionUiProgramExplorerLink() {
  return <AppExplorerLink address={SIMPLECOLLECTION_PROGRAM_ADDRESS} label={ellipsify(SIMPLECOLLECTION_PROGRAM_ADDRESS)} />
}
