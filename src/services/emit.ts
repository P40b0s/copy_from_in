import {Emitter} from 'strict-event-emitter'
import { IPacket } from '../models/types';
const emitter = new Emitter<Events>()
export default emitter
export {type Events, type Emitter};


type Events =
{
    userUpdated: [],
    packetItemDoubleClick: [IPacket]
};