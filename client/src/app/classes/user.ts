import { Course } from "./course";

export interface User{
    name: string,
    password: string,
    school: string,
    courses: Course[],
    email: string,
    Description: any
}