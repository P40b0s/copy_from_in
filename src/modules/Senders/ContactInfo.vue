<template lang="pug">.info-td-name
n-drawer(:show="props.is_open" placement="left" style="min-width: 600px;")
  n-drawer-content
    template(#header) Контактная информация
    n-space(vertical v-if="sender.contact_info.length > 0")
      n-table.contact-table(v-for="contact in sender.contact_info")
        thead        
          tr
            th(colspan="2") 
              .contacts-actions.neon-blue {{ contact.person ? contact.person : "ФИО отсутствует" }}
                n-tooltip редактировать контакт
                  template(#trigger)
                    n-icon(size="30" @click="edit_contact(contact)" style="cursor: pointer;")
                      img(:src="EditIco" style="width: 30px;")
          tbody
            tr(v-if="contact.photo")
              td(rowspan="5" style="width: 2%;")
                n-avatar(:size="200" :src="contact.photo" object-fit="contain" style="background-color: #f0f8ff00;")
            tr(v-if="contact.organization" )
              td.contact-td-name {{ contact.organization }}
            tr(v-if="contact.post")
              td.contact-td-name {{ contact.post }}
            tr(v-if="contact.contacts.length > 0")
              td.contact-td-name
                div(v-for="c in contact.contacts") {{ c.contact_type + ": " + c.value }}
            tr(v-if="contact.note")
              td.contact-td-name {{ contact.note }}
</template>
        
<script lang="ts">
import { ref, defineAsyncComponent, inject, onUnmounted, computed } from 'vue';
import { NIcon, NAvatar, NTooltip, NDrawer, NDrawerContent, NSpace, NTable} from 'naive-ui';
import { type Emitter, type Events } from "../../services/emit";
import EditIco from '../assets/svg/edit2.svg'
import {Senders, type ContactInfo} from '../../models/senders'
</script>

<script lang="ts" setup>
const emitter = inject<Emitter<Events>>('emitter') as Emitter<Events>;
  const props = defineProps<{
    is_open: boolean,
    sender: Senders,
}>();
const emits = defineEmits<{
    'update:is_open': [value: boolean]
    'update:sender': [value: Senders]
    'delete:sender': [value: Senders]
}>();
const del_is_disabled = ref(props.sender.organization.length == 0);
const sender = computed(()=>
{
    return new Senders().clone(props.sender)
})

// const show_contact_info = (info : ContactInfoDb[]) =>
// {
//   contacts.value = info.sort((a,b) => (a.person ? a.person : -1) < (b.person ? b.person : -1) ? -1 : 1);
//   is_open.value = true;

// }

const edit_contact = (contact : ContactInfo) =>
{
  // const sender = packetService.senders.find(f=>f.contact_info.find(ff=>ff.id == contact.id))
  // if(sender)
  // {
  //   emitter.emit('startEditContacts', sender);
  // }
}


onUnmounted(()=>
{

})


</script>
    
<style lang="scss">

.contact-th-name
{
  //border: 1px solid rgb(126, 26, 117) !important;
  text-align: center !important;
  font-size: 20px !important;
}

.contact-td-name
{
  font-weight: 700;
  width: 100% !important;
  font-size: 16px;
}

.contact-table
{
    --n-td-padding: 1px !important;
}
</style>
        