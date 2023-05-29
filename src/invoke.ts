import { invoke } from '@tauri-apps/api/tauri'

interface Credentials { username: string | null, password: string | null }

interface School {
  id: number
  name: string
  dbn: string
}

interface Pathway {
  id: number
  name: string
}

export interface Student {
  id: number
  first_name: string
  last_name: string
  osis: number
  pathway_id: number
}

const _invoke = {
  schools: {
    list: async (creds: Credentials): Promise<School[]> => await invoke('get_schools', { ...creds })
  },
  pathways: {
    list: async (creds: Credentials, schoolId: number | string): Promise<Pathway[]> => await invoke('get_pathways', { ...creds, schoolId })
  },
  students: {
    list: async (creds: Credentials, schoolId: number | null, pathwayId: string | null, gradeId: number | null): Promise<Student[]> => await invoke('get_students', { ...creds, schoolId, pathwayId, gradeId })
  }
}

export default _invoke
