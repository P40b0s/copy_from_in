<template lang="pug">
n-drawer(v-model:show="is_open" style="min-width: 32vw;")
  n-drawer-content
    template(#header)
      div(style="display: flex; flex-direction: row; align-items: center; width: 100%;")
          n-tooltip(placement="left")  Выбор файла для просмотра 
            template(#trigger)
              drawler-file-selector(v-if="packet" :packet="packet" @onSelect="on_selected")
    div.pdf-paging(v-if="is_pdf")
      img(:src="current_image" @wheel="on_wheel" :class="{'bluring': in_progress, 'unbluring': !in_progress}")
    highlightjs.code-block(v-if="!is_pdf" :language='lang' :code="body" :class="[{'is-pdf': is_pdf}, {'is-file': !is_pdf}]")
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
import { NNotificationProvider, NLoadingBarProvider, NButton, NIcon, NSkeleton, NTooltip, NDrawer, NDrawerContent, NPagination, NSpin, NProgress, useLoadingBar} from 'naive-ui';
import { Analytics, Document, Warning, Settings, FingerPrintSharp, PieChart, PulseSharp } from '@vicons/ionicons5';
import { type Emitter, type Events } from "../../services/emit";
import { type Status } from 'naive-ui/es/progress/src/interface';
import DrawlerFileSelector from '../file_selector/drawler_file_selector';
import {type IPacket, type File, type FileRequest} from '../../models/types'
import { type SelectedValue } from '../file_selector/file_selector_label';
import Loader2 from '../Loader/Loader2.vue';
import { commands_packets } from '../../services/tauri/commands';
</script>


<script lang="ts" setup>
//loader2(v-if="in_progress" style="position: absolute; bottom: 30vh")
const emitter = inject<Emitter<Events>>('emitter') as Emitter<Events>;
const is_open = ref(false);
const in_progress = ref(false);
const body = ref<string>("");
const lang = ref("xml");
const packet = ref<IPacket>()
const file_path = ref("");
const file_type = ref<string>();
const file_name = ref("");
const is_pdf = computed((): boolean =>
{
  return file_type.value == 'pdf';
})
const current_image = ref<string>();
const current_page = ref(1);
const pages_count = ref(1);
let file_request: FileRequest;
let file: File;
watch(is_open, (o, n) =>
{
  if(n == false)
  {
    file_type.value = "";
  }
})
///selected path
const on_selected = async (s: SelectedValue) =>
{
    console.log(s)
    // switch(s.ext)
    // {
    //   case "xml":
    //   {
    //     //lang.value = "xml"
    //     break;
    //   }
    // }
    if (s.path != file_path.value)
    {
      file_path.value = s.path;
      file_type.value = s.ext;
      file_name.value = s.label
      file = {file_name: s.label, file_type: s.ext, path: s.path} as File;
      if(s.ext == "pdf")
      {
        in_progress.value = true;
        const request = {file} as FileRequest;
        const pages = await commands_packets.get_pdf_pages_count(request);
        if(pages.is_ok())
        {
          pages_count.value = pages.get_value()
          await change_page(1);
        }
        //todo запрос количества страниц с сервера
      }
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
    console.log(is_pdf.value, current_page.value)
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

emitter.on('packetItemDoubleClick', (p) => 
{
  packet.value = p;
  is_open.value = true;
});

onUnmounted(()=> 
{
  emitter.off('packetItemDoubleClick')
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
        