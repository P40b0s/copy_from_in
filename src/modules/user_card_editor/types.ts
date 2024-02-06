import { FormInst } from 'naive-ui';
import { DiseaseEditor } from './disease_editor';
import { UserInfoEditor } from './user_info_editor';
import { VacationEditor } from './status_editor';
import { DefineComponent } from 'vue';

type SaveForm =
{
    save_form:() => Promise<boolean>
    validate:() => Promise<boolean>
}

export type SaveFormInstance = DefineComponent & SaveForm
//export type UserEditorInstance = typeof UserInfoEditor & SaveForm
//export type VacationEditorInstance = typeof VacationEditor & SaveForm

export const validateForm = async (formRef: FormInst | null) : Promise<boolean> =>
{
    return await formRef?.validate((errors)=> 
    {
        if (!errors)
        {
            return false;
        } 
        else
        {
            console.log(errors)
            return true;
        }
    }).then(() =>{return true;}).catch(e =>
    {
        return false;
    }) ?? false;
}