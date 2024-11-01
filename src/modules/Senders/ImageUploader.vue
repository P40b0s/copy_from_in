<template lang="pug">
n-upload(
    @before-upload="beforeUpload"
    :show-file-list="false"
    :action="action"
    :default-file-list="previewFileList"
    list-type="image"
    @preview="handlePreview")
    n-tooltip(v-if="refIcon") Нажмите для измениния изображения.
      template(#trigger)
        n-image(:src="refIcon" width="187" :fallback-src="image_ico"  preview-disabled style="cursor: pointer;")
    n-tooltip(v-else) Изображение отсутсвует, выможете загрузить изображение нажав на эту иконку.
      template(#trigger)
        n-image( :src="image_ico" :width="props.width ? props.width : 187" preview-disabled style="cursor: pointer;")

</template>
        
<script lang="ts">
import { ref, toRefs, watch } from 'vue';
import { NUpload, NTooltip, NImage, type UploadFileInfo} from 'naive-ui';
import { image_ico } from '../../services/svg';
</script>

<script lang="ts" setup>
const emit = defineEmits<{
  (event: 'update:icon', val: string|undefined): void
}>()
const props = defineProps<{
  icon?: string,
  width?: number
}>()
const { icon } = toRefs(props);
//преременная icon реактивная, но она доступна только для чтения
//делаем копию чтоб vue не выдавал предупреждений на этот счет
const refIcon = ref<string|undefined>(icon?.value);
watch(props, (value) => 
{
  refIcon.value = value.icon
});
const previewImageUrl = ref<string|undefined>(icon?.value);
const action = ref<string|undefined>(undefined);
const beforeUpload  = async (data: {file: UploadFileInfo, fileList: UploadFileInfo[]}) =>
{
  //console.log(data.file);
  previewImageUrl.value = data.file.thumbnailUrl as string;
  if (!data.file.file?.type || data.file.file?.type.indexOf('image') < 0) 
  {
    return false
  }
  
  //const url = URL.createObjectURL(data.file.file);
  const blob = new Blob([new Uint8Array(await data.file.file.arrayBuffer())], {type: data.file.type as string|undefined });
  const b = await blobToBase64(blob);
  previewImageUrl.value = b;
  refIcon.value = b;
  emit('update:icon', b);
  return true
}
// watch(props, (value) => 
// {
//     console.log(value);
// });
const blobToBase64 = async (blob: Blob) : Promise<string> => 
{
  return new Promise((resolve, _) => 
  {
    const reader = new FileReader();
    reader.onloadend = () => resolve(reader.result as string);
    reader.readAsDataURL(blob);
  });
}
// const createThumbnailUrl =  (file: File | null): Promise<string> | undefined =>
// {
//     if (!file) return undefined
//     previewImageUrl.value = file.name;
        
// }
const previewFileList = ref<UploadFileInfo[]>([])
const handlePreview = (file: UploadFileInfo) =>
{
    const { url } = file
    previewImageUrl.value = url as string
    console.log(previewImageUrl.value);
    //showModal.value = true
}
</script>
    
<style lang="scss">

</style>
        