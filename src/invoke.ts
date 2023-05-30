import { invoke } from '@tauri-apps/api/tauri'

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

export interface Certification {
  id: string
  name: string
}

type CertificationAuthority = Certification

const _invoke = {
  schools: {
    list: async (): Promise<School[]> => await invoke('get_schools')
  },
  pathways: {
    list: async (schoolId: number | string): Promise<Pathway[]> => await invoke('get_pathways', { schoolId })
  },
  students: {
    list: async (schoolId: number | null, pathwayId: string | null, gradeId: number | null): Promise<Student[]> => await invoke('get_students', { schoolId, pathwayId, gradeId })
  },
  certifications: {
    list: async (studentId: number): Promise<Certification[]> => await invoke('get_certifications', { studentId }),
    authorities: async (studentId: number, certificationId: string): Promise<CertificationAuthority[]> => await invoke('get_certification_authorities', { studentId, certificationId })
  }
}

export default _invoke
