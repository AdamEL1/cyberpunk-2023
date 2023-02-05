import { Course } from "./course";
import { Description } from "./description";

export interface User{
    name: string,
    password: string,
    school: string,
    courses: Course[],
    email: string,
    description: Description
}