<template lang="pug">
n-drawer.file-drawler(v-model:show="is_open")
  n-drawer-content
    template(#header)
      div(style="width: inherit")
        drawler-file-selector(v-if="packet" :packet="packet" @onSelect="on_selected")
    transition(name="fade")
      .pdf-container(v-if="is_pdf" :class="{'bluring': in_progress, 'unbluring': !in_progress}")
        img(:src="current_image" @wheel="on_wheel")
    transition(name="fade")
      .image-container(v-if="is_image")
        img(:src="current_image")
    transition(name="fade")
      n-scrollbar.hlsb(v-if="is_file")
        highlightjs.code-block(:language='lang' :code="body")
    template(#footer)
      n-pagination.paging(
            v-if="is_pdf"
            v-model:page="current_page"
            :disabled="in_progress"
            :page-count="pages_count"
            :on-update-page="change_page"
            size="medium"
            show-quick-jumper)
</template>
 
<script lang="ts">
import { ref, type Component, defineAsyncComponent, watch, inject, onMounted, onUnmounted, computed } from 'vue';
import { NNotificationProvider, NScrollbar, NLoadingBarProvider, NButton, NIcon, NSkeleton, NTooltip, NDrawer, NDrawerContent, NPagination, NSpin, NProgress, useLoadingBar} from 'naive-ui';
import { Analytics, Document, Warning, Settings, FingerPrintSharp, PieChart, PulseSharp } from '@vicons/ionicons5';
import { type Emitter, type Events } from "../../services/emit";
import { type Status } from 'naive-ui/es/progress/src/interface';
import DrawlerFileSelector from './file_selector/drawler_file_selector';
import {type IPacket, type File, type FileRequest} from '../../models/types'
import { type FileType, FileTypeEnum, supported_files} from '../../models/file_types';
import {type SelectedValue } from './file_selector/file_selector_label';
import Loader2 from '../Loader/Loader2.vue';
import { commands_packets } from '../../services/tauri/commands';
</script>


<script lang="ts" setup>
//loader2(v-if="in_progress" style="position: absolute; bottom: 30vh")
const emitter = inject<Emitter<Events>>('emitter') as Emitter<Events>;
const is_open = ref(false);
const in_progress = ref(false);
const body = ref<string>("");
const lang = ref<string>();
const packet = ref<IPacket>()
const file_path = ref("");
let file_type = ref<FileType>();
const file_name = ref("");
const is_pdf = ref(false);
const is_file = ref(false);
const is_image = ref(false);
const current_image = ref<string>();
const current_page = ref(1);
const pages_count = ref(1);

let file_request: FileRequest;
/**параметры файла для запроса на бэк */
let file: File;
watch(is_open, (o, n) =>
{
  if(n == false)
  {
    file_type.value = undefined;
    current_image.value = undefined;
    is_open.value
  }
})

///selected path
const on_selected = async (s: SelectedValue) =>
{
    //if (s.path != file_path.value)
    {
      current_image.value = undefined;
      is_file.value = false;
      is_pdf.value = false;
      is_image.value = false;
      body.value = "";
      file_path.value = s.path;
      file_name.value = s.label
      file_type.value = supported_files.get_type(s.ext);
      file = {file_name: s.label, file_type: s.ext, path: s.path} as File;
      //только поддерживаемые расширения файлов
      if(file_type)
      {
        lang.value = file_type.value?.highlighting_lang;
        switch(file_type.value?.type)
        {
          case FileTypeEnum.Pdf:
          {
            is_pdf.value = true;
            in_progress.value = true;
            const request = {file} as FileRequest;
            const pages = await commands_packets.get_pdf_pages_count(request);
            if(pages.is_ok())
            {
              pages_count.value = pages.get_value()
              await change_page(1);
            }
            break;
          }
          case FileTypeEnum.File:
          {
            is_file.value = true;
            await process_file();
            break;
          }
          case FileTypeEnum.Image:
          {
            is_image.value = true;
            await process_image();
            break;
          }
          default:
          {
            console.error("Операция не поддерживается с типом файла " + file_type.value?.extension);
            break;
          }
        }
      }
    }
}
const process_file = async () =>
{
  const request = {file} as FileRequest;
  const res = await commands_packets.get_file_body(request);
  if(res.is_ok())
  {
    body.value = res.get_value()
  }
}
const process_image = async () =>
{
  const request = {file} as FileRequest;
  const res = await commands_packets.get_file_body(request);
  if(res.is_ok())
  {
    current_image.value = 'data:image/png;base64,' + res.get_value();
  }
}

const change_page = async (pagenum: number) =>
{
  in_progress.value = true;
  let request = {file, page_number: pagenum} as FileRequest;
  const png = await commands_packets.get_pdf_page(request);
  if (png.is_ok())
  {
    current_page.value = pagenum;
    current_image.value = 'data:image/png;base64,' + png.get_value();
  }
  in_progress.value = false;
}

const on_wheel = async (e: WheelEvent) =>
{
  if(!in_progress.value)
  {
    if (e.deltaY > 0)
    {
      const target_page = current_page.value + 1;
      if (target_page <= pages_count.value)
      {
        await change_page(target_page)
      }
    }
    else
    {
      const target_page = current_page.value - 1;
      if (target_page > 0)
      {
        await change_page(target_page)
      }
    }
    e.stopPropagation();
  }
}

const editor_open_event = (p: IPacket) =>
{
  packet.value = p;
  is_open.value = true;
}

emitter.on('openFileViewer', editor_open_event);
onUnmounted(()=> 
{
  emitter.off('openFileViewer', editor_open_event);
})
</script>
    
<style lang="scss">
.pdf-container
{
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  align-content: space-between;
  //overflow: hidden;
  
}
.image-container
{
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  align-content: space-between;
  max-width: 550px;
}

.pdf-container img
{
  width: inherit;
  height: inherit;
}

.image-container img
{
  width: inherit;
  height: inherit;
  max-width: inherit;
  background-color: aliceblue;
}
.file-drawler
{
  width: 650px;
  min-width: 650px;
  overflow-x: hidden;
  overflow-y: hidden;
  .n-base-selection .n-base-selection-label
  {
    height: initial !important;
  }
}
//TRANSITIONS
.fade-enter-active 
{
  transition: opacity 11s
}

.fade-enter,
.fade-leave-active 
{
  opacity: 0
}

.hlsb
{
  max-height: inherit;
  width: inherit;
}
.code-block
{
  font-size: 12px;
  width: inherit;
  height: inherit;
  background-color: transparent;
  white-space: pre-wrap;
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
  animation:  bluring-animate 0.2s;
  filter: blur(2.5px);
  -webkit-filter: blur(2.5px);
  //transform: translateY(120%);
  //-webkit-transform: translateY(120%);
}
.unbluring
{
  animation:  unbluring-animate 0.1s;
  filter: blur(0px);
  -webkit-filter: blur(0px);
  //transform: translateY(0%);
  //-webkit-transform: translateY(0%);
}

@keyframes bluring-animate
{
  0% 
  {
    filter: blur(0.5px);
    //transform: translateY(0%);
    //-webkit-transform: translateY(0%);
    -webkit-filter: blur(0.5px);
  }
  20%
  {
    filter: blur(1px);
    //transform: translateY(30%);
    //-webkit-transform: translateY(30%);
    -webkit-filter: blur(1px);
  }
  40%
  {
    filter: blur(1.5px);
    //transform: translateY(30%);
    //-webkit-transform: translateY(30%);
    -webkit-filter: blur(1.5px);
  }
  70% 
  {
    filter: blur(2px);
    //transform: translateY(70%);
    //-webkit-transform: translateY(70%);
    -webkit-filter: blur(2px);
  }
  100% 
  {
    filter: blur(2.5px);
    //transform: translateY(100%);
    //-webkit-transform: translateY(100%);
    -webkit-filter: blur(2.5px);
  }
}
@keyframes y-on-animate
{
  0% 
  {
    transform: translateY(0%);
    -webkit-transform: translateY(0%);
  }
  20%
  {
    transform: translateY(20%);
    -webkit-transform: translateY(20%);
  }
  40%
  {
    transform: translateY(40%);
    -webkit-transform: translateY(40%);
  }
  70% 
  {
    transform: translateY(70%);
    -webkit-transform: translateY(70%);
  }
  100% 
  {
    transform: translateY(100%);
    -webkit-transform: translateY(100%);
  }
}
@keyframes y-off-animate
{
  0% 
  {
    transform: translateY(-100%);
    -webkit-transform: translateY(-100%);
  }
  20%
  {
    transform: translateY(-80%);
    -webkit-transform: translateY(-80%);
  }
  40%
  {
    transform: translateY(-60%);
    -webkit-transform: translateY(-60%);
  }
  70% 
  {
    transform: translateY(-30%);
    -webkit-transform: translateY(-30%);
  }
  100% 
  {
    transform: translateY(0%);
    -webkit-transform: translateY(0%);
  }
}

@keyframes unbluring-animate
{
  0% 
  {
    filter: blur(2px);
    //transform: translateY(-100%);
    //-webkit-transform: translateY(-100%);
    -webkit-filter: blur(2px);
  }
  20% 
  {
    filter: blur(1.5px);
    //transform: translateY(-70%);
    //-webkit-transform: translateY(-70%);
    -webkit-filter: blur(1.5px);
  }
  40% 
  {
    filter: blur(1px);
    //transform: translateY(-70%);
    //-webkit-transform: translateY(-70%);
    -webkit-filter: blur(1px);
  }
  70% 
  {
    filter: blur(0.5px);
    //transform: translateY(-30%);
    //-webkit-transform: translateY(-30%);
    -webkit-filter: blur(0.5px);
  }
  100% 
  {
    filter: blur(0px);
    //transform: translateY(0%);
    //-webkit-transform: translateY(0%);
    -webkit-filter: blur(0px);
  }
}
</style>
        