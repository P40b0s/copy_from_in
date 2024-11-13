<template lang="pug">
n-popconfirm(positive-text="Удалить"  @positive-click="emits('accept_delete')")
    template(#icon)
        n-icon(color="#d90d0d")
            TrashBin
    template(#trigger)
        n-tooltip 
            template(#trigger)
                n-button(quatenary circle color="#d90d0d" :disabled="props.disabled")
                    template(#icon)
                        TrashBin
            span {{ props.button_text }}
    slot(name="text")
</template>
        
<script lang="ts">
import {   NButton, NPopconfirm, NIcon, NTooltip} from 'naive-ui';
import { TrashBin } from '@vicons/ionicons5';
</script>

<script lang="ts" setup generic="T">

const props = defineProps<{
    disabled: boolean,
    button_text: string
}>();

defineSlots<{
    //text1: (scope: { value: string }) => any
    text: () => any
}>()

const emits = defineEmits<{
    'accept_delete': [...args: [] | [T]]
}>();


</script>
    
<style lang="scss">
</style>
        