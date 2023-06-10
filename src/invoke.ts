import { invoke } from '@tauri-apps/api/tauri'

interface School {
  id: number
  name: string
  dbn: string
}

export interface Student {
  id: number
  first_name: string
  last_name: string
  osis: number
  pathway_id: number
}

export interface SelectOption {
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

export interface CTEOptions {
  years: SelectOption[]
  courses: SelectOption[]
  terms: SelectOption[]
}

export interface AddCourse {
  yearId: string | null
  courseId: string | null
  status: string | null
  teacherId: string | null
  termId: string | null
}

export interface AddAssessment {
  student_id: string
  score: string
  status: string
  date: string
}

const _invoke = {
  schools: {
    list: async (): Promise<School[]> => await invoke('get_schools')
  },
  pathways: {
    list: async (schoolId: number | string): Promise<SelectOption[]> => await invoke('get_pathways', { schoolId })
  },
  students: {
    list: async (schoolId: number | null, pathwayId: string | null, gradeId: number | null): Promise<Student[]> => await invoke('get_students', { schoolId, pathwayId, gradeId }),
    addCertifications: async (students: number[], payload: AddCertification): Promise<void> => { await invoke('bulk_add_certifications', { students, ...payload }) },
    addSkills: async (students: number[], skillsType: 'employability' | 'technical', payload: AddSkill): Promise<void> => {
      await invoke('bulk_add_skills', { students, skillsType, ...payload })
    },
    addCourses: async (students: number[], payload: AddCourse): Promise<void> => { await invoke('bulk_add_courses', { students, ...payload }) },
    addAssesments: async (part: string, assessments: AddAssessment[]): Promise<void> => { await invoke('bulk_add_assessments', { part, assessments }) }
  },
  certifications: {
    list: async (studentId: number): Promise<SelectOption[]> => await invoke('get_certifications', { studentId }),
    authorities: async (studentId: number, certificationId: string): Promise<SelectOption[]> => await invoke('get_certification_authorities', { studentId, certificationId })
  },
  courses: {
    options: async (studentId: number): Promise<CTEOptions> => await invoke('get_course_options', { studentId }),
    teachers: async (studentId: number, courseId: string): Promise<SelectOption[]> => await invoke('get_teachers', { studentId, courseId })
  }
}

export default _invoke
