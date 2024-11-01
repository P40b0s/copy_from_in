<template lang="pug">.info-td-name
n-drawer(v-model:show="is_open" placement="left" style="min-width: 600px;")
  n-drawer-content
    template(#header) Контактная информация
    n-space(vertical v-if="contacts.length > 0")
      n-table.contact-table(v-for="contact in contacts")
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
import { ref, defineAsyncComponent, inject, onUnmounted } from 'vue';
import { NIcon, NAvatar, NTooltip, NDrawer, NDrawerContent, NSpace, NTable} from 'naive-ui';
import { type Emitter, type Events } from "../services/emit";
import { packetService } from '../services';
import { ContactInfoDb } from '../models/backend/senders';
import EditIco from '../assets/svg/edit2.svg'

const ContactInfo = defineAsyncComponent({
  loader: () => import('./ContactInfo.vue'),
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
const contacts = ref<ContactInfoDb[]>([]);

const show_contact_info = (info : ContactInfoDb[]) =>
{
  contacts.value = info.sort((a,b) => (a.person ? a.person : -1) < (b.person ? b.person : -1) ? -1 : 1);
  is_open.value = true;

}
const edit_contact = (contact : ContactInfoDb) =>
{
  const sender = packetService.senders.find(f=>f.contact_info.find(ff=>ff.id == contact.id))
  console.log(sender);
  if(sender)
  {
    emitter.emit('startEditContacts', sender);
  }
}

emitter.on('viewContactInfo', show_contact_info);
onUnmounted(()=>
{
  emitter.off('viewContactInfo', show_contact_info)
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
        