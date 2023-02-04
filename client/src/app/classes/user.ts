import { Course } from "./course";

export interface User{
    name: string,
    school: string,
    courses: Course[],
    email: string,
    Description: any
}