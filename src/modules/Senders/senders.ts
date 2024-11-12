import { Ref, ref } from "vue";
import { Senders } from "../../models/senders";
import { commands_packets } from "../../services/tauri/commands";
import { Callback, IPacket } from "../../models/types";
import { image_ico } from "../../services/svg";
import { events } from "../../services/tauri/events";


const _updated = events.sender_update(async s=> 
{
    const sender = s.payload;
    const index = senders.value.findIndex(s=>s.id == sender.id);
    if(index >= 0)
    {
        senders.value.splice(index, 1, sender);
    }
    else
    {
        const snd = await commands_packets.get_senders();
        senders.value = snd.value ?? [];
    }
});

const senders = ref<Senders[]>([]);
/**
 * Логика работы с отправителями, получение списка отправителей, получение иконки отправителя и подписка на изменения отправителя через таури события
 * @returns 
 */
export function useSenders()
{
    const get_senders = async () => 
    {
        if(senders.value.length == 0)
        {
            const snd = await commands_packets.get_senders();
            senders.value = snd.value ?? [];
        }
    }
    const get_icon = (packet: IPacket): string =>
    {
        const snd = senders.value.find(f=>f.id == packet.packetInfo?.senderId);
        return snd?.icon ?? image_ico
    }
    const get_organization = (packet: IPacket): string =>
    {
        const snd = senders.value.find(f=>f.id == packet.packetInfo?.senderId);
        return snd?.organization ?? ""
    }
    return {senders, get_senders, get_icon, get_organization}
}
