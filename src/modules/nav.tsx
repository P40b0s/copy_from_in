import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    Component,
    VNode,
    RendererNode,
    RendererElement
  } from 'vue'

import { NAvatar, NBadge, NButton, NDropdown, NIcon, NPopover, NSpin, NTooltip} from 'naive-ui';
import { DictionaryEditorType, User, get_active_dis} from '../models/user.ts';
import { AppsSharp, Book, Medkit, People, Star, TrailSignSharp } from '@vicons/ionicons5';
import emitter from '../services/emit.ts';
import global_store from '../store/global_store.ts';
import { bell_ico, disease_ico, journal_ico, palm_ico } from '../services/svg.ts';
import { app_state_store } from '../store/index.ts';
import { match } from 'ts-pattern';


type drop_opts = 
{
  label: string,
  key: DictionaryEditorType,
  icon:() => VNode<RendererNode, RendererElement, { [key: string]: any; }>
}

export const NavMenuAsync = defineAsyncComponent({
    loader: () => import ('./nav.tsx'),
    loadingComponent: h(NSpin)
})

export const NavMenu =  defineComponent({
    setup (props) 
    {
      const renderIcon = (icon: Component) => 
      {
        return () => 
        {
          return h(NIcon, null, 
          {
            default: () => h(icon)
          })
        }
      }

      const dropdown_options = () : drop_opts[] =>
      {
        return [
          {
            label: 'Звания',
            key: 'ranks',
            icon: renderIcon(Star)
          },
          {
            label: 'Должности',
            key: 'posts',
            icon: renderIcon(People)
          },
          {
            label: 'Отделы',
            key: 'departments',
            icon: renderIcon(AppsSharp)
          },
          {
            label: 'Заболевания',
            key: 'disease',
            icon: renderIcon(Medkit)
          },
          {
            label: 'Поликлинники',
            key: 'clinic',
            icon: renderIcon(TrailSignSharp)
          },
        ]
      }
      const menu = () => 
      {
        return  h('div',
        {
          style: 
          {
            padding: '5px',
            display: 'flex',
            flexDirection: 'row'
          } as CSSProperties
        },
        [
          menu_panel(),
          status_panel()
        ])
      }

      const menu_button = (tooltip: string, icon: string, onClick:(e: MouseEvent) => void) =>
        {
            return h(NTooltip,{placement: 'top'},
            {
                trigger:() =>
                h(NButton,{
                    round: true,
                    text: true,
                    size: 'small',
                    onClick:(c) =>
                    {
                        onClick(c);
                    },
                    style:
                    {
                        backgroundColor: 'transparent'
                    }
                },
                {
                      default:() => h(NAvatar,
                    {
                        size: 30,
                        src: icon,
                        round: true,
                        class: 'hover-button',
                        style:
                        {
                            '--hover-button-boxshadow': '#6cdb39c7',
                            backgroundColor: 'transparent',
                        }   as CSSProperties
                        
                    }),
                }),
                default:() => tooltip
            })
        }  

      const menu_panel = () =>
      {
        return h('div',
        {
          style: {
            display: 'flex',
            flexDirection: 'row',
            gap: '6px'
          } as CSSProperties
        },
        [
          h(NDropdown,
            {
              options: dropdown_options(),
              onSelect(value: DictionaryEditorType)
              {
                emitter.emit("openDictionaryEditor", value)
              },
            },
            {
              default:() => 
              h(NButton,{},
              {
                default:() => "Словари",
                icon:() => h(NIcon, {component: Book})
              })
            }),
            menu_button("Журнал", journal_ico, (e) =>  emitter.emit('openModalWindow', 'journal')),
            menu_button("Диспетчер задач", bell_ico, (e) =>  emitter.emit('openModalWindow', 'time-warning'))
        ])
      }
      const status_panel = () =>
      {
        return  h('div',
        {
          style:
          {
            width: '100%',
            height: '100%',
            display: 'flex',
            marginRight: '5px',
            justifyContent: 'space',
            flexDirection: 'row-reverse'
          } as CSSProperties
        },
        [
          status_icon(palm_ico, "Отпуск", app_state_store.getState().appState.current_vacation_users),
          status_icon(disease_ico, "Заболевание", app_state_store.getState().appState.current_disease_users),
        ])
      }
      const dis_style = (user: User) => 
      {
        const dt = get_active_dis(user.diseases);
        return match(dt)
        .with(undefined, () => "transparent")
        //covid
        .with({id: "9bb122b6-81a7-4967-a844-43733fa88e04"}, () => "red")
        .with({id: "09f179c8-9c82-4423-a7aa-4c8cbc9d637b"}, () => "green")
        .otherwise(() => "transparent");
      }
      const dis_type_title = (user: User) => 
      {
        const dt = get_active_dis(user.diseases);
        if (dt)
        {
          return dt.name
        }
        else
        {
          return ""
        }
      }
     
      const status_icon = (icon: string, title: string, users: User[]) => 
      {
        
        return h(NBadge,
          {
            type: 'info',
            color: 'transparent',
            offset: [-18,30]
          },
          {
            value:() => users.length,
            default:() => 
            h(NPopover,{placement: 'top'},
            {
                trigger:() =>
                h(NAvatar,
                {
                    size: 25,
                    src: icon,
                    style:
                    {
                        backgroundColor: 'transparent',
                        marginRight: '5px'
                    }   as CSSProperties
                    
                }),
                header:() => title + " " + users.length,
                default:() => h('div',
                {
                  style:
                  {
                    display:'flex',
                    flexDirection: 'column'
                  } as CSSProperties
                },
                users.map(m=> h('span',
                {
                  style: 
                  { 
                  
                  } as CSSProperties
                },
                m.surname + " -> " + dis_type_title(m)))),
            }),
          })
      }
      return {menu}
    },
    
    
    render ()
    {
      return this.menu()
    }
})