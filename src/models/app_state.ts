import { User } from "./user"


type AppState = 
{
    diseases_count: number,
    vacations_count: number,
    users_count: number,
    buisness_trip_count: number,
    ordered_count: number,
    current_disease_users: User[],
    users_with_statuses: User[],
    current_date: string
}
export {type AppState}