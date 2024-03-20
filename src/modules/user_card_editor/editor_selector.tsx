import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    ref,
    toRaw,
    Ref,
    VNodeRef
  } from 'vue'

import { FormInst, NButton, NModal,  NScrollbar,  NSpin, NTabPane, NTabs } from 'naive-ui';
import { Disease, EditorType, User, Vacation} from '../../models/user.ts';
import { match } from 'ts-pattern';
import emitter from '../../services/emit.ts';
import  {DiseaseEditor}  from './disease_editor.tsx';
import  {UserInfoEditor}  from './user_info_editor.tsx';
import { VacationEditor } from './status_editor.tsx';
import { SaveFormInstance } from './types.ts';
import { OrderedEditor } from './ordered_editor.tsx';
//import { BuisnessTripEditor } from './buisness_trip_editor.tsx';
//import { VactinationEditor } from './vactination_editor.tsx';

const show_editor = ref(false);
const editorType = ref<EditorType>('edit');
const user = ref<User>() as Ref<User>;
const is_new = ref(false);
emitter.on("openUserCardEditor", ({current_user, type}) => 
{
    is_new.value = type == 'new' ? true : false;
    //console.log(type, is_new.value)
    user.value = structuredClone(toRaw(current_user)),
    show_editor.value = true;
})

const cantSave = ref(false);
// const validateForm = async (formRef: FormInst | null) =>
// {
//     await formRef?.validate((errors)=> 
//     {
//         if (!errors)
//         {
//             cantSave.value =  false;
//         } 
//         else
//         {
//             console.log(errors)
//             cantSave.value = true;
//         }
//     }).then(() =>{}).catch(e =>
//     {
//         //console.log(e)
//     });
// }

export const EditorSelectorAsync = defineAsyncComponent({
    loader: () => import ('./editor_selector.tsx'),
    loadingComponent: h(NSpin)
})

//сохраню на всякию случай, это как извлекать значения из энума
// const diseases = (): Array<SelectOption | SelectGroupOption> =>
// {
//     return enumKeys(DiseaseType).map(m=>
//     {
//         const val = DiseaseType[m];
//         if (typeof val === "string")
//         {
//             return {
//                 label: val,
//                 value: val,
//                 disabled: false
//             }
//         }
//         else
//         {
//             return {
//                 label: DiseaseType.Other,
//                 value: DiseaseType.Other,
//                 disabled: true
//             }
//         }
//     })
// }
/**Редактор карточки юзера (личная информация, заболевания и отпуска)*/
export const EditorSelector = defineComponent({
name: 'EditorSelector',
//props: localProps,
emits:
{
    'update:user': (u: User) => u,
},
    setup (props, {emit}) 
    {
        const labels_style = () =>
        {
            return {
                fontWeight: 700,
                color: "#256a74"
            } as CSSProperties
        }
        const editorType = ref<SaveFormInstance|null>(null);
        //const userEditor = ref<SaveFormInstance|null>(null);
        //const vacationEditor = ref<SaveFormInstance|null>(null);

        /**Модальное окно с редактором карточки*/
        const modal = () =>
        {
            return h(NModal,
            {
                show: show_editor.value,
                closable: true,
                closeOnEsc: true,
                preset: 'card',
                "onUpdate:show":()=> show_editor.value = false,
                style:
                {
                    border: '1px solid rgba(255, 255, 255, .25)',
                    borderRadius: '20px',
                    //backgroundColor: 'rgba(255, 255, 255, 0.45)',
                    boxShadow: '0 0 10px 1px rgba(0, 0, 0, 0.25)',
                    backdropFilter: 'blur(15px)',
                    // position: 'relative',
                    // top: '50%',
                    // transform: 'translateY(-5%)',
                    width: '800px',
                } as CSSProperties,
            },
            {
                default:()=>
                user.value ?
                //editor_selector() : []
                tab_view(): [],
                action:() => h('div',
                {
                    style:
                    {
                        display: 'flex',
                        flexDirection: 'row',
                        background: 'transparent',
                        justifyContent: 'space-between',
                        alignItems: 'center'
                    } as CSSProperties,
                },
                [save_form_button(), cancel_button()]
                )
               
            }
            )
        }

        const tab_view = () =>
        {
            return h(NTabs,
                {
                    
                    justifyContent: 'space-evenly',
                    type: 'line',
                    defaultValue: "user_card"
                },
                {
                    default:() => is_new.value ? user_card()
                    : [user_card(), user_disease(), user_vacation(), user_ordered(), user_buisness_trip(), user_vactination()]
                }
            )
        }
    
        const user_card = () => 
        {
            return h(NTabPane,
                {
                    tab: 'Личная информация',
                    name: 'user_card'
                },
                {
                    default:() =>h(NScrollbar,
                    {
                        style:
                        {
                            maxHeight: '600px',
                            paddingRight: '15px',
                        }   as CSSProperties
                    },
                    {
                        default:() => h(UserInfoEditor,
                        {
                            user: user.value,
                            ref: editorType,
                            styles: labels_style()
                        },
                        {
                            // save:({items, form}: {items: User, form: FormInst | null}) => 
                            // save_button(()=>
                            // {
                            //     console.log(items.surname)
                            //     updateUser(items);
                            // }, form),
                        })
                    })
                     
                }
            )
        }
        const user_disease = () => 
        {
            return h(NTabPane,
                {
                    tab: 'Заболевание',
                    name: 'user_disease',

                },
                {
                    default:() =>h(NScrollbar,
                    {
                        style:
                        {
                            maxHeight: '600px',
                            paddingRight: '15px',
                        }   as CSSProperties
                    },
                    {
                        default:() => h(DiseaseEditor,
                        {
                            user: user.value,
                            ref: editorType,
                            styles: labels_style()
                        },
                        {
                            // save:({items, form}: {items: Disease[], form: FormInst | null}) => 
                            // save_button(() =>
                            // {
                            //     updateUser(user.value);
                            //     updateDiseases(items);
                            // }, form),
                        })
                    })
                }
            )
        }
        const user_vacation = () => 
        {
            return h(NTabPane,
                {
                    tab: 'Отпуск',
                    name: 'user_vacation',

                },
                {
                    default:() =>h(NScrollbar,
                    {
                        style:
                        {
                            maxHeight: '600px',
                            paddingRight: '15px',
                        }   as CSSProperties
                    },
                    {
                        default:() => h(VacationEditor,
                        {
                            user: user.value,
                            ref: editorType,
                            styles: labels_style(),
                        },
                        {
                            // save:({items, form}: {items: Vacation[], form: FormInst | null}) => 
                            // save_button(() =>
                            // {
                            //     updateUser(user.value);
                            //     updateVacations(items);
                            // }, form),
                        })
                    })
                   
                }
            )
        }
        const user_ordered = () => 
        {
            return h(NTabPane,
                {
                    tab: 'Распоряжение',
                    name: 'user_ordered',

                },
                {
                    default:() =>h(NScrollbar,
                    {
                        style:
                        {
                            maxHeight: '600px',
                            paddingRight: '15px',
                        }   as CSSProperties
                    },
                    {
                        default:() => h(OrderedEditor,
                        {
                            user: user.value,
                            ref: editorType,
                            styles: labels_style()
                        })
                    })
                   
                }
            )
        }
        const user_buisness_trip = () => 
        {
            return h(NTabPane,
                {
                    tab: 'Командировка',
                    name: 'user_buisness_trip',

                },
                {
                    default:() =>h(NScrollbar,
                    {
                        style:
                        {
                            maxHeight: '600px',
                            paddingRight: '15px',
                        }   as CSSProperties
                    },
                    {
                        // default:() => h(BuisnessTripEditor,
                        // {
                        //     user: user.value,
                        //     ref: editorType,
                        //     styles: labels_style()
                        // })
                    })
                   
                }
            )
        }

        const user_vactination = () => 
        {
            return h(NTabPane,
                {
                    tab: 'Вакцинации',
                    name: 'user_vactinations',

                },
                {
                    default:() =>h(NScrollbar,
                    {
                        style:
                        {
                            maxHeight: '600px',
                            paddingRight: '15px',
                        }   as CSSProperties
                    },
                    {
                        // default:() => h(VactinationEditor,
                        // {
                        //     user: user.value,
                        //     styles: labels_style(),
                        //     ref: editorType
                        // },
                        // {
                        //     // save:({items, form}: {items: Vacation[], form: FormInst | null}) => 
                        //     // save_button(() =>
                        //     // {
                        //     //     updateUser(user.value);
                        //     //     updateVacations(items);
                        //     // }, form),
                        // })
                    })
                    
                }
            )
        }

        const save_emulation = ref(false);
        const save_form_button = () => 
        {
            return h('div', 
            {
                style:{
                    display: 'flex',
                    flexDirection: 'column',
                    justifyContent: 'space-evenly',
                    alignItems: 'center',
                    color: 'red',
                    marginTop: '15px'


                }  as CSSProperties
            }, 
            [
                h(NButton,
                {
                    type: cantSave.value ? 'error' : 'success',
                    disabled: cantSave.value,
                    loading: save_emulation.value,
                    'onmouseover': async () =>
                    {
                        console.log("валидация формы....")
                        if(editorType.value)
                            cantSave.value = !await editorType.value?.validate();
                        // if(userEditor.value)
                        //     cantSave.value = !await userEditor.value?.validate();
                        // if(vacationEditor.value)
                        //     cantSave.value = !await vacationEditor.value?.validate();
                        //console.log(cantSave.value);
                       
                    },
                    onClick: async (e)=> 
                    {
                        save_emulation.value = true;
                        await editorType.value?.save_form();
                        if(!cantSave.value)
                        {
                            //updateCounts();
                        }
                        let timerId: number = setInterval((()=>
                        {
                            save_emulation.value = false;
                        }) as TimerHandler, 1000)
                    },
                },
                {
                    default:()=> 'Сохранить',
                }),
                h('span', {style: {height: '10px'}}, cantSave.value ?'Ошибка! Не все поля заполнены правильно!' : "")
                
            ])
        }
        const cancel_button = () => 
        {
            return h(NButton,
                {
                    type:  'error',
                    onClick:(e)=> 
                    {
                        show_editor.value = false;
                    },
                },
                {
                    default:()=> 'Закрыть',
                })
        }

        // const save_button = (click: () => void, validationForm?: FormInst | null) => 
        // {
        //     return h('div', 
        //     {
        //         style:{
        //             display: 'flex',
        //             flexDirection: 'column',
        //             justifyContent: 'space-evenly',
        //             alignItems: 'center',
        //             color: 'red',
        //             marginTop: '15px'


        //         }  as CSSProperties
        //     }, 
        //     [
        //         h(NButton,
        //         {
        //             type: cantSave.value ? 'error' : 'success',
        //             disabled: cantSave.value,
        //             'onmouseover': async () =>
        //             {
        //                 if(validationForm)
        //                 {
        //                     await validateForm(validationForm);
        //                 }
        //                 else
        //                 {
        //                     cantSave.value = false;
        //                 }
        //             },
        //             onClick:(e)=> 
        //             {
        //                 click();
        //                 show_editor.value = false;
        //             },
        //         },
        //         {
        //             default:()=> 'Сохранить',
        //         }),
        //         h('span', {style: {height: '10px'}}, cantSave.value ?'Ошибка! Не все поля заполнены правильно!' : "")
                
        //     ])
        // }
        return {modal}
    },
    render ()
    {
        return h(this.modal)
    }
})