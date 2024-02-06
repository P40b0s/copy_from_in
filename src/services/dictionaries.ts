import { DiseaseType, Dictionary, User } from '../models/user';
import { ref } from 'vue';
import { test_clinics, test_departments, test_disease_types, test_posts, test_ranks } from './test_data';
import emitter from './emit';
import { TauriCommands } from './tauri';

const posts = ref<Dictionary[]>(await TauriCommands.Dictionaries.get_posts() ?? []);
const departments = ref<Dictionary[]>(await TauriCommands.Dictionaries.get_departments() ?? []);
const disease_types = ref<DiseaseType[]>(await TauriCommands.Dictionaries.get_diseases_types() ?? []);
const getDiseaseType = (t: string): DiseaseType|undefined => disease_types.value.findLast(f=>f.id == t)
const clinics = ref<Dictionary[]>(await TauriCommands.Dictionaries.get_clinics() ?? [])
const ranks = ref<Dictionary[]>(await TauriCommands.Dictionaries.get_ranks() ?? [])

const updateDisesesTypes = async (dt: DiseaseType[]) =>
{
    const new_dis_list = await TauriCommands.Dictionaries.save_diseases_types(dt);
    if(new_dis_list)
    {
        disease_types.value = new_dis_list;
        emitter.emit('userUpdated');
    }
    else
    {
        console.error("Ошибка при обновлнеии словаря видов заболеваний");
        console.error(dt);
    }
}

const updatePosts = async (dict: Dictionary[]) =>
{
    const new_posts = await TauriCommands.Dictionaries.save_posts(dict);
    if(new_posts)
    {
        posts.value = new_posts;
        emitter.emit('userUpdated');
    }
    else
    {
        console.error("Ошибка при обновлнеии словаря posts");
        console.error(dict);
    }
}
const updateClinics = async (dict: Dictionary[]) =>
{
    const new_clinics = await TauriCommands.Dictionaries.save_clinics(dict);
    if(new_clinics)
    {
        clinics.value = new_clinics;
        emitter.emit('userUpdated');
    }
    else
    {
        console.error("Ошибка при обновлнеии словаря clinics");
        console.error(dict);
    }
}
const updateDepartments = async (dict: Dictionary[]) =>
{
    const new_val = await TauriCommands.Dictionaries.save_departments(dict);
    if(new_val)
    {
        departments.value = new_val;
        emitter.emit('userUpdated');
    }
    else
    {
        console.error("Ошибка при обновлнеии словаря departments");
        console.error(dict);
    }
}
const updateRanks = async (dict: Dictionary[]) =>
{
    const new_val = await TauriCommands.Dictionaries.save_ranks(dict);
    if(new_val)
    {
        ranks.value = new_val;
        emitter.emit('userUpdated');
    }
    else
    {
        console.error("Ошибка при обновлнеии словаря ranks");
        console.error(dict);
    }
}


export {disease_types, getDiseaseType, updateRanks, updateDepartments, updateDisesesTypes, updateClinics, updatePosts, posts, departments, clinics, ranks}