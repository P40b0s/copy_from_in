import mitt, {Emitter} from 'mitt'
import { IPacket } from '../models/types';
const emitter = mitt<Events>()
export default emitter
export {type Events, type Emitter};


type Events =
{
    userUpdated: void,
    packetItemDoubleClick: IPacket
};