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

export interface AddCertification {
  certificationId: string | null
  authorityId: string | null
  status: string | null
  date: string | null
}

export interface AddSkill {
  gradeId: string | null
  date: string | null
  deadline: string | null
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
    list: async (schoolId: number | null, pathwayId: string | null, gradeId: number | null): Promise<Student[]> => await invoke('get_students', { schoolId, pathwayId, gradeId }),
    addCertifications: async (students: number[], payload: AddCertification): Promise<void> => { await invoke('bulk_add_certifications', { students, ...payload }) },
    addSkills: async (students: number[], skillsType: 'employability' | 'technical', payload: AddSkill): Promise<void> => {
      console.log({ students, skillsType, ...payload })
      await invoke('bulk_add_skills', { students, skillsType, ...payload })
    }
  },
  certifications: {
    list: async (studentId: number): Promise<Certification[]> => await invoke('get_certifications', { studentId }),
    authorities: async (studentId: number, certificationId: string): Promise<CertificationAuthority[]> => await invoke('get_certification_authorities', { studentId, certificationId })
  }
}

export default _invoke
