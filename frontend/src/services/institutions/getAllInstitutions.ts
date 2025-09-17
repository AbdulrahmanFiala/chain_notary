import { backend } from 'declarations/backend'
import type { Institution } from 'declarations/backend/backend.did'

const getAllInstitutions = async (): Promise<Institution[]> => {
  const institutions = await backend.get_all_institutions()
  return institutions
}

export default getAllInstitutions
