<template lang="pug">
n-modal(v-model:show="show" 
    preset="card"
    :style="{width: '800px'}")
    template(#header)
        .contacts-header 
            span Контакты
            n-tooltip добавить контакт
                template(#trigger) 
                    n-icon(size="30" @click="add_contact" :component="PersonAdd" color="#33ff5f" style="cursor: pointer;")
    n-scrollbar(style="max-height: 75vh")
        n-space(vertical v-if="sender?.contact_info && sender?.contact_info.length > 0" style="margin-right: 9px;")
            n-table.contact-table(v-for="c in sender?.contact_info" single-column)
                thead        
                    tr
                        th(colspan="2") 
                            .contacts-actions.neon-blue {{ c.person ? c.person : "" }}
                                accept-query(tooltip="удалить контакт" :delete-query-text="delete_question(c)" :del-object="c", :accepted="accept_delete")
                tbody
                    tr
                        td(rowspan="6" style="width: 1%; vertical-align: top; margin-top: 5px;")
                            image-uploader(v-model:icon="c.photo")
                    tr
                        td
                            n-form-item.contacts-form(label="Организация" label-style="fontWeight: 700;" path="user.org" placeholder="Введите организацию")
                                n-input(v-model:value="c.organization" :autosize="{minRows: 1,maxRows: 2}"  type="textarea")
                    tr
                        td
                            n-form-item.contacts-form(label="Должность" label-style="fontWeight: 700;"  path="user.post" placeholder="Заполните должность")
                                n-input(v-model:value="c.post" :autosize="{minRows: 1,maxRows: 2}"  type="textarea")
                    tr
                        td
                            n-form-item.contacts-form(label="ФИО" label-style="fontWeight: 700;"  path="user.fio" placeholder="Заполните ФИО")
                                n-input(v-model:value="c.person" :autosize="{minRows: 1,maxRows: 2}"  type="textarea")
                    tr
                        td
                            n-form-item.contacts-form(label="Контактная информация" label-style="fontWeight: 700;"  path="user.fio" )
                                n-space(vertical style="width: 100%;")
                                    n-input-group(v-for="(tel, i) in c.contacts" style="gap:5px;")
                                        n-input(v-model:value="tel.contact_type" style="width: 35%;")
                                        n-input(v-model:value="tel.value")
                                        n-icon(size="20" @click="delete_phone(c, tel)" :component="TrashBinSharp" color="red" style="cursor: pointer; margin-left: 5px; align-self: center;")
                                    n-tooltip добавить контактную информацию
                                        template(#trigger) 
                                            n-icon(size="30" @click="add_phone(c)" :component="CallOutline" color="#33ff5f" style="cursor: pointer; margin-top: 5px")
                    tr
                        td
                            n-form-item.notes-form(label="Заметка"  label-style="fontWeight: 700;"  path="user.fio")
                                n-input(v-model:value="c.note" :autosize="{minRows: 1,maxRows: 2}"  type="textarea")
    template(#action)
        .actions
            n-button(type="success" @click="save") Сохранить
            n-button(type="error" @click="show = false") Отмена
</template>
        
<script lang="ts">
import { ref, inject } from 'vue';
import { CallOutline, PersonAdd, TrashBinSharp} from '@vicons/ionicons5';
import { NScrollbar, NInput, NTable, NInputGroup, NModal, NFormItem, NSpace, NButton, NIcon, NTooltip} from 'naive-ui';
import ImageUploader from './ImageUploader.vue';
import { onUnmounted } from 'vue';
import { Senders } from '../../models/senders';
</script>

<script lang="ts" setup>
import {type Emitter, type Events} from '../../services/emit';
const emitter = inject<Emitter<Events>>('emitter') as Emitter<Events>;
const show = ref(false);
const sender = ref<Senders>()
const start_edit_sender_event = (s: Senders) =>
{
    sender.value = deep_clone(s);
    show.value = true;
}
emitter.on('startEditContacts', start_edit_sender_event);
onUnmounted(()=> 
{
    emitter.off('startEditContacts', start_edit_sender_event);
})


const add_contact = async () =>
{
    let ci = new ContactInfoDb();
    sender.value?.contact_info.push(ci);
    //emit('update:contacts', contacts.value);
}
const add_phone = async (contact: ContactInfoDb) =>
{
    let ci = new ContactTypeDb();
    contact.contacts.push(ci);
    //emit('update:contacts', contacts.value);
}
const delete_contact = async (contact: ContactInfoDb) =>
{
    contact_for_del = contact;
}
let contact_for_del: ContactInfoDb|undefined = undefined;
const accept_delete_contact = async (e: MouseEvent) =>
{
    if(contact_for_del)
    {
        const index = sender.value?.contact_info.indexOf(contact_for_del) as number;
        sender.value?.contact_info.splice(index,1);
        //emit('update:contacts', contacts.value);
    }
}
const delete_phone = async (contact: ContactInfoDb, phone: ContactTypeDb) =>
{
    const index = contact.contacts.indexOf(phone);
    if(index > -1)
    {
        contact.contacts.splice(index, 1);
        //emit('update:contacts', contacts.value);
    }
}

const save = async () =>
{
    console.log(sender.value);
    if (await packetService.update_sender(sender.value as Senders))
        show.value = false;
}

const delete_question = (c: ContactInfoDb) : string =>
{
   return "Подтвердите удаление контакта " + (c.person ? c.person : "") as string;
}


const accept_delete = (s: ContactInfoDb) =>
{
    const index = sender.value?.contact_info.indexOf(s) as number;
    sender.value?.contact_info.splice(index,1);
    //emit('update:contacts', contacts.value);
}
//const showModal = ref(false)
</script>

<style lang="scss">
.contacts-actions
{
    display: flex;
    flex-direction: row;
    font-size: large;
    justify-content: space-between;
    text-align: center !important;
}
.actions
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
        