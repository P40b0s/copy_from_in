<template lang="pug">
n-modal(:show="props.is_open" 
        preset="card"
        @close="close_card"
        :style="{width: '800px'}")
    template(#header) Карточка отправителя
    n-table.contact-table(single-column)
        tbody
            tr
                td(rowspan="5" style="width: 1%; vertical-align: top; margin-top: 5px;")
                    image-uploader(v-model:icon="sender.icon")
            tr
                td
                    n-form-item.contacts-form(label="Организация" label-style="fontWeight: 700;" path="user.org" )
                        n-input(v-model:value="sender.organization" :autosize="{minRows: 1,maxRows: 2}"  type="textarea" placeholder="Введите организацию")
            tr
                td
                    n-form-item.contacts-form(label="Адрес МЭДО" label-style="fontWeight: 700;"  path="user.post" )
                        n-input(v-model:value="sender.medo_addresse" :autosize="{minRows: 1,maxRows: 2}"  type="textarea" placeholder="Введите адрес МЭДО")
    template(#action)
        div.actions
            n-button(type="success" @click="save_sender") Сохранить
            n-button(type="error" @click="close_card") Отмена
</template>
        
<script lang="ts">
import {type Emitter, type Events} from '../../services/emit';
import {  h, ref, inject, type VNodeChild, onUnmounted, watch } from 'vue';
import {  MailOpenOutline, } from '@vicons/ionicons5';
import {  NTable, NDynamicInput, NInput, NSelect, NModal, NFormItem,  NButton, NIcon, type UploadFileInfo, type SelectOption, type SelectGroupOption, NTooltip} from 'naive-ui';
import ImageUploader from './ImageUploader.vue';
import { type SelectBaseOption, type SelectIgnoredOption } from 'naive-ui/es/select/src/interface';
import { Senders } from '../../models/senders';
</script>

<script lang="ts" setup>
const props = defineProps<{
    is_open: boolean,
    sender: Senders,
}>();
const emits = defineEmits<{
    'update:is_open': [value: boolean]
    'update:sender': [value: Senders]
    'delete:sender': [value: Senders]
}>();
const emitter = inject<Emitter<Events>>('emitter') as Emitter<Events>;
const sender = ref<Senders>(new Senders().clone(props.sender));



const sender_icon = () =>
{
    return sender.value.icon as string|undefined;
}
console.log("почему то не появляется элемент....", props.is_open)
const close_card = () =>
{
    emits('update:is_open', false);
}
const update_show_modal = (value: boolean) =>
{
    if(value == false)
        emits('update:is_open', false);
}
const senders_updated_event = (s: Senders) =>
{
    emits('update:sender', s);
}

const start_edit_sender_event = (s: Senders) => 
{
    sender.value = s;
}
const delete_question = (s: Senders) : string =>
{
   return "Подтвердите удаление отправителя " + s.organization;
}

const accept_delete = async (s: Senders) =>
{
    emits('delete:sender', s);
    emits('update:is_open', false);
}
onUnmounted(()=> 
{
    
})


const save_sender = async () =>
{
    emits('update:sender', sender.value);
    emits('update:is_open', false);
}
const renderLabel = (option: SelectBaseOption, selected: boolean): VNodeChild => 
{
    return [
        h(
        NTooltip,
        null,
        {
            trigger:() =>
            [
                h(
                    
                    h('span',
                    [
                        h(
                            NIcon,
                            {
                                style: 
                                {
                                    verticalAlign: '-0.15em',
                                    marginRight: '4px'
                                }
                            },
                            {
                                default: () => h(MailOpenOutline)
                            },
                        ),
                        option.label as string
                    ]),
                ),
                //option.label as string
            ],
            default: () => option.value
        }
        ), 
    ]
};

const previewImageUrl = ref<string|undefined>(sender.value.icon as string|undefined);
//const selected_icon = ref(0);
//const action = ref<string|undefined>(undefined);
const beforeUpload  = async (data: {file: UploadFileInfo, fileList: UploadFileInfo[]}) =>
{
    console.log(data.file);
    previewImageUrl.value = data.file.thumbnailUrl as string;
    if (!data.file.file?.type || data.file.file?.type.indexOf('image') < 0) 
    {
        return false
    }
    
    const url = URL.createObjectURL(data.file.file);
    const blob = new Blob([new Uint8Array(await data.file.file.arrayBuffer())], {type: data.file.type as string|undefined });
    const b = await blobToBase64(blob);
    //console.log(b);
    previewImageUrl.value = b;
    sender.value.icon = b;
    //action.value = "http://sdasd";
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
const createThumbnailUrl =  (file: File | null): Promise<string> | undefined =>
{
    if (!file) return undefined
    previewImageUrl.value = file.name;
        
}
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
.actions
{
    display: flex;
    justify-content: space-between;
}
.contacts-header
{
    display: flex;
    justify-content: space-between;
}
.contacts-actions
{
    display: flex;
    flex-direction: row;
    font-size: large;
    justify-content: space-between;
    text-align: center !important;
}
.contacts-header
{
    display: flex;
    flex-direction: column;
    justify-content: space-between;
}
.contacts-form
{
    --n-feedback-height: 1px !important;
    font-weight: 700;
}

.contact-table
{
    --n-td-padding: 5px !important;
}
</style>
        