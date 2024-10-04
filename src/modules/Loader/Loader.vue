<template lang="pug">
.main
    n-tooltip(trigger="hover") Подождите, идет загрузка информации...
        template(#trigger)
            .loader
</template>
        
<script lang="ts">
import { defineAsyncComponent } from 'vue';

</script>

<script lang="ts" setup>
import { NTooltip } from 'naive-ui';
</script>
    
<style lang="scss" scoped>
.loader 
{
    display: inline-flex;
    gap: 10px;
}
.loader:before,
.loader:after 
{
    content: "";
    height: 40px;
    aspect-ratio: 1;
    border-radius: 50%;
    background:
    linear-gradient(#222 0 0) top/100% 0% no-repeat,
    radial-gradient(farthest-side,#000 98%,#fd070700) 50%/14px 14px no-repeat
    #fff;
    animation: l9 4s infinite ease-in;
}
@keyframes l9 
{
    0%  {background-size:100% 0%,14px 14px;background-position:top, 50% 50%}
    80% {background-size:100% 70%,14px 14px;background-position:top, 50% 70%}
    84%,
    100%{background-size:100% 0%,18px 18px;background-position:top, 50% 50%}
}
.main
{
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100px;
    align-items: center;
    justify-content: center;
}
</style>