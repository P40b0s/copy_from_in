import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    ref,
    VNode,
    RendererNode,
    RendererElement,
    onMounted,
    onUnmounted
  } from 'vue'

import { NSpin } from 'naive-ui';
import { DictionaryEditorType, DiseaseType, Dictionary } from '../models/user.ts';
import { time_warnings, updateTimeWarnings, users } from '../services/data.ts';
import {DictionaryEditor} from './dictionary_editor.tsx'
import { match } from 'ts-pattern';
import emitter from '../services/emit.ts';
import {EditorSelector} from './user_card_editor/editor_selector.tsx'
import { JournalEditor } from './journal_editor.tsx';
import { TimeWarningsEditor } from './time_warnings/time_warnings_editor.tsx';
import { clinics, departments, disease_types, posts, ranks, updateClinics, updateDepartments, updateDisesesTypes, updatePosts, updateRanks } from '../services/dictionaries.ts';


type drop_opts = 
{
  label: string,
  key: DictionaryEditorType,
  icon:() => VNode<RendererNode, RendererElement, { [key: string]: any; }>
}

const localProps = 
{
    size: 
    {
        type: Number,
        default: 100
    },
} as const

export const ModalsAsync = defineAsyncComponent({
    loader: () => import ('./modals.tsx'),
    loadingComponent: h(NSpin)
})

export type ModalEditorType = 'journal' | 'time-warning'

export const Modals =  defineComponent({
props: localProps,
    setup (props) 
    {
      const deps_editor = ref(false);
      const posts_editor = ref(false);
      const clinics_editor = ref(false);
      const diseases_editor = ref(false);
      const ranks_editor = ref(false);
      const journal_editor = ref(false);
      const time_warnings_editor = ref(false);
      onMounted(()=>
      {
        emitter.on("openDictionaryEditor", (t) =>
        {
            match(t)
            .with('posts', ()=> posts_editor.value = true)
            .with('departments', ()=> deps_editor.value = true)
            .with('clinic', ()=> clinics_editor.value = true)
            .with('disease', ()=> diseases_editor.value = true)
            .with('ranks', ()=> ranks_editor.value = true)
            .exhaustive()
        })
        emitter.on("openModalWindow", (type) =>
        {
          match(type)
          .with('journal', () => journal_editor.value = true)
          .with('time-warning', () => time_warnings_editor.value = true)
          .exhaustive()
        })
      });
      onUnmounted(()=>
      {
        emitter.off('openDictionaryEditor');
        emitter.off('openModalWindow');
      })
      
      const modal_windows = () =>
      {
        return h('div',
        [
          h(DictionaryEditor, 
            {
                is_open: deps_editor.value,
                values: departments.value,
                'onUpdate:value': async (v)=> 
                {
                    await updateDepartments(v);
                    deps_editor.value = false
                },
                'onClose':(c:boolean) => deps_editor.value = c
            }),
            h(DictionaryEditor, 
            {
                is_open: posts_editor.value,
                values: posts.value,
                'onUpdate:value':async (v)=> 
                {
                    await updatePosts(v);
                    posts_editor.value = false
                },
                'onClose':(c:boolean) => posts_editor.value = c
            }),
            h(DictionaryEditor, 
            {
                is_open: clinics_editor.value,
                values: clinics.value,
                'onUpdate:value': async (v)=> 
                {
                  await updateClinics(v);
                  clinics_editor.value = false
                },
                'onClose':(c:boolean) => clinics_editor.value = c
            }),
            h(DictionaryEditor, 
            {
                is_open: diseases_editor.value,
                values: disease_types.value as Dictionary[],
                'onUpdate:value': async(v)=> 
                {
                  await updateDisesesTypes(v as DiseaseType[]);
                  diseases_editor.value = false
                },
                'onClose':(c:boolean) => diseases_editor.value = c
            }),
            h(DictionaryEditor, 
              {
                is_open: ranks_editor.value,
                values: ranks.value as Dictionary[],
                'onUpdate:value': async(v)=> 
                {
                  await updateRanks(v);
                  ranks_editor.value = false
                },
                'onClose':(c:boolean) => ranks_editor.value = c
              }),
            h(EditorSelector, 
            {
                
            }),
            h(JournalEditor,
            {
              is_open: journal_editor.value,
              'onClose':(c:boolean) => journal_editor.value = c,
            }),
            h(TimeWarningsEditor,
              {
                value: time_warnings.value,
                is_open: time_warnings_editor.value,
                'onClose':(c:boolean) => time_warnings_editor.value = c,
                'onUpdate:value':(v)=> 
                {
                  updateTimeWarnings(v);
                  time_warnings_editor.value = false;
                },
              })

        ])
      }
      return {modal_windows}
    },
    
    
    render ()
    {
      return this.modal_windows();
    }
})

