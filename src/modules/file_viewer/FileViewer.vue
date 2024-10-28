<template lang="pug">
n-drawer(v-model:show="is_open" style="min-width: 45vw;")
  n-drawer-content
    template(#header)
      div(style="display: flex; flex-direction: row; align-items: center; width: 100%;")
          n-tooltip(placement="left")  Выбор файла для просмотра 
            template(#trigger)
              drawler-file-selector(v-if="packet" :packet="packet" @onSelect="on_selected" style="min-width: 42vw;")
    template(#footer)
      n-pagination.paging(
            v-if="is_pdf"
            v-model:page="current_page"
            :disabled="in_progress"
            :page-count="pages"
            :on-update-page="change_page"
            size="medium"
            show-quick-jumper)
    div.loader.loader7(v-if="in_progress")
    div.pdf-paging(v-if="is_pdf")
      img(:src="current_image" @wheel="on_wheel" style="min-height: 500px" :class="{'bluring': in_progress, 'unbluring': !in_progress}")
      n-progress.progressbar(type="line" :percentage="percentage" :status="get_render_status()" :show-indicator="false")
      template(v-if="errors.length > 0" v-for="e in errors")
        div e
    highlightjs.code-block(v-if="!is_pdf" :language='lang' :code="body" :class="[{'is-pdf': is_pdf}, {'is-file': !is_pdf}]")
</template>
 
<script lang="ts">
import { ref, type Component, defineAsyncComponent, watch, inject, onMounted, onUnmounted } from 'vue';
import { NNotificationProvider, NButton, NIcon, NTooltip, NDrawer, NDrawerContent, NPagination, NSpin, NProgress} from 'naive-ui';
import { Analytics, Document, Warning, Settings, FingerPrintSharp, PieChart, PulseSharp } from '@vicons/ionicons5';
import { type Emitter, type Events } from "../../services/emit";
import { type Status } from 'naive-ui/es/progress/src/interface';
import {pdf_logic} from './pdf_logic';
import DrawlerFileSelector from '../file_selector/drawler_file_selector';
import {type IPacket, type File} from '../../models/types'
import { type SelectedValue } from '../file_selector/file_selector_label';

const FileViewer = defineAsyncComponent({
  loader: () => import('./FileViewer.vue'),
  loadingComponent: 
  {
    template : `
    <n-spin size="large"/>
  `}
});
</script>


<script lang="ts" setup>
const emitter = inject<Emitter<Events>>('emitter') as Emitter<Events>;
const is_open = ref(false);
const in_progress = ref(false);
const file = ref<string>();
const body = ref<string>("");
const lang = ref("xml");
const packet = ref<IPacket>()
let full_file_path = "";
const is_pdf = ref(false);

///selected path
const on_selected = (s: SelectedValue) =>
{
  console.log(s)
}


const {
  render_page,
  change_page,
  select_pdf,
  response_pdf_pages,
  current_image,
  current_page,
  pages,
  errors,
  on_wheel
} = pdf_logic(is_open, in_progress);

const request_file = (file_path: string) =>
{
  if(full_file_path != file_path)
  {
    console.log(full_file_path, file_path);
    full_file_path = file_path
    const splitted = file_path.split('/');
    //const p = packetService.packets.find(f=>f.packetDirectory && f.packetDirectory == splitted[0]);
    //packet.value = p;
    //console.log(p, splitted);
    //это файл
    //selected.value = splitted[splitted.length -1];
    //if(selected.value.indexOf(".pdf") >=0)
    {
      is_pdf.value = true;
      select_pdf(file_path);
      file.value = file_path;
    }
    //else
    {
      is_pdf.value = false;
      //let cmd = create_file_command(file_path);
      //file.value = file_path;
      //ws.send_message(cmd);
      in_progress.value = true;
    }
  }
  else
    is_open.value = true;
  
}

// const response_file = (response: File[]) =>
// {
//   if(response[0])
//   {
//     let file_ok = false;
//     switch(response[0].fileType)
//     {
//       case "rc":
//       case "xml":
//       {
//         lang.value = response[0].fileType;
//         body.value = response[0].body;
//         file_ok = true;
//         break;
//       }
//       case "ltr":
//       {
//         lang.value = "toml";
//         body.value = response[0].body;
//         file_ok = true;
//         break;
//       }
//       case "txt":
//       {
//         lang.value = "text";
//         body.value = response[0].body;
//         file_ok = true;
//         break;
//       }
//     }
//     if(file_ok)
//     {
//       in_progress.value = false;
//       is_open.value = true;
//     }
//   }
// }

emitter.on('packetItemDoubleClick', (p) => 
{
  packet.value = p;
  is_open.value = true;
});
//emitter.on('fileResponse', response_file);
//emitter.on('pdfRenderStatus', render_status)
//emitter.on('pdfResponsed', response_pdf_pages)

onUnmounted(()=> 
{
  emitter.off('packetItemDoubleClick')
  //emitter.off('pdfResponsed', response_pdf_pages)
  //emitter.off('fileRequest', request_file);
  //emitter.off('fileResponse', response_file);
})

</script>
    
<style lang="scss">
.pdf-paging
{
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  align-content: space-between;
}

.is-pdf
{
  min-width: 45vw;
}
.is-file
{
  min-width: 45vw;
}
.code-block
{
  font-size: 12px;
  min-width: 42.5vw;
  white-space: pre-wrap;
}


.loader
{
  width:100px;
  height:100px;
  display:flex;
  justify-content:center;
  align-items:center;
  position: absolute;
  top: 40%;
  left: 40%;
  z-index: 2000;
}
@keyframes load7
{
  100%{
    transform:rotatez(360deg);
  }
}

.loader7::before
{
  content:"";
  color:white;
  height:50px;
  width:50px;
  background:transparent;
  border-radius:50%;
  border:10px solid blue;
  border-color:#0277BD #0277BD #0277BD #81D4FA;
  animation:load7 .6s infinite ease-in-out;
  box-shadow:0px 0px 40px -2px skyblue;
}
.progressbar
{
  position:relative;
  top: 15px;
  min-width: 107%;
}
.paging
{
  max-width: 532px;
}

.header-t
{
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  align-content: space-between;
}
.bluring
{
  animation:  bluring-animate 0.3s;
  filter: blur(4px);
  -webkit-filter: blur(4px);
}
.unbluring
{
  animation:  unbluring-animate 0.35s;
  filter: blur(0px);
  -webkit-filter: blur(0px);
}

@keyframes bluring-animate
{
  0% 
  {
    filter: blur(1px);
    -webkit-filter: blur(1px);
  }
  30%
  {
    filter: blur(2px);
    -webkit-filter: blur(2px);
  }
  70% 
  {
    filter: blur(3px);
    -webkit-filter: blur(3px);
  }
  100% 
  {
    filter: blur(4px);
    -webkit-filter: blur(4px);
  }
}
@keyframes unbluring-animate
{
  0% 
  {
    filter: blur(3px);
    -webkit-filter: blur(3px);
  }
  30% 
  {
    filter: blur(2px);
    -webkit-filter: blur(2px);
  }
  70% 
  {
    filter: blur(1px);
    -webkit-filter: blur(1px);
  }
  100% 
  {
    filter: blur(0px);
    -webkit-filter: blur(0px);
  }
}
</style>
        