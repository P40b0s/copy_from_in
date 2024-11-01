<template lang="pug">
div.senders-div
    div.filters-bar
        live-search(v-model:value='live_search_value')
        n-button(@click="add_new_sender") добавить
            template(#icon)
                n-icon(:component="AddCircleSharp" color="green")
    
    //- contact-editor
    sender-editor(v-if="edited_sender != undefined" v-model:is_open="sender_editor_is_open", @update:sender="senders_updated_event" v-model:sender="edited_sender")
    n-data-table(
        :key="component_key"
        :columns="columns"
        :data="senders"
        :max-height="750"
        :row-key="rowKey"
        :row-class-name="rowClassName"
        @update:sorter="handleSorterChange"
        virtual-scroll)
</template>
        
<script lang="ts">
import { computed, h, reactive, type Ref, ref, inject, onMounted, watch, watchEffect, nextTick, onUnmounted } from 'vue';
import { Analytics, ThumbsUpOutline, TimeOutline, AddCircleSharp} from '@vicons/ionicons5';
import { NStatistic, NDatePicker, NInput, NSpace, NLayout, NLayoutSider, NMenu, NPopconfirm, NCol, NRow, NButton, NIcon, NGridItem, NGrid, NDataTable, NAlert, NTag, type DataTableCreateSummary, NAvatar, NGradientText, NImage} from 'naive-ui';
import { type RowData } from 'naive-ui/es/data-table/src/interface';
import {LiveSearch} from '../live_search.tsx'
import SenderEditor from './SenderEditor.vue';
//import ContactEditor from './ContactEditor.vue';
import { Senders } from '../../models/senders.ts';
import { commands_packets } from '../../services/tauri/commands.ts';
</script>

<script lang="ts" async setup>
import { organ_column } from './main_column';
import {type Emitter, type Events} from '../../services/emit';
const emitter = inject<Emitter<Events>>('emitter') as Emitter<Events>;
const snd = await commands_packets.get_senders();
const senders = ref<Senders[]>(snd.value ?? []);
const component_key = ref(0);
const sender_editor_is_open = ref(false);
const edited_sender = ref<Senders>();


const start_edit_sender = (s: Senders) =>
{
    edited_sender.value = s;
    sender_editor_is_open.value = true;
    console.log("Нало редактирования отправителей...", sender_editor_is_open.value);
}
const start_edit_contacts = (s: Senders) =>
{
    edited_sender.value = s;
    console.log("Нало редактирования контактов...")
}

const { organColumn } = organ_column(start_edit_sender, start_edit_contacts);
const sortStatesRef = ref([])
const senders_updated_event = (s: Senders) =>
{
    console.log(s);
    const index = senders.value.findIndex(s=>s.id == s.id);
    senders.value.splice(index, 1, s);
    component_key.value ++;
}

const senders_delete_event = (s: Senders) =>
{
    const index = senders.value.findIndex(s=>s.id == s.id);
    senders.value.splice(index, 1);
    component_key.value --;
}



// const sortKeyMapOrderRef = computed<PacketInfo>(() =>
//     sortStatesRef.value.reduce<PacketInfo>((result, { columnKey, order }) => 
//     {
//         result[columnKey] = order
//         return result
//     }, new PacketInfo())
// )

const rowKey = (rowData: any) =>
{
    const rd = rowData as Senders;
    return rd.id;
}
const add_new_sender = () =>
{
    //emitter.emit('startEditSender', new Senders());
}

const columns = 
ref([
    organColumn
]);
const live_search_value = ref("");
watch(live_search_value, (n, o)=>
{
   organColumn.filterOptionValue = n;
})


const handleSorterChange = (sorter: any) =>
{
    columns.value.forEach((column) => 
    {
        /** column.sortOrder !== undefined means it is uncontrolled */
        if (column.sortOrder === undefined)
         return
        console.log(sorter)
        if (!sorter) 
        {
            column.sortOrder = false
            return
        }
        if (column.key === sorter.columnKey) column.sortOrder = sorter.order
        else column.sortOrder = false
    })
}

const rowClassName =  (row: RowData) => 
{
    if (row.error) 
    {
        return 'error'
    }
        return ''
}

</script>
    
<style lang="scss">
.senders-div
{
    display: flex;
    flex-direction: column;
    width: 100%;
}
.filters-bar
{
  display: flex;
  margin-bottom: 3px;
  gap: 10px;
}
.filters-bar > *
{
  margin-top: 6px;
  margin-bottom: 4px;
}
</style>
        