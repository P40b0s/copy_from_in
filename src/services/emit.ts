import {Emitter} from 'strict-event-emitter'
import { IPacket } from '../models/types';
const emitter = new Emitter<Events>()
export default emitter
export {type Events, type Emitter};


type Events =
{
    userUpdated: [],
    /**Происходит когда юзер апдейтит информацию о сендере, чтобы например у всех появилась
     *  новая иконка нужно уведомить об этом сервер, а тот пришлет этот эвент,
     *  значит нужно обновить список сендеров с сервера
     * */
    senderUpdated:[]
    /**
     * На этом эвенте происходи открытие просмотрщика файлов
     */
    openFileViewer: [IPacket]
};