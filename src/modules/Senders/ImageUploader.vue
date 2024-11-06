<template lang="pug">
n-upload(
    @before-upload="beforeUpload"
    :show-file-list="false"
    :action="action"
    :default-file-list="previewFileList"
    list-type="image"
    @preview="handlePreview")
    n-tooltip(v-if="icon") Нажмите для измениния изображения.
      template(#trigger)
        n-image(:src="icon" width="187" :fallback-src="image_ico"  preview-disabled style="cursor: pointer;")
    n-tooltip(v-else) Изображение отсутсвует, выможете загрузить изображение нажав на эту иконку.
      template(#trigger)
        n-image( :src="image_ico" :width="props.width ? props.width : 187" preview-disabled style="cursor: pointer;")

</template>
        
<script lang="ts">
import { ref, toRef, toRefs, watch } from 'vue';
import { NUpload, NTooltip, NImage, type UploadFileInfo} from 'naive-ui';
import { image_ico } from '../../services/svg';
</script>

<script lang="ts" setup>
const emit = defineEmits<{
  'update:icon': [val: string|undefined]
}>()
const props = defineProps<{
  icon?: string,
  width?: number
}>()

const icon  = ref(props.icon);
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
  const blob = new Blob([new Uint8Array(await data.file.file.arrayBuffer())], {type: data.file.type as string|undefined });
  const b = await blobToBase64(blob);
  previewImageUrl.value = b;
  icon.value = b;
  emit('update:icon', b);
  return true
}

const blobToBase64 = async (blob: Blob) : Promise<string> => 
{
  return new Promise((resolve, _) => 
  {
    const reader = new FileReader();
    reader.onloadend = () => resolve(reader.result as string);
    reader.readAsDataURL(blob);
  });
}

const previewFileList = ref<UploadFileInfo[]>([])
const handlePreview = (file: UploadFileInfo) =>
{
    const { url } = file
    previewImageUrl.value = url as string
}
</script>
    
<style lang="scss">

</style>
        