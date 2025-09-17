import { backend } from 'declarations/backend'
import type { Result } from 'declarations/backend/backend.did'

export interface CreateInstitutionData {
  name: string
  email: string
}

const createInstitution = async ({
  name,
  email,
}: CreateInstitutionData): Promise<Result> => {
  const result = await backend.create_institution(name, email)
  return result
}

export default createInstitution
