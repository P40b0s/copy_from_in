import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';


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