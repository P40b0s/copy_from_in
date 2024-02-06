import { Disease, DiseaseTest, DiseaseType, Journal, JournalItem, Dictionary, Ordered, Phones, User, Vacation, Vactination } from "../models/user";
import { TypesBuilder } from "./data";
import { dateToString } from "./date";

const test_disease_types = 
[
    {id: 'd428fc2b-db42-4737-a211-41467c41809d', name: "Тест_1", needReference: true},
    {id: '603d4cf0-52f1-4842-83b1-fc7edc5f3386', name: "Тест_2", needReference: true},
    {id: 'f4117313-4f0e-4024-875e-67a7c729da27', name: "Тест_3", needReference: false},
    {id: '3366ce11-3222-4f63-8385-4715cc108a6e', name: "Неврология", needReference: false},
    {id: 'e294e23a-03d2-4cd6-a591-6c10c97864df', name: "Травма", needReference: false},
    {id: '1e393a7c-de86-46cd-9467-04dd954f346d', name: "Другое", needReference: false},
] as DiseaseType[];

const test_clinics =
[
    {id: 'd428fc2b-db42-4737-a211-41467c41809d', name: "1 поликлинника ФСБ РФ"},
    {id: '603d4cf0-52f1-4842-83b1-fc7edc5f3386', name: "2 поликлинника ФСБ РФ"},
    {id: 'f4117313-4f0e-4024-875e-67a7c729da27', name: "3 поликлинника ФСБ РФ"},
    {id: '3366ce11-3222-4f63-8385-4715cc108a6e', name: "4 поликлинника ФСБ РФ"},
    {id: '1e393a7c-de86-46cd-9467-04dd954f346d', name: "5 поликлинника ФСБ РФ"},
] as Dictionary[];

const test_ranks =
[
    {id: 'd428fc2b-db42-4737-a211-41467c41809d', name: "Прапорщик"},
    {id: '603d4cf0-52f1-4842-83b1-fc7edc5f3386', name: "Лейтенант"},
    {id: 'f4117313-4f0e-4024-875e-67a7c729da27', name: "Старший лейтенант"},
    {id: '3366ce11-3222-4f63-8385-4715cc108a6e', name: "Капитан"},
    {id: '1e393a7c-de86-46cd-9467-04dd954f346d', name: "Майор"},
    {id: 'a15a272b-ff94-453d-9929-cb382827e07a', name: "Подполковник"},
    {id: '9f12f7fc-60e1-40a6-af27-a9a2b6bb8b78', name: "Полковник"},
] as Dictionary[];

const test_posts =
[
    {id: 'd428fc2b-db42-4737-a211-41467c41809d', name: "Сотрудник"},
    {id: '603d4cf0-52f1-4842-83b1-fc7edc5f3386', name: "Старший сотрудник"},
    {id: 'f4117313-4f0e-4024-875e-67a7c729da27', name: "Инспектор"},
    {id: '3366ce11-3222-4f63-8385-4715cc108a6e', name: "Старший инспектор"},
    {id: 'e294e23a-03d2-4cd6-a591-6c10c97864df', name: "Инженер"},
] as Dictionary[];

const test_departments =
[
    {id: 'd428fc2b-db42-4737-a211-41467c41809d', name: "Руководство"},
    {id: '603d4cf0-52f1-4842-83b1-fc7edc5f3386', name: "Секретариат"},
    {id: 'f4117313-4f0e-4024-875e-67a7c729da27', name: "1 отдел"},
    {id: '3366ce11-3222-4f63-8385-4715cc108a6e', name: "2 отдел"},
    {id: 'e294e23a-03d2-4cd6-a591-6c10c97864df', name: "3 отдел"},
] as Dictionary[];


const test_journal =
{
    date: dateToString(new Date()),
    items:
    [
        {time : "09:00", note: "тестовая запись"} as JournalItem,
        {time : "09:10", note: "тестовая запись 2"} as JournalItem
    ]
} as Journal

const diseases_list = () =>
{
    // let dis1 = {
    //     id: '9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d',
    //     diseaseType: {id: 'd428fc2b-db42-4737-a211-41467c41809d', name: "123", needReference: false}
    //     dateOfIllness: "20.07.2021",
    //     dateOfRecovery: "30.07.2021",
    //     clinic: "1 Поликлинника ФСБ РФ",
    // } as Disease

    // let dis2 = {
    //     id: '9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d',
    //     type: '603d4cf0-52f1-4842-83b1-fc7edc5f3386',
    //     dateOfIllness: "10.09.2021",
    //     clinic: "1 Поликлинника ФСБ РФ",
    // } as Disease

    // let dis3 = {
    //     id: '1b9d6bcd-bbfd-4b2d-9b5d-ab8dfbbd4bed',
    //     type: 'd428fc2b-db42-4737-a211-41467c41809d',
    //     dateOfIllness: "20.07.2021",
    //     dateOfRecovery: "30.07.2021",
    //     clinic: "1 Поликлинника ФСБ РФ",
    // } as Disease

    // let dis4 = {
    //     id: '1b9d6bcd-bbfd-4b2d-9b5d-ab8dfbbd4bed',
    //     type: 'f4117313-4f0e-4024-875e-67a7c729da27',
    //     dateOfIllness: "10.09.2021",
    //     clinic: "1 Поликлинника ФСБ РФ",
    // } as Disease
    return [] as Disease[]
}

const vactionation_list = () =>
{
    const vactination = {
        id: '1b9d6bcd-bbfd-4b2d-9b5d-ab8dfbbd4bed',
        type: '603d4cf0-52f1-4842-83b1-fc7edc5f3386',
        date: "29.08.2023",
    } as Vactination

    const vactination2 = {
        id: '9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d',
        type: 'd428fc2b-db42-4737-a211-41467c41809d',
        date: "09.09.2023",
    } as Vactination

    return [vactination, vactination2]
}
const vacation_list = () =>
{
    const vacation1 = TypesBuilder.build_vacation('1b9d6bcd-bbfd-4b2d-9b5d-ab8dfbbd4bed', new Date(2023, 11, 11), 20);
    const vacation2 = TypesBuilder.build_vacation('1b9d6bcd-bbfd-4b2d-9b5d-ab8dfbbd4bed', new Date(2023, 9, 11), 40);
    vacation1.place = "Крым, оленевка";
    vacation2.place = "Дома сижу";
    return [vacation1, vacation2]
}

const ordered = {
    id: '1b9d6bcd-bbfd-4b2d-9b5d-ab8dfbbd4bed',
    startDate: "22.11.2023",
    endDate: "29.12.2023",
    place: "Распоряжение хз куда"
}   as Ordered

const trip = {
    id: '9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d',
    startDate: "22.11.2023",
    endDate: "29.12.2023",
    place: "Содный отряд"
}   as Ordered



const test_data = () =>
{
    const phones1 = {
        phoneType: "стационарный",
        phoneNumber: "352-52-52"
    } as Phones;

    const phones2 = {
        phoneType: "мобильный",
        phoneNumber: "999-99-99"
    } as Phones;

    let test_1 = 
    {
        isActive: true,
        date: "08.12.2023",
    } as DiseaseTest;
    let test_2 = 
    {
        isActive: false,
        date: "10.12.2023",
    } as DiseaseTest;

    


    const vacation = {
        startDate: "03.11.2023",
        daysCount: 100,
        place: "Оленевка, Крым"
    }   as Vacation

    const vactination = {
        type: '9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d',
        date: "29.08.2023",
    } as Vactination

    const vactination2 = {
        type: '1b9d6bcd-bbfd-4b2d-9b5d-ab8dfbbd4bed',
        date: "09.09.2023",
    } as Vactination

    // const user = {
    //     id: '1b9d6bcd-bbfd-4b2d-9b5d-ab8dfbbd4bed',
    //     name_1: "Алексей",
    //     name_2: "Игоревич",
    //     surname: "Пиксар",
    //     post: {id: '603d4cf0-52f1-4842-83b1-fc7edc5f3386', name: 'Аяаяая'},
    //     department: {id: '603d4cf0-52f1-4842-83b1-fc7edc5f3386', name: "новые разработки"},
    //     rank:  {id: '603d4cf0-52f1-4842-83b1-fc7edc5f3386', name: "Прапорщик"},
    //     livePlace: "Марс, космопорт 'Рубин'",
    //     phones: [phones1, phones2],
    //     vacations: [vacation],
    //     vactinations: [vactination, vactination2],
    //     tests: [test_1, test_2],
    //     diseases: [],
    // } as User

    // const user2 = {
    //     id: '9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d',
    //     name_1: "Джон",
    //     name_2: "Андреевич",
    //     surname: "Лапоть",
    //     post: {id: 'f4117313-4f0e-4024-875e-67a7c729da27', name: 'Аяаяая'},
    //     department: {id: 'f4117313-4f0e-4024-875e-67a7c729da27', name: "новые разработки"},
    //     rank:  {id: 'f4117313-4f0e-4024-875e-67a7c729da27', name: "Прапорщик"},
    //     livePlace: "Шелезяка, космопорт '1В2'",
    //     phones: [phones1, phones2],
    //     vacations: [vacation],
    //     vactinations: [vactination],
    //     tests: [],
    //     diseases: [],
    // } as User

    //return [user, user2]
}
export {test_disease_types, test_clinics, test_ranks, test_journal, test_posts, test_departments, test_data, trip, ordered, vacation_list, vactionation_list, diseases_list}