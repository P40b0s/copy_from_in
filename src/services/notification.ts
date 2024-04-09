import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';
import { NButton, NotificationType } from 'naive-ui';
import { NotificationApiInjection } from 'naive-ui/es/notification/src/NotificationProvider';
import { CSSProperties, h } from 'vue';


export const notify = async (title: string, body : string) =>
{
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) 
    {
      const permission = await requestPermission();
      permissionGranted = permission === 'granted';
    }
    console.log(permissionGranted);
    if (permissionGranted) 
    {
      sendNotification('Tauri is awesome!');
      sendNotification({ title: title, body: body });
    }
}


export const naive_notify =  (injection: NotificationApiInjection, type: NotificationType, title: string, description: string) => 
{
    const n = injection.create({
        type: type,
        title: title,
        description: description,
        // avatar:() => h(NProgress,
        //     {
        //         style:{
        //             width: '100px',
        //             marginBottom: '20px'
        //         } as CSSProperties,
        //         type: 'circle',
        //         //circleGap: 0.6,
        //         //strokeWidth: 10,
        //         status: 'info',
        //         indicatorPosition: 'inside',
                
        //         percentage: tw.progress,
        //         color: 'rgba(255, 43, 15, 0.8)',
        //         railColor: 'rgba(84, 237, 33, 0.8)'
                
        //     },
        //     {
        //         default:()=> h(NIcon,
        //         {
        //             style:{
        //                 marginTop: '14px'
        //             } as CSSProperties,
        //             size: '20',
        //             color: 'rgba(255, 43, 15, 0.8)',
        //             component: WarningOutline
        //         }),
        //     }
        //     ),
        duration: 25500,
        keepAliveOnHover: true
    })
}

//export const notification = useNotification();
        
// const notify =  (type: NotificationType, ) => 
// {
//     const n = notification.create({
//         type: type,
//         title: "Осталось " + tw.minutes_left + " мин.",
//         description: "Автоматическое напоминание на " + tw.time,
//         content: tw.text,
       
//         //meta: tw.id.toString(),
//         action: () =>
//             h(
//             NButton,
//             {
//                 text: true,
//                 type: 'primary',
//                 onClick: () => 
//                 {
//                     tw.showNotify = false;
//                     n.destroy()
//                 }
//             },
//             {
//                 default: () => 'Больше не показывать'
//             }
//             ),
//         avatar:() => h(NProgress,
//             {
//                 style:{
//                     width: '100px',
//                     marginBottom: '20px'
//                 } as CSSProperties,
//                 type: 'circle',
//                 //circleGap: 0.6,
//                 //strokeWidth: 10,
//                 status: 'info',
//                 indicatorPosition: 'inside',
                
//                 percentage: tw.progress,
//                 color: 'rgba(255, 43, 15, 0.8)',
//                 railColor: 'rgba(84, 237, 33, 0.8)'
                
//             },
//             {
//                 default:()=> h(NIcon,
//                 {
//                     style:{
//                         marginTop: '14px'
//                     } as CSSProperties,
//                     size: '20',
//                     color: 'rgba(255, 43, 15, 0.8)',
//                     component: WarningOutline
//                 }),
//             }
//             ),
//         duration: 25500,
//         keepAliveOnHover: true
//     })
// }