// import { ref } from "vue";
// import { TauriEvents } from "./tauri-service";

import { TauriEvents } from "./tauri-service";

// export const current_log = ref<string[]>(["123 34 2234 234", "TEST!"]);
// await TauriEvents.new_document_event((doc) => 
// {
//     console.log(doc);
//     const pl = doc.payload
//     if(pl.error)
//     {
//         current_log.value.push(pl.error);
//     }
//     if(pl.document)
//     {
//         current_log.value.push(pl.document.name);
//     }
// })
        

const ndc = async () => 
{
    await TauriEvents.new_document_event((doc) => 
    {
        console.log(doc);
        const pl = doc.payload
    })
}
export const start_events = () =>
{
    ndc();
}
